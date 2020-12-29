; A Program to move data from one segment to another
SET 0                   ; set address for segment 1
src:DB 0x3              ; store data
DB 0x5
DB 0x7

SET 0x1                 ; set addresss for segment 2
dest:DB [0,3]               ; store data

; actual entry point of the program
start:
print mem 0:8           ; print initial state of segment 1
print mem 0x10:8        ; print initial state of segment 2

MOV AX, 0               ; move address of seg1
MOV DS,AX               ; to ds
MOV AX , 0x1            ; move address of seg2
MOV ES,AX               ; to es
MOV SI, OFFSET src      ; move offset of source data
MOV SI, OFFSET dest     ; move offset of destination data
MOV CX, 0x3             ; move number of data items
print reg               ; print state of registers

_loop:
mov AH, byte DS[SI]     ; move one byte from source to ah
mov byte ES[DI],AH      ; move ah to destination
inc SI                  
inc DI                  
dec CX                  ; decrement count
jnz _loop               ; if count is not zero jump back
print mem 0:8           ; print final state of segment 1
print mem 0x10:8        ; print final state of segment 2
