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

REG_READ_WRITE(fcsr, 0x003)
REG_SET_CLEAR(fcsr, 0x003)

// M-mode registers
REG_READ(mcause, 0x342)
REG_READ(mcycle, 0xB00)
REG_READ(mepc, 0x341)
REG_READ(mie, 0x304)
REG_SET_CLEAR(mie, 0x304)
REG_READ(minstret, 0xB02)
REG_READ(mip, 0x344)
REG_READ(misa, 0x301)
REG_READ(mstatus, 0x300)
REG_SET_CLEAR(mstatus, 0x300)
REG_READ_WRITE(mtvec, 0x305)
REG_READ(mvendorid, 0xF11)

// S-mode registers
REG_READ_WRITE(satp, 0x180)
REG_READ(scause, 0x142)
REG_READ_WRITE(sepc, 0x141)
REG_READ(sie, 0x104)
REG_SET_CLEAR(sie, 0x104)
REG_READ(sip, 0x144)
REG_READ_WRITE(sscratch, 0x140)
REG_READ(sstatus, 0x100)
REG_SET_CLEAR(sstatus, 0x100)
REG_READ(stval, 0x143)
REG_READ_WRITE(stvec, 0x105)

REG_READ(time, 0xC01)
