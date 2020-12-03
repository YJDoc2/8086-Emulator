directives

data directives

set number : sets the data segment value to given number

num can be signed or unsigned, for decimal - sign is -ve, for hex and binary, use 1's complement negatives

DB num : sets a 8-bit value
DB \[num] : sets num bytes to 0
DB \[val ; num] : sets num bytes to value val
DB OFFSET label

DW num : sets 16-bit value
DW \[num] : sets 2\*num bytes to 0
DW \[val ; num] : sets num bytes to 16-bit value val (So total 2\*num bytes are set)
DW OFFSET label

code directives

MACROS : error reporting is still a bit wonky
definition : MACRO name (params) -> code <-
use : name (values)
macros must have params, for constant/static macro use \_ as param in both declaration and invocation

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
