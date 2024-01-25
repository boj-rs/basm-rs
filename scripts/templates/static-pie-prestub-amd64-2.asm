; -*- tab-width: 4 -*-
; 
; The prestub 2 for amd64-rust target
; (prestub 2: the code that runs after the prestub but before the stub and sets the stage)
;
; build: nasm -f bin -O9 static-pie-prestub-amd64-2.asm -o static-pie-prestub-amd64-2.bin

BITS 64
ORG 0
section .text

; Decode binary (rsi -> rdi)
    push    rdi
    push    r14
    pop     rsi                     ; rsi = BINARY_BASE91
    push    rsi
    pop     rdi                     ; rdi = BINARY_BASE91 (in-place decoding)
    push    rdi
    call    rbx

; Prepare for stub
    pop     rdx                     ; rdx = LZMA-compressed binary
    pop     rdi
    lea     rcx, qword [rsp+40]     ; rcx = PLATFORM_DATA table