use std::{collections::VecDeque, env, fs, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1]).expect("Missing source file.");
    let mut cola = VecDeque::with_capacity(1024);
    let mut output = String::with_capacity(8192);
    let mut count = 0;

    output.push_str(PRELUDE);

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
                "jmp label{0}start\nlabel{0}end:\n",
                cola.pop_front().unwrap()
            )),

            _ => {}
        }
    }

    output.push_str(PROLOG);
    fs::write("tmp.asm", &output).expect("Error writing assembly file.");

    //assembler
    Command::new("nasm")
        .args(&["-f", "elf64", "tmp.asm", "-o", "tmp.o"])
        .status()
        .expect("Error while generating ELF file.");

    //linker
    Command::new("ld")
        .args(&[
            "-lc",
            "tmp.o",
            "-o",
            &args[1].trim_end_matches(".bf"),
            "-I",
            "/lib64/ld-linux-x86-64.so.2",
        ])
        .status()
        .expect("Error while linking.");

    //execute
    Command::new(&format!("./{}", &args[1].trim_end_matches(".bf")))
        .spawn()
        .expect("While trying to run generated program.");
}

const PROLOG: &str = "add rsp, 8320\nmov eax, 0\ncall exit";
const PRELUDE: &str = r"global _start
extern getchar
extern putchar
extern exit
section .text
_start:
sub rsp, 8192
mov eax, 0
mov ecx, 8192
mov rdi, rsp
rep stosb
mov r12, rsp
sub rsp, 128
";
