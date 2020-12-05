directives

data directives

set number : sets the data segment value to given number

num can be signed or unsigned, for decimal - sign is -ve, for hex and binary, use 1's complement negatives

DB num : sets a 8-bit value
DB \[num] : sets num bytes to 0
DB \[val ; num] : sets num bytes to value val
DB OFFSET label
DB "ascii string" : keep string size less tha 65525 characters, otherwise label mapping will be incorrect

DW num : sets 16-bit value
DW \[num] : sets 2\*num bytes to 0
DW \[val ; num] : sets num bytes to 16-bit value val (So total 2\*num bytes are set)
DW OFFSET label
DW "ascii string" : keep string size less than 32758 characters, otherwise label mapping will be incorrect

code directives

MACROS :
definition : MACRO name (params) -> code <-
use : name (values)
macros must have params, for constant/static macro use \_ as param in both declaration and invocation
self referential (direct or indirect) macros are not allowed.
for passing macro name to macro for invocation, make sure to leave space between param name and brackets :
MACRO a(q)-> ADD AX,q <- MACRO b(k,q) -> k **this space** (q)<- b(a,5)

offset :
use : offset name
only supported for data labels

functions
definition : def name {code}
By default a ret is added at end, for safety

AND-type instructions : Verify once again
and byte_reg,byte_reg
and word_reg,word_reg
and byte_reg,"byte" mem_addr
and word_reg,"word" mem_addr
and "byte" mem_addr , byte_reg
and "word" mem_addr , word_reg
and byte_reg,number
and word_reg,number
and "byte" mem_add, number
and "word" mem_add, number

shifts supports immediate number of shift, even though 8086 does not

labels with same names as opcode are not supported

print syntax
print flags - print all flags
print reg - print all registers (general , segment, offset)
print mem start -> end - print memory starting from start, till end, both inclusive
print mem start:offset - print offset bytes of memory from start, start inclusive
print mem :offset - print offset bytes of memory starting from value in DS\*10H (similar to memory conversion done in 8086 )

i8086 manual : https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf

movsb/movsw are not supported, as movs does the same work, and due to syntax not ambiguity is there

REP supports movs, loads,stos
REP(E/Z/NE/NZ) support cmps scas

XLAT always takes base address of table from BX. the form in manual (XLAT source-table) allows source-table to documentation purpose label, indicating where the BX offset points, but XLAT always takes BX as offset, even if the label points to some other, hence that form is not supported here. https://www.felixcloutier.com/x86/xlat:xlatb

push and pop only supports word label

lds and les are not supported, as they are used to load/store a 32 bit pointer, which we don't support

Only int 3H,10H, and 21H are supported
In 10H : value of AH allowed are : 0AH,13H
IN 21H : value of AH allowed are : 1H,2H,0AH
ALL OF THESE WILL BUFFER INPUT, depending on the interpreter driver
