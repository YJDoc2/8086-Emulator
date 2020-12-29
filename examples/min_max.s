; A program to find minimum and maximum of given values
vals:                       ; values
DB 0x12
DB 0x34
DB 0x78
DB 0x13
DB 0x99
DB 0x65
DB 0x85
DB 0x11
DB 0x84
DB 0x36
last:DB 0                   ; declaring an element to get total number of elements later, and also to separate values from min and max
MIN: DB 0                   ; place to store minimum number
MAX: DB 0                   ; place to store maximum number

; actual entry point of the program
start:
MOV SI,0                    ; move starting point of values in si
MOV CX,OFFSET last          ; move number of values to cx
MOV AL, byte [SI]           ; move value to al
back:
CMP byte [SI],AL            ; compare value to al
JNC skip                    ; if not smaller, jump to skip
MOV AL,byte [SI]            ; move the value to si
skip:
INC SI                      ; increment counter
loop back                   ; loop back
mov byte MIN,AL             ; move minimum value to MIN
mov SI,0                    ; move starting point of values in si
mov CX, OFFSET last         ; move number of values to cx
mov AL, byte [SI]           ; move value to al
back1:
CMP AL,byte [SI]            ; compare al to value
JNC skip1                   ; if value is smaller jump to skip1
MOV AL,byte [SI]            ; move the value to al
skip1:
INC SI                      ; increment counter
loop back1                  ; loop back1
mov byte MAX,AL             ; move the maximum value to MAX
print mem :15               ; print the final state of memory
