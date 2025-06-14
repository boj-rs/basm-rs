; -*- tab-width: 4 -*-
; 
; The prestub for amd64-C target
; (prestub: the code that runs before the stub and sets the stage)
;
; build: nasm -f bin -O9 static-pie-prestub-amd64-shorter-c.asm -o static-pie-prestub-amd64-shorter-c.bin
; note: after building with the above command, run static-pie-prestub-amd64-print.py static-pie-prestub-amd64-shorter-c.bin --c --signed --no-asciz
;       to obtain the form that can be embedded in C.

BITS 64
ORG 0
section .text

; Reserve space on stack
    nop
    and     rsp, 0xffffffffffffff80 ; ensures at least 128 bytes

; mprotect: make stack executable
    mov     eax, 10                 ; mprotect
    mov     esi, 0x1000             ; len
    push    rdi                     ; Save binary_raw_base91
    lea     rdi, [rsp + 8]          ; addr
    push    7                       ; protect (RWX)
    pop     rdx
    and     rdi, 0xfffffffffffff000 ; align to page boundary (4K)
    syscall

; Relocate to stack
    lea     rsi, [rel _start]
    lea     rdi, [rsp + 8]
    push    rdi                     ; _start of relocated stub
    mov     ecx, _end - _start
    add     ecx, 8                  ; binary size in bytes
    rep     movsb

; Jump to stack
    pop     rax                     ; _start of relocated stub
    call    rax

_start:

; Free the .text section
    pop     rdi                     ; Get RIP saved on stack by call instruction
    and     rdi, 0xfffffffffffff000
    mov     esi, 0x1000
    mov     eax, 11
    syscall

; svc_alloc_rwx for Linux
_svc_alloc_rwx:
    push    9
    pop     rax                     ; syscall id of x64 mmap
    cdq                             ; rdx=0
    xor     r9d, r9d                ; offset
    xor     edi, edi                ; rdi=0
    mov     rsi, qword [rel _end]   ; size in bytes
    mov     dl, 7                   ; protect (safe since we have ensured rdx=0)
    push    0x22
    pop     r10                     ; flags
    push    -1
    pop     r8                      ; fd
    syscall
    pop     rsi                     ; restore rsi

; Current state: rax = new buffer
    xchg    rax, rdi                ; rdi = new buffer

; Base91 decoder
_decode:
    mov     al, 0x1f                ; syscall preserves all registers except rcx, r11, rax; hence at this point rax=(previous rdi)=0
_decode_loop:
    shl     eax, 13
_decode_loop_2:
    lodsb
    xor     ecx, ecx                ; ecx = 0
    sub     al, 0x23
    jbe     _decode_zeros
    dec     al
    xchg    eax, ecx
    lodsb
    sub     al, 0x24
    imul    eax, eax, 91
    add     eax, ecx
_decode_output:
    stosb
    shr     eax, 8
    test    ah, 16
    jnz     _decode_output
    jmp     _decode_loop
_decode_zeros:
    xchg    byte [rdi-1], cl        ; ecx = cl = ((number of zeros) - 1), byte [rdi-1] = 0
    rep     stosb                   ; we have made sure the last byte is zero (in the packager)
    jz      _decode_loop_2

; Jump to entrypoint
_jump_to_entrypoint:
    sub     rdi, qword [rdi-8]
    xor     ecx, ecx
    push    rcx
    call    rdi

    align 8, db 0x0                 ; zero padding
_end: