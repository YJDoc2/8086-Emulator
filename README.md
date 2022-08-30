# 8086 Emulator

<img src="./cmdline.gif" alt="Execution GIF" />

This is an Intel 8086 emulator / vm. It can run most of 8086 instruction set and provides an interactive interpreter to run the program line by line.
This repository contains the core library which contains the preprocessor, data parser and interpreter ; as well as a command line driver which provides command line interface for running the program.
For syntax check <a href="./syntax.md">syntax.md</a>.

This also has be compiled to WASM and available in Web version : [https://github.com/YJDoc2/8086-emulator-web](https://github.com/YJDoc2/8086-emulator-web)

## Note

This is a Intel 8086 Emulator, providing a way to run programs written for 8086 assembly instruction set. This internally stores data in the emulated "memory" of 1 MB size, but the code is not compiled to binary or stored in memory. Assembly statements are executed using an interpreter, which operates on the memory and architecture (registers, flags etc.) to emulate execution of the program.

As this does not have a 'True' memory, this does not allow jumps to memory positions, and Does not support ISRs, as ISR requires the code to be stored in memory as well.

This also does not emulate external devices like storage, or co-processors, but allows almost all instructions that 8086 support.

Most of the assembly syntax is same as Intel assembly syntax, with few minor changes, which are documented under respective instructions in the <a href="./syntax.md">syntax.md</a>.

<ul>
  <li><a href ="#installation">Installation</a></li>
  <li><a href ="#to-use-as-dependency">Use As dependency</a></li>
  <li><a href ="#commandline-usage">Commandline Usage</a></li>
  <li><a href ="#core-library">Core Library</a>
    <ul>
      <li><a href ="#file-structure">Genral File Structure</a></li>
      <li><a href ="#driver">Commandline Driver</a></li>
      <li><a href ="#interpreting-flow">Interpreting Flow</a></li>
    </ul>
  </li>
</ul>

## Installation

As this is not on crates.io, installing is done using this repository itself.

```sh
cargo install --git https://github.com/YJDoc2/8086-Emulator.git
```

This should install the binary and put the program with name 'emulator_8086' in a folder in PATH, so you should be able to directly run.

## To Use As Dependency

As this is not on crates.io, dependency is specified using this repository itself.

```TOML
[dependencies]
emulator_8086 = { git = "https://github.com/YJDoc2/8086-Emulator" }
```

This will allow to import and use the core library of this in your project :

```Rust
use emulator_8086_lib;
```

Or if you don't want to use the long name, you can either rename the import :

```Rust
use emulator_8086_lib as lib;
```

or use it under different name :

```TOML
...
[dependencies]
a_different_name = { git = "https://github.com/YJDoc2/8086-Emulator", package="emulator_8086"}
...
```

## Commandline usage

```shell
USAGE:
emulator_8086 [FLAGS] [file_path]

FLAGS:
-h, --help Prints help information
-i, --interpreted To run in interpreted mode
-V, --version Prints version information

ARGS:
<file_path> Input assembly file path
```

- `file_path` is required argument for the assembly file which is to be run.
- The interpreted flag (-i) will enable a user prompt before execution of every instruction , which currently allows to print flags, registers and memory, and can be used to debug.
- <strong>Note :</strong> if you don't want to check after every command, but just before/after a particular command, use `int 3` instead, syntax explained in <a href="./syntax.md">syntax.md</a>.

The user prompt support following commands :

- n/next : this will continue the execution of instructions.
- q/quit : this will exit the program
- print statements : these allow to print flags, registers, and memory locations, the syntax is same as the assembly file print, explained in <a href="./syntax.md">syntax.md</a>.

Another way to get user prompt is to set trap flag, in which case, the prompt will be displayed before execution of each instruction as long as the trap flag is set.

## Core Library

#### File structure

The complete project has following file structure :

```
.
├── examples              ->  examples of 8086 assembly programs
├── src                   ->  the code
    ├── driver            ->  code for the commandline driver
    ├── lib               ->  code of core library
    |   ├── data_parser   ->  code and test for the data parser, which
    |                         interprets data commands and fill data in memory
    |   ├── instructions  -> contains functions which are used to run some opcode instructions,
    |                         which are not directly coded in the interpreter
    |   ├── interpreter   -> code and tests for the interpreter
    |   ├── preprocessor  -> code and test for the preprocessor
    |   ├── util          -> utility and helper functions / structures
    |   ├── arch.rs       -> definition of 8086 architecture struct
    |   ├── vm.rs         -> definition of vm struct
    |   └── lib.rs        -> lib file which re-exports various structs and function
    └── bin.rs            -> main file for the binary
├── build.rs              -> the build code required to generate parsers from lalrpop files
├── Cargo.toml            -> Cargo TOML file
├── README.md             -> This file
├── LICENSE-APACHE        -> Licence file
├── LICENCE-MIT           -> Licence file
├── syntax.md             -> file containing syntax for the assembler
├── flowcharts.md         -> Markdown file containing flowcharts for various parts of emulator
└── .gitignore            -> gitignore file for the repository
```

#### Driver

Driver is the program which uses the core library to run the vm and interpret the code, hence the name '_driver_'.
AS the core library contains functionality related to preprocessing/syntax checking, data feed into vm, and running the code via interpreter, it is kept purposefully as much platform independent as it can be. The driver takes care of taking the input (via a file or so), removing comments from the file, feeding that to the prerpocessor (which converts everything to uniform small case, and does syntac checking etc.), take its output, then run data parser to store the data of DB/DW commands into the vm memory, and then run the interpreter, as well as provide the user prompt.
The driver is kept this way, so that the core library can be also used without any changes in the [web version](https://github.com/YJDoc2/8086-Emulator-Web) as well.
<br />
The print parser in the driver interprets the print commands and displays the output. This can be used to interactively check state of vm, as well as for debugging purposes.

#### Interpreting Flow

The flow of the complete process is :

- Drivers takes the input of program, removes comments from the program, and feeds this to preproecssor
- Preprocessor checks the program for correct syntax, separates the code and data instructions, maps labels and functions to appropriate output instructions, and returns :

  - A list containing code instructions, and another list containing data instructions, also makes it uniform byt converting to small case, and converts from supported number formats like hexadecimal and binary to decimal
  - Maps of output code list elements to input code positions, code labels to the output code elements, stores the offset of data elements

- The driver checks for errors, start labels, and if all labels that are used before declaration (in jump commands) are declared or not. Then it gives data list of preprocessor output to data parser, which interprets data commands and stores the data into vm's memory
- The driver checks for errors and runs an unconditional loop, in which it gives the code instructions to interpreter, checks its return state, and accordingly gives next input. If it is run in interpreted mode / encounters int 3 or trap flag is set, it also provides user prompt and interprets and runs print commands. Driver also has responsibility to handle DOS and BIOS interrupts.

Flowcharts for various parts of emulator are in the [flowcharts.md](./flowcharts.md) file.

---

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
