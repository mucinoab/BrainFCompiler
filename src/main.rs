use std::{collections::VecDeque, env, fmt::Write, fs, process::Command};

fn main() {
    let source_name = env::args().nth(1).expect("Missing source file.");
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

                writeln!(&mut output_asm, " sub r12, {}", consecutive).unwrap();
            }

            '>' => {
                while iter.peek() == Some(&'>') {
                    consecutive += 1;
                    iter.next();
                }

                writeln!(&mut output_asm, " add r12, {}", consecutive).unwrap();
            }

            '+' => {
                while iter.peek() == Some(&'+') {
                    consecutive += 1;
                    iter.next();
                }

                writeln!(&mut output_asm, " add byte [r12], {}", consecutive).unwrap();
            }

            '-' => {
                while iter.peek() == Some(&'-') {
                    consecutive += 1;
                    iter.next();
                }

                writeln!(&mut output_asm, " sub byte [r12], {}", consecutive).unwrap();
            }

            ',' => writeln!(&mut output_asm, " call getchar;\n mov [r12], al").unwrap(),

            '.' => writeln!(&mut output_asm, " mov dil, [r12];\n call putchar").unwrap(),

            '[' => {
                writeln!(
                    &mut output_asm,
                    "label{0}start:\n cmp byte [r12], 0\n jz label{0}end",
                    label_count
                )
                .unwrap();

                cola.push_back(label_count);
                label_count += 1;
            }

            ']' => writeln!(
                &mut output_asm,
                " jmp label{0}start\nlabel{0}end:",
                cola.pop_front().expect("No matching loop")
            )
            .unwrap(),

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
        .status()
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
