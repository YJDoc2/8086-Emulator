; Program to calculate factorial using looping
NUM: DW 0x6     ; calculate factorial of 6 
RESULT: DW 0    ; place to store the reult

; actual entry point of the program
start:
MOV CX,word NUM     ; move number into cx
MOV AX, 0x1         ; initialize accumulator with 1
NOTZEROLOOP:        ; label to jump back to
MUL CX              ; multiple by the number
DEC CX              ; decrement the number
JNZ NOTZEROLOOP     ; if not zero jump back
MOV word RESULT,AX  ; store the result in memory
print reg           ; print registers
