/// Macro to create a mutable reference to a statically allocated value
///
/// This macro returns a value with type `Option<&'static mut $ty>`. `Some($expr)` will be returned
/// the first time the macro is executed; further calls will return `None`. To avoid `unwrap`ping a
/// `None` variant the caller must ensure that the macro is called from a function that's executed
/// at most once in the whole lifetime of the program.
///
/// # Note
///
/// This macro requires a `critical-section` implementation to be set. For most single-hart systems,
/// you can enable the `critical-section-single-hart` feature for this crate. For other systems, you
/// have to provide one from elsewhere, typically your chip's HAL crate.
///
/// # Example
///
/// ``` no_run
/// use riscv::singleton;
///
/// fn main() {
///     // OK if `main` is executed only once
///     let x: &'static mut bool = singleton!(: bool = false).unwrap();
///
///     let y = alias();
///     // BAD this second call to `alias` will definitively `panic!`
///     let y_alias = alias();
/// }
///
/// fn alias() -> &'static mut bool {
///     singleton!(: bool = false).unwrap()
/// }
/// ```
#[macro_export]
macro_rules! singleton {
    (: $ty:ty = $expr:expr) => {
        $crate::_export::critical_section::with(|_| {
            static mut VAR: Option<$ty> = None;

            #[allow(unsafe_code)]
            let used = unsafe { VAR.is_some() };
            if used {
                None
            } else {
                let expr = $expr;

                #[allow(unsafe_code)]
                unsafe {
                    VAR = Some(expr)
                }

                #[allow(unsafe_code)]
                unsafe {
                    VAR.as_mut()
                }
            }
        })
    };
}

/// Macro to create interfaces to PLIC contexts in PACs.
///
/// This macro expects 5 arguments:
///
/// - `PLIC`: name of the PLIC context interface structure to be created.
/// We recommend to leave `PLIC` for context 0 and `PLICx` for the remaining contexts.
///
/// - `BASE`: base address of the PLIC peripheral of the target.
///
/// - `CONTEXT`: context number assigned to the PLIC interface.
///
/// - `INTERRUPT`: enum type of the external interruptions of the target.
/// This type must implement the [`crate::peripheral::plic::InterruptNumber`] trait.
///
/// - `PRIORITY`: enum type of the priority levels supported by the target.
/// This type must implement the [`crate::peripheral::plic::PriorityNumber`] trait.
///
/// # Note
///
/// This macro requires the `plic` feature to be active.
#[cfg(feature = "plic")]
#[macro_export]
macro_rules! plic_context {
    ($PLIC:ident, $BASE:literal, $CONTEXT:literal, $INTERRUPT:ident, $PRIORITY:ident) => {
        /// Platform-Level Interrupt Controller (PLIC) context.
        #[repr(transparent)]
        pub struct $PLIC {
            context: $crate::peripheral::PLIC<$BASE, $CONTEXT>,
        }

        impl $PLIC {
            /// Creates a new PLIC context interface.
            pub const fn new() -> Self {
                Self {
                    context: $crate::peripheral::PLIC::new(),
                }
            }

            /// Enables machine external interrupts.
            #[inline(always)]
            pub fn enable() {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::enable();
            }

            /// Disables machine external interrupts.
            #[inline(always)]
            pub fn disable() {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::disable();
            }

            /// Returns the priority level associated to a given interrupt source.
            #[inline(always)]
            pub fn priority(source: $INTERRUPT) -> $PRIORITY {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::priority(source)
            }

            /// Getter method for the priority level associated to a given interrupt source.
            #[inline(always)]
            pub fn get_priority(&self, source: $INTERRUPT) -> $PRIORITY {
                Self::priority(source)
            }

            /// Sets the priority level of a given interrupt source.
            ///
            /// # Note
            ///
            /// Interrupt source priorities are shared among all the contexts of the PLIC.
            /// Thus, changing the priority of sources  may affect other PLIC contexts.
            ///
            /// # Safety
            ///
            /// Changing priority levels can break priority-based critical sections and compromise memory safety.
            #[inline(always)]
            pub unsafe fn set_priority(&mut self, source: $INTERRUPT, priority: $PRIORITY) {
                self.context.set_priority(source, priority);
            }

            /// Checks if an interrupt triggered by a given source is pending.
            #[inline(always)]
            pub fn is_interrupt_pending(source: $INTERRUPT) -> bool {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::is_interrupt_pending(source)
            }

            /// Checks if an interrupt source is enabled for the PLIC context.
            #[inline(always)]
            pub fn is_interrupt_enabled(source: $INTERRUPT) -> bool {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::is_interrupt_enabled(source)
            }

            /// Enables an interrupt source for the PLIC context.
            ///
            /// # Safety
            ///
            /// It performs non-atomic read-modify-write operations, which may lead to undefined behavior.
            /// Additionally, Enabling an interrupt source can break mask-based critical sections.
            #[inline(always)]
            pub unsafe fn enable_interrupt(&mut self, source: $INTERRUPT) {
                self.context.enable_interrupt(source);
            }

            /// Disables an interrupt source for the PLIC context.
            ///
            /// # Safety
            ///
            /// It performs non-atomic read-modify-write operations, which may lead to undefined behavior.
            #[inline(always)]
            pub unsafe fn disable_interrupt(&mut self, source: $INTERRUPT) {
                self.context.disable_interrupt(source);
            }

            /// Returns the priority threshold of the PLIC context.
            #[inline(always)]
            pub fn threshold() -> $PRIORITY {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::threshold()
            }

            /// Getter method for the priority threshold of the PLIC context.
            #[inline(always)]
            pub fn get_threshold(&self) -> $PRIORITY {
                Self::threshold()
            }

            /// Sets the priority threshold for for the PLIC context.
            ///
            /// # Safety
            ///
            /// Unmasking an interrupt source can break mask-based critical sections.
            #[inline(always)]
            pub unsafe fn set_threshold(&mut self, priority: $PRIORITY) {
                self.context.set_threshold(priority);
            }

            /// Claims the number of a pending interrupt for for the PLIC context.
            /// If no interrupt is pending for this context, it returns [`None`].
            #[inline(always)]
            pub fn claim() -> Option<$INTERRUPT> {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::claim()
            }

            /// Marks a pending interrupt as complete from for the PLIC context.
            #[inline(always)]
            pub fn complete(source: $INTERRUPT) {
                $crate::peripheral::PLIC::<$BASE, $CONTEXT>::complete(source);
            }

            /// Resets the PLIC peripherals.
            ///
            /// # Safety
            ///
            /// It performs non-atomic read-modify-write operations, which may lead to undefined behavior.
            #[inline(always)]
            pub unsafe fn reset(&mut self) {
                self.context.reset::<$INTERRUPT, $PRIORITY>();
            }
        }
    };
}
