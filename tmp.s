.intel_syntax noprefix
.globl main
main:
  push 3
  push 4
  push 2
  push 1
  sub rax, rdi
  push rax
  imul rax, rdi
  push rax
  add rax, rdi
  push rax
  ret
