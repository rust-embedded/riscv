#include "asm.h"

.section .text.__ebreak
.global __ebreak
__ebreak:
    ebreak
    ret

.section .text.__wfi
.global __wfi
__wfi:
    wfi
    ret

.section .text.__sfence_vma_all
.global __sfence_vma_all
__sfence_vma_all:
    sfence.vma
    ret

.section .text.__sfence_vma
.global __sfence_vma
__sfence_vma:
    sfence.vma a0, a1
    ret

// User Trap Setup
RW(0x000, ustatus)  // User status register
RW(0x004, uie)      // User interrupt-enable register
RW(0x005, utvec)    // User trap handler base address

// User Trap Handling
RW(0x040, uscratch) // Scratch register for user trap handlers
RW(0x041, uepc)     // User exception program counter
RW(0x042, ucause)   // User trap cause
RW(0x043, utval)    // User bad address or instruction
RW(0x044, uip)      // User interrupt pending

// User Floating-Point CSRs
RW(0x001, fflags)   // Floating-Point Accrued Exceptions
RW(0x002, frm)      // Floating-Point Dynamic Rounding Mode
RW(0x003, fcsr)     // Floating-Point Control and Status Register (frm + fflags)

// User Counter/Timers
RO(  0xC00, cycle)          // Cycle counter for RDCYCLE instruction
RO(  0xC01, time)           // Timer for RDTIME instruction
RO(  0xC02, instret)        // Instructions-retired counter for RDINSTRET instruction
RO(  0xC03, hpmcounter3)    // Performance-monitoring counter
RO(  0xC04, hpmcounter4)    // Performance-monitoring counter
RO(  0xC05, hpmcounter5)    // Performance-monitoring counter
RO(  0xC06, hpmcounter6)    // Performance-monitoring counter
RO(  0xC07, hpmcounter7)    // Performance-monitoring counter
RO(  0xC08, hpmcounter8)    // Performance-monitoring counter
RO(  0xC09, hpmcounter9)    // Performance-monitoring counter
RO(  0xC0A, hpmcounter10)   // Performance-monitoring counter
RO(  0xC0B, hpmcounter11)   // Performance-monitoring counter
RO(  0xC0C, hpmcounter12)   // Performance-monitoring counter
RO(  0xC0D, hpmcounter13)   // Performance-monitoring counter
RO(  0xC0E, hpmcounter14)   // Performance-monitoring counter
RO(  0xC0F, hpmcounter15)   // Performance-monitoring counter
RO(  0xC10, hpmcounter16)   // Performance-monitoring counter
RO(  0xC11, hpmcounter17)   // Performance-monitoring counter
RO(  0xC12, hpmcounter18)   // Performance-monitoring counter
RO(  0xC13, hpmcounter19)   // Performance-monitoring counter
RO(  0xC14, hpmcounter20)   // Performance-monitoring counter
RO(  0xC15, hpmcounter21)   // Performance-monitoring counter
RO(  0xC16, hpmcounter22)   // Performance-monitoring counter
RO(  0xC17, hpmcounter23)   // Performance-monitoring counter
RO(  0xC18, hpmcounter24)   // Performance-monitoring counter
RO(  0xC19, hpmcounter25)   // Performance-monitoring counter
RO(  0xC1A, hpmcounter26)   // Performance-monitoring counter
RO(  0xC1B, hpmcounter27)   // Performance-monitoring counter
RO(  0xC1C, hpmcounter28)   // Performance-monitoring counter
RO(  0xC1D, hpmcounter29)   // Performance-monitoring counter
RO(  0xC1E, hpmcounter30)   // Performance-monitoring counter
RO(  0xC1F, hpmcounter31)   // Performance-monitoring counter
RO32(0xC80, cycleh)         // Upper 32 bits of cycle, RV32I only
RO32(0xC81, timeh)          // Upper 32 bits of time, RV32I only
RO32(0xC82, instreth)       // Upper 32 bits of instret, RV32I only
RO32(0xC83, hpmcounter3h)   // Upper 32 bits of hpmcounter3, RV32I only
RO32(0xC84, hpmcounter4h)
RO32(0xC85, hpmcounter5h)
RO32(0xC86, hpmcounter6h)
RO32(0xC87, hpmcounter7h)
RO32(0xC88, hpmcounter8h)
RO32(0xC89, hpmcounter9h)
RO32(0xC8A, hpmcounter10h)
RO32(0xC8B, hpmcounter11h)
RO32(0xC8C, hpmcounter12h)
RO32(0xC8D, hpmcounter13h)
RO32(0xC8E, hpmcounter14h)
RO32(0xC8F, hpmcounter15h)
RO32(0xC90, hpmcounter16h)
RO32(0xC91, hpmcounter17h)
RO32(0xC92, hpmcounter18h)
RO32(0xC93, hpmcounter19h)
RO32(0xC94, hpmcounter20h)
RO32(0xC95, hpmcounter21h)
RO32(0xC96, hpmcounter22h)
RO32(0xC97, hpmcounter23h)
RO32(0xC98, hpmcounter24h)
RO32(0xC99, hpmcounter25h)
RO32(0xC9A, hpmcounter26h)
RO32(0xC9B, hpmcounter27h)
RO32(0xC9C, hpmcounter28h)
RO32(0xC9D, hpmcounter29h)
RO32(0xC9E, hpmcounter30h)
RO32(0xC9F, hpmcounter31h)

// Supervisor Trap Setup
RW(0x100, sstatus)      // Supervisor status register
RW(0x102, sedeleg)      // Supervisor exception delegation register
RW(0x103, sideleg)      // Supervisor interrupt delegation register
RW(0x104, sie)          // Supervisor interrupt-enable register
RW(0x105, stvec)        // Supervisor trap handler base address
RW(0x106, scounteren)   // Supervisor counter enable

// Supervisor Trap Handling
RW(0x140, sscratch)     // Scratch register for supervisor trap handlers
RW(0x141, sepc)         // Supervisor exception program counter
RW(0x142, scause)       // Supervisor trap cause
RW(0x143, stval)        // Supervisor bad address or instruction
RW(0x144, sip)          // Supervisor interrupt pending

// Supervisor Protection and Translation
RW(0x180, satp)         // Supervisor address translation and protection

// Machine Information Registers
RO(0xF11, mvendorid)    // Vendor ID
RO(0xF12, marchid)      // Architecture ID
RO(0xF13, mimpid)       // Implementation ID
RO(0xF14, mhartid)      // Hardware thread ID

// Machine Trap Setup
RW(0x300, mstatus)      // Machine status register
RW(0x301, misa)         // ISA and extensions
RW(0x302, medeleg)      // Machine exception delegation register
RW(0x303, mideleg)      // Machine interrupt delegation register
RW(0x304, mie)          // Machine interrupt-enable register
RW(0x305, mtvec)        // Machine trap handler base address
RW(0x306, mcounteren)   // Machine counter enable

// Machine Trap Handling
RW(0x340, mscratch)     // Scratch register for machine trap handlers
RW(0x341, mepc)         // Machine exception program counter
RW(0x342, mcause)       // Machine trap cause
RW(0x343, mtval)        // Machine bad address or instruction
RW(0x344, mip)          // Machine interrupt pending

// Machine Protection and Translation
RW(  0x3A0, pmpcfg0)    // Physical memory protection configuration
RW32(0x3A1, pmpcfg1)    // Physical memory protection configuration, RV32 only
RW(  0x3A2, pmpcfg2)    // Physical memory protection configuration
RW32(0x3A3, pmpcfg3)    // Physical memory protection configuration, RV32 only
RW(  0x3B0, pmpaddr0)   // Physical memory protection address register
RW(  0x3B1, pmpaddr1)   // Physical memory protection address register
RW(  0x3B2, pmpaddr2)   // Physical memory protection address register
RW(  0x3B3, pmpaddr3)   // Physical memory protection address register
RW(  0x3B4, pmpaddr4)   // Physical memory protection address register
RW(  0x3B5, pmpaddr5)   // Physical memory protection address register
RW(  0x3B6, pmpaddr6)   // Physical memory protection address register
RW(  0x3B7, pmpaddr7)   // Physical memory protection address register
RW(  0x3B8, pmpaddr8)   // Physical memory protection address register
RW(  0x3B9, pmpaddr9)   // Physical memory protection address register
RW(  0x3BA, pmpaddr10)  // Physical memory protection address register
RW(  0x3BB, pmpaddr11)  // Physical memory protection address register
RW(  0x3BC, pmpaddr12)  // Physical memory protection address register
RW(  0x3BD, pmpaddr13)  // Physical memory protection address register
RW(  0x3BE, pmpaddr14)  // Physical memory protection address register
RW(  0x3BF, pmpaddr15)  // Physical memory protection address register

// Machine Counter/Timers
RO(  0xB00, mcycle)          // Machine cycle counter
RO(  0xB02, minstret)        // Machine instructions-retired counter
RO(  0xB03, mhpmcounter3)    // Machine performance-monitoring counter
RO(  0xB04, mhpmcounter4)    // Machine performance-monitoring counter
RO(  0xB05, mhpmcounter5)    // Machine performance-monitoring counter
RO(  0xB06, mhpmcounter6)    // Machine performance-monitoring counter
RO(  0xB07, mhpmcounter7)    // Machine performance-monitoring counter
RO(  0xB08, mhpmcounter8)    // Machine performance-monitoring counter
RO(  0xB09, mhpmcounter9)    // Machine performance-monitoring counter
RO(  0xB0A, mhpmcounter10)   // Machine performance-monitoring counter
RO(  0xB0B, mhpmcounter11)   // Machine performance-monitoring counter
RO(  0xB0C, mhpmcounter12)   // Machine performance-monitoring counter
RO(  0xB0D, mhpmcounter13)   // Machine performance-monitoring counter
RO(  0xB0E, mhpmcounter14)   // Machine performance-monitoring counter
RO(  0xB0F, mhpmcounter15)   // Machine performance-monitoring counter
RO(  0xB10, mhpmcounter16)   // Machine performance-monitoring counter
RO(  0xB11, mhpmcounter17)   // Machine performance-monitoring counter
RO(  0xB12, mhpmcounter18)   // Machine performance-monitoring counter
RO(  0xB13, mhpmcounter19)   // Machine performance-monitoring counter
RO(  0xB14, mhpmcounter20)   // Machine performance-monitoring counter
RO(  0xB15, mhpmcounter21)   // Machine performance-monitoring counter
RO(  0xB16, mhpmcounter22)   // Machine performance-monitoring counter
RO(  0xB17, mhpmcounter23)   // Machine performance-monitoring counter
RO(  0xB18, mhpmcounter24)   // Machine performance-monitoring counter
RO(  0xB19, mhpmcounter25)   // Machine performance-monitoring counter
RO(  0xB1A, mhpmcounter26)   // Machine performance-monitoring counter
RO(  0xB1B, mhpmcounter27)   // Machine performance-monitoring counter
RO(  0xB1C, mhpmcounter28)   // Machine performance-monitoring counter
RO(  0xB1D, mhpmcounter29)   // Machine performance-monitoring counter
RO(  0xB1E, mhpmcounter30)   // Machine performance-monitoring counter
RO(  0xB1F, mhpmcounter31)   // Machine performance-monitoring counter
RO32(0xB80, mcycleh)         // Upper 32 bits of mcycle, RV32I only
RO32(0xB82, minstreth)       // Upper 32 bits of minstret, RV32I only
RO32(0xB83, mhpmcounter3h)   // Upper 32 bits of mhpmcounter3, RV32I only
RO32(0xB84, mhpmcounter4h)
RO32(0xB85, mhpmcounter5h)
RO32(0xB86, mhpmcounter6h)
RO32(0xB87, mhpmcounter7h)
RO32(0xB88, mhpmcounter8h)
RO32(0xB89, mhpmcounter9h)
RO32(0xB8A, mhpmcounter10h)
RO32(0xB8B, mhpmcounter11h)
RO32(0xB8C, mhpmcounter12h)
RO32(0xB8D, mhpmcounter13h)
RO32(0xB8E, mhpmcounter14h)
RO32(0xB8F, mhpmcounter15h)
RO32(0xB90, mhpmcounter16h)
RO32(0xB91, mhpmcounter17h)
RO32(0xB92, mhpmcounter18h)
RO32(0xB93, mhpmcounter19h)
RO32(0xB94, mhpmcounter20h)
RO32(0xB95, mhpmcounter21h)
RO32(0xB96, mhpmcounter22h)
RO32(0xB97, mhpmcounter23h)
RO32(0xB98, mhpmcounter24h)
RO32(0xB99, mhpmcounter25h)
RO32(0xB9A, mhpmcounter26h)
RO32(0xB9B, mhpmcounter27h)
RO32(0xB9C, mhpmcounter28h)
RO32(0xB9D, mhpmcounter29h)
RO32(0xB9E, mhpmcounter30h)
RO32(0xB9F, mhpmcounter31h)

RW(0x323, mhpmevent3)   // Machine performance-monitoring event selector
RW(0x324, mhpmevent4)   // Machine performance-monitoring event selector
RW(0x325, mhpmevent5)   // Machine performance-monitoring event selector
RW(0x326, mhpmevent6)   // Machine performance-monitoring event selector
RW(0x327, mhpmevent7)   // Machine performance-monitoring event selector
RW(0x328, mhpmevent8)   // Machine performance-monitoring event selector
RW(0x329, mhpmevent9)   // Machine performance-monitoring event selector
RW(0x32A, mhpmevent10)  // Machine performance-monitoring event selector
RW(0x32B, mhpmevent11)  // Machine performance-monitoring event selector
RW(0x32C, mhpmevent12)  // Machine performance-monitoring event selector
RW(0x32D, mhpmevent13)  // Machine performance-monitoring event selector
RW(0x32E, mhpmevent14)  // Machine performance-monitoring event selector
RW(0x32F, mhpmevent15)  // Machine performance-monitoring event selector
RW(0x330, mhpmevent16)  // Machine performance-monitoring event selector
RW(0x331, mhpmevent17)  // Machine performance-monitoring event selector
RW(0x332, mhpmevent18)  // Machine performance-monitoring event selector
RW(0x333, mhpmevent19)  // Machine performance-monitoring event selector
RW(0x334, mhpmevent20)  // Machine performance-monitoring event selector
RW(0x335, mhpmevent21)  // Machine performance-monitoring event selector
RW(0x336, mhpmevent22)  // Machine performance-monitoring event selector
RW(0x337, mhpmevent23)  // Machine performance-monitoring event selector
RW(0x338, mhpmevent24)  // Machine performance-monitoring event selector
RW(0x339, mhpmevent25)  // Machine performance-monitoring event selector
RW(0x33A, mhpmevent26)  // Machine performance-monitoring event selector
RW(0x33B, mhpmevent27)  // Machine performance-monitoring event selector
RW(0x33C, mhpmevent28)  // Machine performance-monitoring event selector
RW(0x33D, mhpmevent29)  // Machine performance-monitoring event selector
RW(0x33E, mhpmevent30)  // Machine performance-monitoring event selector
RW(0x33F, mhpmevent31)  // Machine performance-monitoring event selector

// Debug/Trace Registers (shared with Debug Mode)
RW(0x7A0, tselect)  // Debug/Trace trigger register select
RW(0x7A1, tdata1)   // First Debug/Trace trigger data register
RW(0x7A2, tdata2)   // Second Debug/Trace trigger data register
RW(0x7A3, tdata3)   // Third Debug/Trace trigger data register

// Debug Mode Registers
RW(0x7B0, dcsr)     // Debug control and status register
RW(0x7B1, dpc)      // Debug PC
RW(0x7B2, dscratch) // Debug scratch register
