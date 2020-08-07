use std::{collections::VecDeque, env, fs, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let source = fs::read_to_string(&args[1]).expect("Missing source file.");
    let mut iter = source.chars().peekable();

    let mut output = String::with_capacity(8192);
    let mut cola = VecDeque::with_capacity(1024);
    let mut label_count = 0;
    let mut consecutive = 1;

    output.push_str(PROLOG);

    while let Some(chars) = iter.next() {
        match chars {
            '<' => {
                while iter.peek() == Some(&'<') {
                    consecutive += 1;
                    iter.next();
                }

                output.push_str(&format!(" sub r12, {}\n", consecutive));
            }

            '>' => {
                while iter.peek() == Some(&'>') {
                    consecutive += 1;
                    iter.next();
                }

                output.push_str(&format!(" add r12, {}\n", consecutive));
            }

            '+' => {
                while iter.peek() == Some(&'+') {
                    consecutive += 1;
                    iter.next();
                }

                output.push_str(&format!(" add byte [r12], {}\n", consecutive));
            }

            '-' => {
                while iter.peek() == Some(&'-') {
                    consecutive += 1;
                    iter.next();
                }

                output.push_str(&format!(" sub byte [r12], {}\n", consecutive));
            }

            ',' => output.push_str(" call getchar;\n mov [r12], al\n"),

            '.' => output.push_str(" mov dil, [r12];\n call putchar\n"),

            '[' => {
                output.push_str(&format!(
                    "label{0}start:\n cmp byte [r12], 0\n jz label{0}end\n",
                    label_count
                ));

                cola.push_back(label_count);
                label_count += 1;
            }

            ']' => output.push_str(&format!(
                " jmp label{0}start\nlabel{0}end:\n",
                cola.pop_front().expect("No matching loop")
            )),

            _ => {}
        }
        consecutive = 1;
    }

    output.push_str(EPILOG);
    fs::write("tmp.asm", &output).expect("Error writing assembly file.");

    //assembler
    Command::new("nasm")
        .args(&["-f", "elf64", "tmp.asm", "-o", "tmp.o", "-gdwarf"])
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

const EPILOG: &str = " add rsp, 8320\n mov eax, 0\n call exit";
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
