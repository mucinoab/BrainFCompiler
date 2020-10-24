# BrainFCompiler
A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) toy compiler that *very poorly* generates, assembles and links x86 instructions.


## Requirements

- OS ~Linux
- Assembler [NASM](https://en.wikipedia.org/wiki/Netwide_Assembler)
- Linker [ld](https://www.gnu.org/software/binutils/)
- [Rust](https://www.rust-lang.org/)


## Usage

You just clone the repo and run, ```cargo run --release _sourcefile.bf_```


#### Related
My other brainfuck compiler. [mucinoab/BrainFCompiler-LLVM](https://github.com/mucinoab/BrainFCompiler-LLVM)
