; A program to sort the numbers in descending order, using bubble sort
vals:DB 0xF                 ; declaration of the numbers
DB 0x5A
DB 0x24
DB 0x2
DB 0x56
last: DB 0                  ; declaring an element to get total number of elements later


; actual entry point of the program
start:
print mem :8                ; print initial state of memory
MOV CH, OFFSET last         ; move number of elemnts to CH
outer:                      ; loop label for outer loop
MOV CL, OFFSET last         ; move number of elemnts to CL
MOV SI, OFFSET vals         ; move offset of values to si

inner:                      ; loop label for inner loop
MOV AX, word [SI]           ; move two adjecent numbers to AX
CMP AL,AH                   ; compate both values
JNC skip                    ; jump to skip if first num is greater than second
XCHG AL, AH                 ; exchange both numbers
MOV word [SI], AX           ; move exchanged numbers to memory
skip:
INC SI                      ; increment SI
DEC CL                      ; decrement inner loop counter
JNZ inner                   ; jump back to inner if counter is not zero
DEC CH                      ; decrement outer loop counter
JNZ outer                   ; jump back to outer, if counter is not zero
print mem :8                ; print final state of memory