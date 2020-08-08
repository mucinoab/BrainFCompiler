use std::{collections::VecDeque, env, fs, process::Command};

fn main() {
    let source_name = env::args().skip(1).next().expect("Missing source file.");
    let source_file = fs::read_to_string(&source_name).expect("Source file not found.");
    let mut iter = source_file.chars().peekable();

    let mut cola = VecDeque::with_capacity(1024);
    let mut label_count = 0;
    let mut consecutive = 1;

    let mut output_asm = String::with_capacity(8192);
    output_asm.push_str(PROLOG);

    while let Some(chars) = iter.next() {
        match chars {
            '<' => {
                while iter.peek() == Some(&'<') {
                    consecutive += 1;
                    iter.next();
                }

                output_asm.push_str(&format!(" sub r12, {}\n", consecutive));
            }

            '>' => {
                while iter.peek() == Some(&'>') {
                    consecutive += 1;
                    iter.next();
                }

                output_asm.push_str(&format!(" add r12, {}\n", consecutive));
            }

            '+' => {
                while iter.peek() == Some(&'+') {
                    consecutive += 1;
                    iter.next();
                }

                output_asm.push_str(&format!(" add byte [r12], {}\n", consecutive));
            }

            '-' => {
                while iter.peek() == Some(&'-') {
                    consecutive += 1;
                    iter.next();
                }

                output_asm.push_str(&format!(" sub byte [r12], {}\n", consecutive));
            }

            ',' => output_asm.push_str(" call getchar;\n mov [r12], al\n"),

            '.' => output_asm.push_str(" mov dil, [r12];\n call putchar\n"),

            '[' => {
                output_asm.push_str(&format!(
                    "label{0}start:\n cmp byte [r12], 0\n jz label{0}end\n",
                    label_count
                ));

                cola.push_back(label_count);
                label_count += 1;
            }

            ']' => output_asm.push_str(&format!(
                " jmp label{0}start\nlabel{0}end:\n",
                cola.pop_front().expect("No matching loop")
            )),

            _ => {}
        }
        consecutive = 1;
    }

    output_asm.push_str(EPILOG);
    fs::write("tmp.asm", &output_asm).expect("Writing assembly file.");

    //assembler
    Command::new("nasm")
        .args(&["-f", "elf64", "tmp.asm", "-o", "tmp.o", "-gdwarf"])
        .status()
        .expect("Generating ELF file.");

    //linker
    Command::new("ld")
        .args(&[
            "-lc",
            "tmp.o",
            "-o",
            &source_name.trim_end_matches(".bf"),
            "-I",
            "/lib64/ld-linux-x86-64.so.2",
        ])
        .status()
        .expect("Linking.");

    //execute
    Command::new(&format!("./{}", &source_name.trim_end_matches(".bf")))
        .spawn()
        .expect("Running generated program.");
}

const PROLOG: &str = r" global _start
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

const EPILOG: &str = " add rsp, 8320\n mov eax, 0\n call exit";
