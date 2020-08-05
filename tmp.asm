global _start
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
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
label0start:
cmp byte [r12], 0
jz label0end
add r12, 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add r12, 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add r12, 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add r12, 1
add byte [r12], 1
sub r12, 1
sub r12, 1
sub r12, 1
sub r12, 1
sub byte [r12], 1
 jmp label0start
label0end:
add r12, 1
add byte [r12], 1
add byte [r12], 1
mov dil, [r12];
call putchar
add r12, 1
add byte [r12], 1
mov dil, [r12];
call putchar
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
mov dil, [r12];
call putchar
mov dil, [r12];
call putchar
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
mov dil, [r12];
call putchar
add r12, 1
add byte [r12], 1
add byte [r12], 1
mov dil, [r12];
call putchar
sub r12, 1
sub r12, 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
mov dil, [r12];
call putchar
add r12, 1
mov dil, [r12];
call putchar
add byte [r12], 1
add byte [r12], 1
add byte [r12], 1
mov dil, [r12];
call putchar
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
mov dil, [r12];
call putchar
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
sub byte [r12], 1
mov dil, [r12];
call putchar
add r12, 1
add byte [r12], 1
mov dil, [r12];
call putchar
add r12, 1
mov dil, [r12];
call putchar
add rsp, 4064
mov eax,0
call exit