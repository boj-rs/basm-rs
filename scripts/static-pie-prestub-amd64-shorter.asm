; -*- tab-width: 4 -*-
; 
; The prestub for amd64-rust target
; (prestub: the code that runs before the stub and sets the stage)
;
; build: nasm -f bin -O9 static-pie-prestub-amd64-shorter.asm -o static-pie-prestub-amd64-shorter.bin
; note: after building with the above command, run static-pie-prestub-amd64-print.py static-pie-prestub-amd64-shorter.bin
;       to obtain the form that can be embedded in Rust as inline assembly.

BITS 64
ORG 0
section .text

; Align stack to 16 byte boundary
; [rsp+ 32, rsp+120): PLATFORM_DATA
; [rsp+  0, rsp+ 32): (shadow space for win64 calling convention)
    enter   56, 0

; svc_alloc_rwx for Linux
_svc_alloc_rwx:
    push    9
    pop     rax                     ; syscall id of x64 mmap
    cdq                             ; rdx=0
    xor     r9d, r9d                ; offset
    push    rsi                     ; save rsi
    xor     edi, edi                ; rdi=0
    push    1
    pop     rsi                     ; size
    mov     dl, 7                   ; protect (safe since we have ensured rdx=0)
    push    0x22
    pop     r10                     ; flags
    push    -1
    pop     r8                      ; fd
    syscall
    pop     rsi                     ; restore rsi

; PLATFORM_DATA
_t:                                 ; PLATFORM_DATA[32..39] = ptr_alloc_rwx
    push    rdx                     ; PLATFORM_DATA[24..31] = win_GetProcAddress
    push    rax                     ; PLATFORM_DATA[16..23] = win_kernel32
    push    1                       ; PLATFORM_DATA[ 8..15] = env_flags (0=None, 1=ENV_FLAGS_LINUX_STYLE_CHKSTK)
    push    2                       ; PLATFORM_DATA[ 0.. 7] = env_id (1=Windows, 2=Linux)

; Current state: rax = new buffer
    push    rax
    xchg    rax, rdi                ; rdi = new buffer

; Base91 decoder
_decode:
    mov     al, 0x1f                ; syscall preserves rax; hence at this point rax=9
_decode_loop:
    shl     eax, 13
_decode_loop_2:
    lodsb
    sub     al, 0x23
    cdq
    jc      _jump_to_entrypoint
    jz      _decode_zeros
    dec     al
    xchg    eax, edx
    lodsb
    sub     al, 0x24
    imul    eax, eax, 91
    add     eax, edx
_decode_output:
    stosb
    shr     eax, 8
    test    ah, 16
    jnz     _decode_output
    jmp     _decode_loop
_decode_zeros:
    xchg    eax, edx
    movzx   ecx, byte [rdi-1]
    dec     rdi
    rep     stosb
    xchg    eax, edx
    jmp     _decode_loop_2

; Jump to entrypoint
_jump_to_entrypoint:
    mov     eax, dword [rdi-4]
    pop     rcx
    add     rax, rcx
    push    rsp
    pop     rcx
    call    rax