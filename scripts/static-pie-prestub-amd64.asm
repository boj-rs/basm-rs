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
; [rsp+208, rsp+272]: PLATFORM_DATA
; [rsp+128, rsp+208]: SERVICE_FUNCTIONS
; [rsp+  0, rsp+128]: digittobin
; [rsp+  0, rsp+ 32]: (shadow space for win64 calling convention)
    and     rsp, 0xFFFFFFFFFFFFFFF0
    mov     rbp, r12

; PLATFORM_DATA
    push    r12                     ; PLATFORM_DATA[56..63] = win_GetProcAddress
    push    r11                     ; PLATFORM_DATA[48..55] = win_GetModuleHandleW
    push    rsi                     ; PLATFORM_DATA[40..47] = pe_size_reloc
    push    rdi                     ; PLATFORM_DATA[32..39] = pe_off_reloc
    push    rdx                     ; PLATFORM_DATA[24..31] = pe_image_base
    push    r9                      ; PLATFORM_DATA[16..23] = leading_unused_bytes
    xor     eax, eax
    test    ebp, ebp
    jnz      _1
    inc     eax                     ; Enable ENV_FLAGS_LINUX_STYLE_CHKSTK outside Windows
_1:
    push    rax                     ; PLATFORM_DATA[ 8..15] = env_flags (0=None, 1=Enable debug breakpoint)
    inc     eax
    push    rax                     ; PLATFORM_DATA[ 0.. 7] = env_id (1=Windows, 2=Linux)

; SERVICE_FUNCTIONS
    push    rsp                     ; SERVICE_FUNCTIONS[72..79] = ptr_platform
    sub     rsp, 200                ; 72 + 128

; Allocate memory for stub
    lea     rbx, [rel _svc_alloc_rwx_linux] ; Register svc_alloc_rwx on Linux
    test    ebp, ebp
    jz      _u
    lea     rbx, [rel _svc_alloc_rwx_windows_pre]   ; Register svc_alloc_rwx on Windows
    lea     rcx, [rel _kernel32]
    call    r11
    mov     rcx, rax
    lea     rdx, [rel _VirtualAlloc]
    call    r12
    mov     r12, rax
    mov     rdx, rax                ; pointer to VirtualAlloc
    stc
_u:
    mov     rcx, 0x1000
    call    rbx

; Windows: copy svc_alloc_rwx to the new buffer
    test    ebp, ebp
    jz      _x
    mov     rbx, rax                ; rbx = new svc_alloc_rwx
    lea     rcx, [rel _svc_alloc_rwx_windows]
_v:
    mov     dl, byte [rcx]
    mov     byte [rax], dl
    inc     rcx                     ; src
    inc     rax                     ; dst
    cmp     dl, 0xc3                ; 'ret' instruction
    jne      _v
    mov     qword [rbx+3], r12      ; pointer to VirtualAlloc
_x:
    mov     r12, rax

; Register svc_alloc_rwx
    mov     qword [rsp+192], rbx    ; SERVICE_FUNCTIONS[64..71] = ptr_alloc_rwx

; Initialize base85 decoder buffer
    lea     rax, [rel _7]           ; rax = b85
    xor     ecx, ecx
_2:
    movzx   edx, byte [rax+rcx]     ; Upper 32bit of rdx automatically gets zeroed
    mov     byte [rsp+rdx], cl
    inc     ecx
    cmp     ecx, 85
    jb      _2

; Decode stub (rsi -> rdi; rsp = digittobin (rsp+8 after call instruction))
    mov     rsi, r13                ; rsi = STUB_BASE85
    mov     rdi, r12                ; rdi = stub memory
    call    _3

; Decode binary (rsi -> rdi; rsp = digittobin (rsp+8 after call instruction))
    mov     rsi, r14                ; rsi = BINARY_BASE85
    mov     rdi, rsi                ; rdi = BINARY_BASE85 (in-place decoding)
    call    _3

; Call stub
    add     rsp, 96                 ; Discard digittobin
    lea     rcx, qword [rsp+ 32]    ; rcx = SERVICE_FUNCTIONS table
    mov     rdx, r14                ; rdx = LZMA-compressed binary
    mov     r8, r15                 ; r8  = Entrypoint offset
    xor     r9d, r9d                ; r9  = 1 if debugging is enabled, otherwise 0
    call    r12

; Base85 decoder
_3:
    push    85
    pop     rcx
_4:
    xor     ebp, ebp
    xor     eax, eax
_5:
    mul     ecx
    movzx   edx, byte [rsi]
    cmp     edx, 93                 ; 93 = 0x5D = b']' denotes end of base85 stream
    je      _6
    movzx   edx, byte [rsp+rdx+8]
    add     eax, edx
    inc     rsi
    inc     ebp
    cmp     ebp, 5
    jl      _5
    bswap   eax
    mov     dword [rdi], eax
    add     rdi, 4
    jmp     _4
_6:
    ret

; svc_alloc_rwx for Windows
; rcx = size
; rdx = pointer to VirtualAlloc ('pre' only)
_svc_alloc_rwx_windows:
    clc
_svc_alloc_rwx_windows_pre:
    mov     rax, 0x0123456789ABCDEF
    cmovb   rax, rdx
    sub     rsp, 40                 ; shadow space
    mov     rdx, rcx                ; size
    xor     ecx, ecx
    mov     r8d, 0x3000             ; MEM_COMMIT | MEM_RESERVE
    push    0x40
    pop     r9                      ; PAGE_EXECUTE_READWRITE
    call    rax                     ; kernel32!VirtualAlloc
    add     rsp, 40
    ret

; svc_alloc_rwx for Linux
; rcx = size
_svc_alloc_rwx_linux:
    push    9
    pop     rax                     ; syscall id of x64 mmap
    xor     edi, edi
    mov     esi, ecx                ; size
    push    7
    pop     rdx                     ; protect
    push    0x22
    pop     r10                     ; flags
    push    -1
    pop     r8                      ; fd
    xor     r9d, r9d                ; offset
    syscall
    ret

_kernel32:
    dw      'k','e','r','n','e','l','3','2',0

_VirtualAlloc:
    db      "VirtualAlloc"
    db      0

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