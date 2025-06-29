; -*- tab-width: 4 -*-
; 
; The prestub for amd64-C target
; (prestub: the code that runs before the stub and sets the stage)
;
; build: nasm -f bin -O9 static-pie-prestub-amd64-shorter-c.asm -o static-pie-prestub-amd64-shorter-c.bin
; note: after building with the above command, run static-pie-prestub-amd64-print.py static-pie-prestub-amd64-shorter-c.bin --c --signed --no-asciz
;       to obtain the form that can be embedded in C.
; note: mmap, mprotect, munmap applies to the pages in [addr, addr+len-1], possibly except for huge pages

BITS 64
ORG 0
section .text

; Function: __libc_start_main
; Parameters:
; * RDI (main): In our case, the address of binary_raw_base91.
; * RSI (argc): Always 1 when run from BOJ
; * RDX (ubp_av): a.k.a the argv pointer
; * RCX (init): May or may not be NULL
; * R8  (fini): May or may not be NULL
; * R9  (rtld_fini): May or may not be NULL
;
; See also: https://refspecs.linuxbase.org/LSB_3.0.0/LSB-PDA/LSB-PDA/baselib---libc-start-main-.html
__libc_start_main:

; Reserve space on stack
    and     rsp, 0xffffffffffffff80 ; ensures at least 128 bytes

; mprotect: make stack executable
    push    10                      ; mprotect
    pop     rax
    push    rdi                     ; Save binary_raw_base91 to rbx
    pop     rbx
    lea     rsi, [rel _start]
    push    rsp                     ; addr
    pop     rdi

; Relocate to stack
    push    _end - _start + 8        ; binary size in bytes
    pop     rcx
    rep     movsb

    push    7                       ; protect (RWX)
    pop     rdx                     ; (*) reused below for mmap
    jmp     _syscall_4k

_start:
; Free the .text section
    pop     rdi                     ; Get RIP saved on stack by call instruction
    mov     al, 11                  ; prev syscall already zeroed rax, assuming it succeeded
_syscall_4k:
    mov     esi, eax                ; len (does not need to be page-aligned)
    and     rdi, 0xfffffffffffff000 ; align to page boundary (4K)
    syscall
    shr     esi, 1                  ; prevent infinite loop
    jc      _svc_alloc_rwx          ; taken if esi has lowest bit set; jump not taken when esi=10, taken when esi=11

; Jump to stack
    call    rsp                     ; _start of relocated stub

; svc_alloc_rwx for Linux
_svc_alloc_rwx:
    mov     al, 9                   ; syscall id of x64 mmap / prev call already zeroed upper 32bit of rax
    push    0x22
    ;mov     dl, 7                  ; protect; we reuse RDX from above (*)
    xor     edi, edi                ; rdi=0
    pop     r10                     ; flags
    push    -1
    mov     esi, dword [rel _end]   ; size in bytes (we assume the code size will be <4GiB)
    pop     r8                      ; fd
    xor     r9d, r9d                ; offset
    syscall
    push    rbx
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
    ;xor     ecx, ecx               ; RCX=0 marks we do not supply PLATFORM_DATA even though we are running with loader
                                    ; Also, RCX=0 is ensured by the above `rep stosb`, which decrements `cl` to zero.
    sub     rdi, qword [rdi-8]
    jmp     rdi

    align 8, db 0x0                 ; zero padding
_end: