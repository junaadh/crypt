mov  r1, #1
mov  r2, #10
add  r8, r2, r1

svc  #0xe0

mov  r8, #0
svc  #0xf0
