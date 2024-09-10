        mov  r8, #0

loop:
        cmp  r8, #10
        b.eq done
        add  r8, r8, #1
        svc  #0xe0
        b    loop

done:
        svc #0xf0
