/// Variable argument version of `syscall`
#[macro_export]
macro_rules! syscall {
    ($nr:ident) => {
        $crate::syscall1($crate::nr::$nr, 0)
    };
    ($nr:ident, $a1:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize,
                                           $a3 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize,
                                           $a3 as usize, $a4 as usize])
    };
}

/// Macro version of `syscall1`
#[macro_export]
macro_rules! syscall1 {
    ($nr:ident, $a1:expr) => {
        $crate::syscall1($crate::nr::$nr, $a1 as usize)
    };
}
