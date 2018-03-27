# `riscv`

> Low level access to RISCV processors

## Implemented Peripherals
- [ ] plic
- [ ] clint

## Implemented privileged ASM instructions
- [x] ecall
- [x] ebreak
- [x] uret
- [x] sret
- [x] mret
- [x] wfi
- [ ] sfence.vma

## Implemented CSR's

### User mode
- [ ] ustatus
- [ ] uie
- [ ] utvec
- [ ] uscratch
- [ ] uepc
- [ ] ucause
- [ ] utval
- [ ] uip
- [ ] fflags
- [ ] frm
- [ ] fcsr
- [ ] cycle
- [ ] time
- [ ] instret
- [ ] hpmcounter[3-31]
- [ ] cycleh
- [ ] timeh
- [ ] instreth
- [ ] hpmcounter[3-31]h

### Supervisor mode
- [ ] sstatus
- [ ] sedeleg
- [ ] sideleg
- [ ] sie
- [ ] stvec
- [ ] scounteren
- [ ] sscratch
- [ ] sepc
- [ ] scause
- [ ] stval
- [ ] sip
- [ ] satp

### Machine mode
- [x] mvendorid
- [ ] marchid
- [ ] mimpid
- [ ] mhartid
- [x] mstatus
- [x] misa
- [ ] medeleg
- [ ] mideleg
- [x] mie
- [x] mtvec
- [ ] mcounteren
- [ ] mscratch
- [ ] mepc
- [x] mcause
- [ ] mtval
- [x] mip
- [ ] pmpcfg[0-3]
- [ ] pmpaddr[0-15]
- [x] mcycle
- [x] minstret
- [ ] mhpmcounter[3-31]
- [x] mcycleh
- [x] minstreth
- [ ] mhpmcounter[3-31]h
- [ ] mhpmevent[3-31]
- [ ] tselect
- [ ] tdata[1-3]

# License
Copyright 2017 David Craven

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
