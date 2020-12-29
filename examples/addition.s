; Program to add two word length numbers
OPR1: DW 0x6969         ; declare first number
OPR2: DW 0x0420         ; declare second number
RESULT: DW 0            ; declare place to store result

; actual entry point of the program
start:
MOV AX, word OPR1       ; move first number to AX
MOV BX, word OPR2       ; move second number to BX
CLC                     ; clear the carry flag
ADD AX, BX              ; add BX to AX
MOV DI, OFFSET RESULT   ; move offset of result to DI
MOV word [DI], AX       ; store result
print reg               ; preint result