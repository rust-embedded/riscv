	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_c2p0"
	.file	"3wdx7lxc78dgym21"

	.section	.init,"ax",@progbits
	.globl	_start

_start:

	lui	ra, %hi(_abs_start)
	jalr	zero, %lo(_abs_start)(ra)

_abs_start:
	.cfi_startproc
	.cfi_undefined ra
	csrwi	mie, 0
	csrwi	mip, 0
	li	ra, 0
	li	sp, 0
	li	gp, 0
	li	tp, 0
	li	t0, 0
	li	t1, 0
	li	t2, 0
	li	s0, 0
	li	s1, 0
	li	a3, 0
	li	a4, 0
	li	a5, 0
	li	a6, 0
	li	a7, 0
	li	s2, 0
	li	s3, 0
	li	s4, 0
	li	s5, 0
	li	s6, 0
	li	s7, 0
	li	s8, 0
	li	s9, 0
	li	s10, 0
	li	s11, 0
	li	t3, 0
	li	t4, 0
	li	t5, 0
	li	t6, 0

	.option	push

	.option	norelax

.Lpcrel_hi0:
	auipc	gp, %pcrel_hi(__global_pointer$)
	addi	gp, gp, %pcrel_lo(.Lpcrel_hi0)
	.option	pop

	csrr	t2, mhartid
	lui	t0, %hi(_max_hart_id)
	addi	t0, t0, %lo(_max_hart_id)
	bltu	t0, t2, abort

.Lpcrel_hi1:
	auipc	sp, %pcrel_hi(_stack_start)
	addi	sp, sp, %pcrel_lo(.Lpcrel_hi1)
	lui	t0, %hi(_hart_stack_size)
	addi	t0, t0, %lo(_hart_stack_size)
	mul	t0, t2, t0
	sub	sp, sp, t0
	mv	s0, sp
	j	_start_rust

	.cfi_endproc
	.section	.trap,"ax",@progbits
	.globl	default_start_trap
default_start_trap:
	addi	sp, sp, -64
	sw	ra, 0(sp)
	sw	t0, 4(sp)
	sw	t1, 8(sp)
	sw	t2, 12(sp)
	sw	t3, 16(sp)
	sw	t4, 20(sp)
	sw	t5, 24(sp)
	sw	t6, 28(sp)
	sw	a0, 32(sp)
	sw	a1, 36(sp)
	sw	a2, 40(sp)
	sw	a3, 44(sp)
	sw	a4, 48(sp)
	sw	a5, 52(sp)
	sw	a6, 56(sp)
	sw	a7, 60(sp)
	mv	a0, sp
	jal	_start_trap_rust
	lw	ra, 0(sp)
	lw	t0, 4(sp)
	lw	t1, 8(sp)
	lw	t2, 12(sp)
	lw	t3, 16(sp)
	lw	t4, 20(sp)
	lw	t5, 24(sp)
	lw	t6, 28(sp)
	lw	a0, 32(sp)
	lw	a1, 36(sp)
	lw	a2, 40(sp)
	lw	a3, 44(sp)
	lw	a4, 48(sp)
	lw	a5, 52(sp)
	lw	a6, 56(sp)
	lw	a7, 60(sp)
	addi	sp, sp, 64
	mret	
	.section	.text.abort,"ax",@progbits
	.globl	abort
abort:
	j	abort
