	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_a2p0_c2p0"
	.file	"1ruuache5c4x1fns"

	.section	.init,"ax",@progbits
	.globl	_start

_start:

	.option	push

	.option	norelax
.Ltmp0:
	auipc	ra, %pcrel_hi(.Ltmp1)
	ld	ra, %pcrel_lo(.Ltmp0)(ra)
	ret
	.p2align	3
.Ltmp1:
	.quad	_abs_start
	.option	pop


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
	addi	sp, sp, -128
	sd	ra, 0(sp)
	sd	t0, 8(sp)
	sd	t1, 16(sp)
	sd	t2, 24(sp)
	sd	t3, 32(sp)
	sd	t4, 40(sp)
	sd	t5, 48(sp)
	sd	t6, 56(sp)
	sd	a0, 64(sp)
	sd	a1, 72(sp)
	sd	a2, 80(sp)
	sd	a3, 88(sp)
	sd	a4, 96(sp)
	sd	a5, 104(sp)
	sd	a6, 112(sp)
	sd	a7, 120(sp)
	mv	a0, sp
	jal	_start_trap_rust
	ld	ra, 0(sp)
	ld	t0, 8(sp)
	ld	t1, 16(sp)
	ld	t2, 24(sp)
	ld	t3, 32(sp)
	ld	t4, 40(sp)
	ld	t5, 48(sp)
	ld	t6, 56(sp)
	ld	a0, 64(sp)
	ld	a1, 72(sp)
	ld	a2, 80(sp)
	ld	a3, 88(sp)
	ld	a4, 96(sp)
	ld	a5, 104(sp)
	ld	a6, 112(sp)
	ld	a7, 120(sp)
	addi	sp, sp, 128
	mret	
	.section	.text.abort,"ax",@progbits
	.globl	abort
abort:
	j	abort
