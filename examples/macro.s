; Program to calculate factorial of a number using a macro
; not perticulary good use of macro, but still a use of macro :_)
NUM: DW 0x6     ; calculate factorial of 6
RESULT: DW 0    ; place to store the result

; Macro for factorial
MACRO fact(no) ->  MUL word no <-

; actual entry point of the program
start: 
MOV AX, 0x0001      ; initialize accumulator with 1
NOTZEROLOOP:        ; place to jump back
fact(NUM)           ; this will be expanded to MUL word NUM

DEC word NUM        ; decrement number
JNZ NOTZEROLOOP     ; jump back if number is not zero yet 
MOV word RESULT,AX; ; mov result from accumulator to memory
print mem :16       ; print memory

