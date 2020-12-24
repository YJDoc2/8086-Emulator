Data is stored in little endian format, lower byte first
directives
<br/>
data directives

set number : sets the data segment value to given number

num can be signed or unsigned, for decimal - sign is -ve, for hex and binary, use 2's complement negatives
<br/>
DB num : sets a 8-bit value<br/>
DB \[num] : sets num bytes to 0<br/>
DB \[val ; num] : sets num bytes to value val<br/>
DB OFFSET label<br/>
DB "ascii string" : keep string size less tha 65525 characters, otherwise label mapping will be incorrect<br/>

DW num : sets 16-bit value<br/>
DW \[num] : sets 2\*num bytes to 0<br/>
DW \[val ; num] : sets num bytes to 16-bit value val (So total 2\*num bytes are set)<br/>
DW OFFSET label<br/>
DW "ascii string" : keep string size less than 32758 characters, otherwise label mapping will be incorrect<br/>
<br/>
<br/>
code directives

MACROS :<br/>
definition : MACRO name (params) -> code <-<br/>
use : name (values)<br/>
macros must have params, for constant/static macro use \_ as param in both declaration and invocation<br/>
self referential (direct or indirect) macros are not allowed.
for passing macro name to macro for invocation, make sure to leave space between param name and brackets :<br/>
MACRO a(q)-> ADD AX,q <- MACRO b(k,q) -> k **this space** (q)<- b(a,5)<br/>
<br/>
offset :<br/>
use : offset name<br/>
only supported for data labels<br/>
<br/>
functions<br/>
definition : def name {code}<br/>
By default a ret is added at end, for safety<br/>
<br/>
<br/>
AND-type instructions :<br/>
and byte_reg,byte_reg<br/>
and word_reg,word_reg<br/>
and byte_reg,"byte" mem_addr<br/>
and word_reg,"word" mem_addr<br/>
and "byte" mem_addr , byte_reg<br/>
and "word" mem_addr , word_reg<br/>
and byte_reg,number<br/>
and word_reg,number<br/>
and "byte" mem_add, number<br/>
and "word" mem_add, number<br/>
<br/>
<br/>
shifts supports immediate number of shift, even though 8086 does not<br/>

labels with same names as opcode are not supported
<br/>
print syntax<br/>
print flags - print all flags<br/>
print reg - print all registers (general , segment, offset)<br/>
print mem start -> end - print memory starting from start, till end, both inclusive<br/>
print mem start:offset - print offset bytes of memory from start, start inclusive<br/>
print mem :offset - print offset bytes of memory starting from value in DS\*10H (similar to memory conversion done in 8086 )<br/>

i8086 manual : https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf

movsb/movsw are not supported, as movs does the same work, and due to syntax not ambiguity is there

REP supports movs, loads,stos<br/>
REP(E/Z/NE/NZ) support cmps scas<br/>

XLAT always takes base address of table from BX. the form in manual (XLAT source-table) allows source-table to documentation purpose label, indicating where the BX offset points, but XLAT always takes BX as offset, even if the label points to some other, hence that form is not supported here. https://www.felixcloutier.com/x86/xlat:xlatb

push and pop only supports word label / word operand

lds and les are not supported, as they are used to load/store a 32 bit pointer, which we don't support
<br/>
<br/>
Only int 3H,10H, and 21H are supported<br/>
Only byte string is supported to display and input<br/>
In 10H : value of AH allowed are : 0AH,13H  
0AH ignores BH & BL (page number and page attribute)
13H ignores AL (write mode), BH & BL (page number and attributes), DH (row to print the string on), supports DL (column to print string on)
<br/>
IN 21H : value of AH allowed are : 1H,2H,0AH<br/>
ALL OF THESE WILL BUFFER INPUT, depending on the interpreter driver<br/>
<br/>
<br/>
For CALL, unlike real 8086 no ip/cs is pushed on stack, instead it is maintained internally<br/>
Similarly for RET no pop is done<br/>
<br/>
Conditions for jumps in 8086 family manual seemed incorrect, so have used from : https://css.csail.mit.edu/6.858/2014/readings/i386/Jcc.htm
<br/>
Segment override is not supported in memory addressing
For memory addressing explanation check : https://www.ic.unicamp.br/~celio/mc404s2-03/addr_modes/intel_addr.html
<br/>
Check https://retrocomputing.stackexchange.com/questions/2927/did-the-intel-8086-8088-not-guarantee-the-value-of-sssp-immediately-after-reset
for default values of segment registers
