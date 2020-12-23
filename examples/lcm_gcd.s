; Program to calculate LCM and GCD of two numbers

no1: dw 0x6     ; number 1
no2: dw 0x5     ; number 2
gcd: dw 0       ; place to store gcd
lcm: dw 0       ; place to store lcm

; actual entry point of the program
start:
mov ax, word no1    ; move number 1 in accumulatore
mov bx, word no2    ; move number 2 in register BX
loop0: mov dx, 0x0  ; place to loop back 
                    ; cannot use 'loop' as label, as loop is an opcode which will give error when used with jumps
div bx              ; divide accumulator by bx
mov ax, bx          
mov bx, dx          
cmp bx, 0x0         ; check if bx is 0
jnz loop0           ; if not loop back
mov word gcd, ax    ; store gcd
mov cx, ax          ; move ax in cx
mov ax, word no1    ; move number 1 in accumulatore 
mov bx, word no2    ; move number 2 in register BX
mul bx              ; multiply accumulator by BX
div cx              ; divide accumulator by CX
mov word lcm, ax    ; store lcm
print mem :16       ; print memory 
