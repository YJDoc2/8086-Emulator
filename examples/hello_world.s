; Program to show use of interrupts
; Also, Hello World program !
hello: DB "Hello World" ; store string

; actual entry point of the program, must be present
start:
MOV AH, 0x13            ; move BIOS interrupt number in AH
MOV CX, 11              ; move length of string in cx
MOV BX, 0               ; mov 0 to bx, so we can move it to es
MOV ES, BX              ; move segment start of string to es, 0
MOV BP, OFFSET hello    ; move start offset of string in bp
MOV DL, 0               ; start writing from col 0
int 0x10                ; BIOS interrupt
