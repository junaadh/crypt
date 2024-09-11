.text
.global _start

_start:
	mov  r1, #1
	FUCK r1, r2

done:
	svc  #0xf0

.macro FUCK reg1 reg2
	Add  \reg1, \reg2, \reg1
.endm

.data
; duck: .word 4
