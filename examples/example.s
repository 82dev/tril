	.text
	.file	"example"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movabsq	$184683562313, %rax             # imm = 0x2AFFFF8549
	movq	%rax, 12(%rsp)
	movl	$73, 20(%rsp)
	leaq	12(%rsp), %rsi
	shrq	$32, %rsi
	movl	$.Lstr, %edi
                                        # kill: def $esi killed $esi killed $rsi
	callq	printf@PLT
	xorl	%eax, %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	.Lstr,@object                   # @str
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lstr:
	.asciz	"%d "
	.size	.Lstr, 4

	.section	".note.GNU-stack","",@progbits
