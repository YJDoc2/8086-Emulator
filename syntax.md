# Syntax for the 8086 emulator

This shows the syntax for the 8086 emulator.
The main reference for this was 8086 family user manual : i8086 manual : https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf
<br />
In some places the reference has also been taken from https://css.csail.mit.edu/6.858/2014/readings/i386/c17.htm, which is for 80386 instruction set, but as most of it is backward compatible, it applies to 8086.

The opcodes and directives are case independent, but labels are case sensitive.
In general this used little endian format. Lower byte first, then higher byte

#### Structure of Program

<strong>Note that all data directives must come before code directives and opcodes</strong>

```bash
Data directives

Code directives and opcodes

```

#### Commonly used terms :

<div>
<ul>
    <li><strong>Label</strong> : a single, no space containing word, immediately followed by a colon (:) when defining the label. Can contain _, 0-9, a-z, A-Z, but must not start with a number.
    </li>
    <li>
        <strong>byte</strong> : the actual word "byte"
    </li>
    <li>
        <strong>word</strong> : the actual word "word"
    </li>
    <li>
        <strong>number</strong> : A number can be specified in three formats :
        <ul>
        <li> Decimal : using 0-9.</li>
        <li> Binary : using 0 and 1, must start with 0b, eg : 5 = 0b0101 </li>
        <li> Hexadecimal : using 0-9,a-f, must start with 0x, eg : 5 = 0x5 </li>
        </ul>
        value can be set using offset data directive as well.
    </li>
    <li>
        <strong>unsigned byte number</strong> : number in range 0 -> 255
    </li>
    <li>
        <strong>signed byte number</strong> : number in range -128 -> 127, only decimal numbers with '-' can be used, for other format use 2's complement for negating.
    </li>
    <li>
        <strong>unsigned word number</strong> : numbers in range 0 -> 65535.
    </li>
    <li>
        <strong>signed word number</strong> : numbers in range -32768 -> 32767, only decimal numbers with '-' can be used, for other format use 2's complement for negating.
    </li>
    <li>
        <strong>Word Registers</strong> : AX,BX,CX,DX,BP,SP,SI,DI
    </li>
    <li>
        <strong>Byte Registers</strong> : AL,AH,BL,BH,CL,CH,DL,DH
    </li>
    <li>
        <strong>Segment Registers</strong> : ES,DS,SS,CS
    </li>
    <li>
        <strong>memory</strong> : 8086 allows four types of memory addressing :
        For all cases, when BP is used, the segment used is SS, for others, DS is used, unless segment register is provided as override, in which case that is sued as the segment.
        <ul>
        <li>(segment-register) [offset] : offset of the data from current DS or given segment register</li>
        <li>(segment-register) [bx/bp/si/di] : The offset of value is taken from the specified register.</li>
        <li>(segment-register)[bs/bp/si/di , signed word number] : The offset is taken from the registers, and the number is added to it.</li>
        <li>(segment-register)[bs/bp , si/di , (signed word number) ] : The offset is taken from the base registers, and offset in index registers as well as the number is added to it. The number offset is optional.</li>
        </ul>
    </li>
</ul>
</div>

<details>
  <summary>Data Directives</summary>
    <div>
    <h4>Data directives supported by emulator</h4>
        <ul>
            <li>
                <strong>set</strong> : set directive is used for setting the value of  ds when storing the data.
                <p><strong>syntax</strong> : set unsigned-word-number</p>
            </li>
            <li>
                <strong>DB</strong> : used to store a single byte
                <p><strong>syntax</strong> : label is optional
                    <ul>
                    <li>(label) DB signed/unsigned byte number : sets a single byte to given value</li>
                    <li>(label) DB [unsigned word number] : sets given number of bytes to 0 (can be used to declare empty array)</li>
                    <li>(label) DB [signed/unsigned byte number ; unsigned word number] : sets given number of bytes (second argument) to given value (first argument).</li>
                    <li>(label) DB "string" : stores a string , characters or not escaped, eg : \n will be stored as \ and n.</li>
                    </ul>
                </p>
            </li>
            <li>
                <strong>DW</strong> : used to store a word number
                <p><strong>syntax</strong> : label is optional
                    <ul>
                    <li>(label) DW signed/unsigned word number : sets a word to given value</li>
                    <li>(label) DW [unsigned word number] : sets given number of words to 0 (can be used to declare empty array)</li>
                    <li>(label) DW [signed/unsigned word number ; unsigned word number] : sets given number of words (second argument) to given value (first argument).</li>
                    <li>(label) DW "string" : stores a string , characters or not escaped, eg : \n will be stored as \ and n.</li>
                    </ul>
                </p>
            </li>
            <li>
                <strong>offset</strong> : used to get offset of value from the data segment it was defined in. <strong>Note</strong> that this only gives offset from the segment was defined in, so if DS was changed using set, it will contain offset from that value.
                <p><strong>syntax</strong> :
                    <ul>
                    <li>offset label_name : can be used in place of number, as this is determined at compile time.</li>
                    </ul>
                </p>
            </li>
        </ul>
    </div>
</details>
<details>
  <summary>Code Directives</summary>
  <div>
    <h4>Code directives supported by emulator</h4>
    <ul>
        <li>
            <strong>macro definition</strong> : used to define macros, which can be used to put code in place, where parameters are replaced by given values at compile time
            <p><strong>syntax</strong> : macro macro_name (comma separated parameter list) -> replace string <- </p> The code between '->' and '<-' will be placed in place of macro use, where the parameters will be replaced by the ones given in macro call.<br />
            <strong> Note </strong> that recursive macros direct/ indirect are not supported. For no parameter macro use single _ as parameter in definition as well as use.<br/>
            For passing macro name to macro for invocation, make sure to leave space between param name and brackets :<br/>MACRO a(q)-> ADD AX,q <- MACRO b(k,q) -> k **this space** (q)<- b(a,5)<br/>
        </li>
        <li>
            <strong>macro use</strong> : used to 'call' macro, the code defined in macro will be placed in place of this,with parameters replaced.
            <p><strong>syntax</strong> : macro_name (comma separated value list)</p> The code between '->' and '<-' will be placed in place of macro use, where the parameters will be replaced by the ones given in macro call.
        </li>
        <li>
            <strong>procedure definition</strong> : used to define procedure
            <p><strong>syntax</strong> : def procedure_name {opcodes/macro use} <br />procedure name has same format as label, except ':'.</p>
        </li>
    </ul>
  </div>
</details>

<details>
  <summary>Print Statements</summary>
  <div>
    <h4>This shows print commands syntax, which can be used in the code as well as in interactive user prompt.</h4>
    <ul>
        <li>print flags : This will print the value of various flags.</li>
        <li>print reg   : This will print the value of registers.</li>
        <li>print mem start -> end : This will print the value of memory, from start to end, both inclusive. the start and end are unsigned number, in range 0 ->1048575</li>
        <li>print mem start:offset : This will print the value of memory from start, to start+offset. Value of start and start+offset must lie in 0 -> 1048575</li>
        <li>print mem :offset : This will print the value of memory from start of current data segment till offset, both inclusive.</li>
    </ul>
  </div>
</details>
<br />
<h5>For opcodes, detail explanation of what they do is given in 8086 family manual, this explains only the syntax.</h5>

<details>
  <summary>Control Instructions</summary>
    <div>
        These are single opcode instructions.<br />
        STC,CLC,CMC,STD,CLD,STI,CLI,HLT,NOP are supported.<br />
        WAIT, ESC, and LOCK are not supported <br />
        <strong>Syntax</strong> : opcode
    </div>
</details>

<details>
  <summary>Control Transfer Instructions</summary>
    <div>
        <ul>
            <li>jump instructions :<br />jmp, ja,jnbe,jae,jnb,jb,jnae,jbe,jna,jc,je,jz,jg,jnle,jge,jnl,jl,jnge,jle,jng,jnc,jne,jnz,jno,jnp,jpo,jns,jo,jp,jpe,js,jcxz</li>
            <li>loop instructions : <br />loop,loope,loopz,loopne,loopnz</li>
            <strong>Syntax</strong> : opcode label
        </ul>
        <div>
            <strong>int</strong> : Following interrupts are supported
            <ul>
                <li>int 3 : Can be used for debugging, displays user prompt</li>
                <li>int 0x10 : value of AH allowed are : 0AH,13H <br />
                0AH ignores BH & BL (page number and page attribute)<br/>
                13H ignores AL (write mode), BH & BL (page number and attributes), DH (row to print the string on), supports DL (column to print string on)</li>
                <li>int 0x21 :  value of AH allowed are : 1H,2H,0AH</li>
            </ul>
        </div>
        <p>into and iret are not supported</p>
        <div>
            <ul>
                <li><strong>call</strong> : used for calling a procedure.<br/>
                    <p><strong>syntax</strong> : call proc_name</p>
                </li>
                <li><strong>ret</strong> : used for returning from a procedure.<br/>
                    <p><strong>syntax</strong> : call</p>
                </li>
            </ul>
        </div>        
    </div>
</details>

<details>
  <summary>Bit Manipulation Instructions</summary>
    <div>
        <ul>
            <li><strong>not</strong> : bitwise not <br />
                <div><strong>syntax</strong> : <br/>
                not byte register<br/>
                not word register<br/>
                not byte memory<br/>
                not word memory<br/>
                not byte label<br/>
                not word label<br/>
                </div>
            </li>
            <li><strong>binary logical</strong> : and,or,xor,test 
            <div><strong>syntax</strong> : <br/>
                opcode byte-register , byte-register<br/>
                opcode word-register , word-register<br/>
                opcode byte-register , byte memory<br/>
                opcode word-register , word memory<br/>
                opcode byte-register , byte label<br/>
                opcode word-register , word label<br/>
                opcode byte memory , byte-register<br/>
                opcode word memory , word-register<br/>
                opcode byte label , byte-register<br/>
                opcode word label , word-register<br/>
                opcode byte-register , unsigned byte number<br/>
                opcode word-register , unsigned word number<br/>
                opcode byte memory , unsigned byte number<br/>
                opcode word memory , unsigned word number<br/>
                opcode byte label , unsigned byte number<br/>
                opcode word label , unsigned word number<br/>
                </div>
            </li>
            <li><strong>shifts and rotates</strong> : sal,shl,sar,shr,rol,ror,rcl,rcl
            <div><strong>syntax</strong> : <br/>
                opcode byte-register , unsigned byte number<br/>
                opcode word-register , unsigned byte number<br/>
                opcode byte-register , cl<br/>
                opcode word-register , cl<br/>
                opcode byte memory , unsigned byte number<br />
                opcode word memory , unsigned byte number<br />
                opcode byte memory , cl<br />
                opcode word memory , cl<br />
                opcode byte label , unsigned byte number<br />
                opcode word label , unsigned byte number<br />
                opcode byte label , cl<br />
                opcode word label , cl<br />
                </div>
            </li>
        </ul> 
    </div>
</details>

<details>
  <summary>Arithmetic Instructions</summary>
    <div>
        <ul>
            <li><strong>No operands</strong> : aaa,aad,aam,aas,daa,das,cbw,cwd<br />
                <strong>syntax</strong> : opcode
            </li>
            <li><strong>Single operand</strong> : dec,inc,neg,mul,imul,div,idiv<br/>
            <div><strong>syntax</strong> : <br/>
                    opcode byte-register<br/>
                    opcode word-register<br/>
                    opcode byte memory<br/>
                    opcode word memory<br/>
                    opcode byte label<br/>
                    opcode word label<br/>
                </div>
            </li>
            <li><strong>Binary opcodes</strong> : add, adc, sub, sbb, cmp <br/>
            <div><strong>syntax</strong> : <br/>
                    opcode byte-register , byte-register<br/>
                    opcode word-register , word-register<br/>
                    opcode byte-register , byte memory<br/>
                    opcode word-register , word memory<br/>
                    opcode byte-register , byte label<br/>
                    opcode word-register , word label<br/>
                    opcode byte memory , byte-register<br/>
                    opcode word memory , word-register<br/>
                    opcode byte label , byte-register<br/>
                    opcode word label , word-register<br/>
                    opcode byte-register , unsigned/signed byte number<br/>
                    opcode word-register , unsigned/signed word number<br/>
                    opcode byte memory , unsigned/signed byte number<br/>
                    opcode word memory , unsigned/signed word number<br/>
                    opcode byte label , unsigned/signed byte number<br/>
                    opcode word label , unsigned/signed word number<br/>
                    </div>
            </li>
        </ul>        
    </div>
</details>

<details>
  <summary>String Instructions</summary>
    <div>Instructions are : movs, lods,stos,cmps,scas<br />
        movsb, movsw are not supported<br />
        <strong>Syntax</strong> : <br />
        opcode byte<br/>
        opcode word<br/>
        The word and byte specifies if the string is byte string or word string<br/>
        <strong>repeat instructions</strong> <br/>
        rep supports movs,lods,stos<br/>
        repe,repz,repne,repnz supports cmps, scas<br/>
    </div>
</details>

<details>
  <summary>Data Transfer Instructions</summary>
    <div>
        in,out,lds,les are not supported <br/>    
        <ul>
    <li><strong>No operands</strong> : lahf,sahf,pushf,popf,xlat<br />
        <strong>syntax</strong> : opcode
    </li>
    <li><strong>lea</strong> : <br />
    <strong>syntax</strong> : 
    lea word-register word memory<br />
    lea word-register word label<br />
    </li>
    <li><strong>push</strong> : supports only word length memory<br />
    <strong>syntax</strong> : 
    push word-register<br />
    push segment-register (cs register allowed)<br />
    push word memory<br />
    push word label<br />
    </li>
    <li><strong>pop</strong> : supports only word length memory<br />
    <strong>syntax</strong> : 
    pop word-register<br />
    push segment-register (cs register not allowed)<br />
    push word memory<br />
    push word label<br />
    </li>
    <li><strong>xchg</strong> :<br />
    <strong>syntax</strong> : 
    xchg byte-register , byte-register<br />
    xchg word-register ,  word-register<br />
    xchg byte memory , byte-register<br />
    xchg byte-register , byte memory<br />
    xchg word memory , word-register<br />
    xchg word-register , word memory<br />
    xchg byte label , byte-register<br />
    xchg byte-register , byte label<br />
    xchg word label , word-register<br />
    xchg word-register , word label<br />
    </li>
    <li><strong>mov</strong> :<br />
    <strong>syntax</strong> : 
    mov byte-register , byte-register<br />
    mov word-register , word-register<br />
    mov byte-register , byte memory<br />
    mov word-register , word memory<br />
    mov byte-register , byte label<br />
    mov word-register , word label<br />
    mov byte memory , byte-register<br />
    mov word memory , word-memory<br />
    mov byte label , byte-register<br />
    mov word label , word-register<br />
    mov byte-register , unsigned/signed byte number<br />
    mov word-register , unsigned/signed word number<br />
    mov byte memory , unsigned/signed byte number<br />
    mov word memory , unsigned/signed word number<br />
    mov byte label , unsigned/signed byte number<br />
    mov word label , unsigned/signed word number<br />
    mov segment-register , word-register<br />
    mov word-register , segment-register<br />
    mov segment-register , word memory<br />
    mov segment-register , word label<br />
    mov word memory , segment-register<br />
    mov word label , segment-register<br />
    </li>
</ul>
    </div>
</details>
