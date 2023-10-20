; -*- tab-width: 4 -*-
; 
; The prestub for amd64-rust target
; (prestub: the code that runs before the stub and sets the stage)
;
; build: nasm -f bin -O9 static-pie-prestub-amd64.asm -o static-pie-prestub-amd64.bin
; note: after building with the above command, run static-pie-prestub-amd64-print.py
;       to obtain the form that can be embedded in Rust as inline assembly.

BITS 64
ORG 0
section .text

; Align stack to 16 byte boundary
; [rsp+368, rsp+432]: PLATFORM_DATA
; [rsp+288, rsp+368]: SERVICE_FUNCTIONS
; [rsp+ 32, rsp+288]: digittobin
; [rsp+  0, rsp+ 32]: (shadow space for win64 calling convention)
    and     rsp, 0xFFFFFFFFFFFFFFF0
    sub     rsp, 432

; PLATFORM_DATA
    lea     rcx, qword [rsp+368]    ; rcx = PLATFORM_DATA table
    mov     qword [rcx+  0], r8     ; env_id
    xor     eax, eax
    cmp     r8, 1
    setne   al                      ; Enable ENV_FLAGS_LINUX_STYLE_CHKSTK outside Windows
    mov     qword [rcx+  8], rax    ; env_flags
    mov     qword [rcx+ 16], r9     ; leading_unused_bytes
    mov     qword [rcx+ 24], rdx    ; pe_image_base
    mov     qword [rcx+ 32], rdi    ; pe_off_reloc
    mov     qword [rcx+ 40], rsi    ; pe_size_reloc
    mov     qword [rcx+ 48], r10    ; win_GetModuleHandleW
    mov     qword [rcx+ 56], r11    ; win_GetProcAddress

; SERVICE_FUNCTIONS
    lea     rax, qword [rsp+288]    ; rax = SERVICE_FUNCTIONS table
;   mov     qword [rax+  0], 0      ; ptr_imagebase
;   mov     qword [rax+  8], 0      ; ptr_alloc
;   mov     qword [rax+ 16], 0      ; ptr_alloc_zeroed
;   mov     qword [rax+ 24], 0      ; ptr_dealloc
;   mov     qword [rax+ 32], 0      ; ptr_realloc
;   mov     qword [rax+ 40], 0      ; ptr_exit
;   mov     qword [rax+ 48], 0      ; ptr_read_stdio
;   mov     qword [rax+ 56], 0      ; ptr_write_stdio
    mov     qword [rax+ 64], r12    ; ptr_alloc_rwx
    mov     qword [rax+ 72], rcx    ; ptr_platform

; Initialize base85 decoder buffer
    lea     rax, [rel _7]           ; rax = b85
    lea     rcx, qword [rsp+ 32]    ; rcx = digittobin
    xor     ebx, ebx
_2:
    movzx   edx, byte [rax+rbx]     ; Upper 32bit of rdx automatically gets zeroed
    mov     byte [rcx+rdx], bl
    inc     ebx
    cmp     ebx, 85
    jb      _2

; Allocate memory for stub
    mov     rcx, 0x1000
    call    r12
    mov     r12, rax                ; r12 = stub memory

; Decode stub (rsi -> rdi; rcx = digittobin)
    lea     rcx, qword [rsp+ 32]    ; rcx = digittobin
    mov     rsi, r13                ; rsi = STUB_BASE85
    mov     rdi, r12                ; rdi = stub memory
    call    _3

; Decode binary (rsi -> rdi; rcx = digittobin)
    mov     rsi, r14                ; rsi = BINARY_BASE85
    mov     rdi, rsi                ; rdi = BINARY_BASE85 (in-place decoding)
    call    _3

; Call stub
    lea     rcx, qword [rsp+288]    ; rcx = SERVICE_FUNCTIONS table
    mov     rdx, r14                ; rdx = LZMA-compressed binary
    mov     r8, r15                 ; r8  = Entrypoint offset
    mov     r9, 0                   ; r9  = 1 if debugging is enabled, otherwise 0
    add     rsp, 256                ; Discard digittobin
    call    r12

; Base85 decoder
_3:
    mov     ebx, 85
_4:         
    movzx   eax, byte [rsi]
    cmp     eax, 93                 ; 93 = 0x5D = b']' denotes end of base85 stream
    je      _6
    xor     ebp, ebp
    xor     eax, eax
_5:
    mul     ebx
    movzx   edx, byte [rsi+rbp]
    movzx   edx, byte [rcx+rdx]
    add     eax, edx
    inc     ebp
    cmp     ebp, 5
    jl      _5
    bswap   eax
    mov     dword [rdi], eax
    add     rsi, 5
    add     rdi, 4
    jmp     _4
_6:
    ret

; b85 table
_7:
    dq      0x3736353433323130
    dq      0x4645444342413938
    dq      0x4E4D4C4B4A494847
    dq      0x565554535251504F
    dq      0x646362615A595857
    dq      0x6C6B6A6968676665
    dq      0x74737271706F6E6D
    dq      0x23217A7978777675
    dq      0x2D2B2A2928262524
    dq      0x5F5E403F3E3D3C3B
    dd      0x7D7C7B60
    db      0x7E