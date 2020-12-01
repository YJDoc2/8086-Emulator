directives

data directives

set number : sets the data segment value to given number

DB num : sets a 8-bit value
DB \[num] : sets num bytes to 0
DB \[num ; val] : sets num bytes to value val

DW num : sets 16-bit value
DW \[num] : sets 2\*num bytes to 0
DW \[num ; val] : sets num bytes to 16-bit value val (So total 2\*num bytes are set)

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
