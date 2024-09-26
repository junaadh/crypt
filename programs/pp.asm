.macro FUCK \reg1 \reg2
 Add  \reg1, \reg2, \reg1 
 Sub  \reg2, \reg1, \reg1
.endm

.macro FUCK2 \reg1 \reg2
 Add  \reg1, \reg2, \reg1 
 Sub  \reg2, \reg1, \reg1
.endm

; .section text
.global _start

_start:
	mov  r1, #1 ; damn bro
	FUCK r1 r2
	b    done

done:
	svc  #0xf0


; .data
; duck: .word 4
