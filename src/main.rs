use std::{collections::VecDeque, fs, process::Command};

fn main() {
    let source = fs::read_to_string("./hello.bf").expect("No se pudo leer archivo");
    let mut cola = VecDeque::with_capacity(256);
    let mut output = String::with_capacity(8192);
    let mut count = 0;

    output.push_str(
        r"global _start
extern getchar
extern putchar
extern exit
section .text
_start:
sub rsp, 4000
mov eax, 0
mov ecx, 4000
mov rdi, rsp
rep stosb
mov r12, rsp
sub rsp, 64
",
    );

    for c in source.chars() {
        match c {
            '<' => output.push_str("sub r12, 1\n"),

            '>' => output.push_str("add r12, 1\n"),

            '+' => output.push_str("add byte [r12], 1\n"),

            '-' => output.push_str("sub byte [r12], 1\n"),

            ',' => output.push_str("call getchar;\nmov [r12], al\n"),

            '.' => output.push_str("mov dil, [r12];\ncall putchar\n"),

            '[' => {
                output.push_str(&format!(
                    "label{0}start:\ncmp byte [r12], 0\njz label{0}end\n",
                    count
                ));

                cola.push_back(count);
                count += 1;
            }

            ']' => output.push_str(&format!(
                " jmp label{0}start\nlabel{0}end:\n",
                cola.pop_front().unwrap()
            )),

            _ => {}
        }
    }

    output.push_str("add rsp, 4064\nmov eax,0\ncall exit");

    fs::write("tmp.asm", &output).expect("No se pudo escribir asm.");

    //assembler
    Command::new("nasm")
        .arg("-felf64")
        .arg("tmp.asm")
        .arg("-o")
        .arg("tmp.o")
        .status()
        .expect("assembler");

    //linker
    Command::new("ld")
        .arg("-lc")
        .arg("tmp.o")
        .arg("-o")
        .arg("hello")
        .arg("-I")
        .arg("/lib64/ld-linux-x86-64.so.2")
        .status()
        .expect("linker");

    //execute
    Command::new("./hello").spawn().expect("execute");
}
