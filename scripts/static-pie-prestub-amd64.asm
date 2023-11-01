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
; [rsp+128, rsp+192]: PLATFORM_DATA
; [rsp+ 48, rsp+128]: SERVICE_FUNCTIONS
; [rsp-  8, rsp+120]: digittobin
; [rsp+  0, rsp+ 32]: (shadow space for win64 calling convention)
; [rsp+ 16, rsp+ 48]: (shadow space for win64 calling convention, only for next stage stub)
    and     rsp, 0xFFFFFFFFFFFFFFF0

; PLATFORM_DATA
    push    r12                     ; PLATFORM_DATA[56..63] = win_GetProcAddress
    pop     rbp
    push    rbp                     ; Perform mov rbp, r12
    push    r11                     ; PLATFORM_DATA[48..55] = win_GetModuleHandleW
    push    rsi                     ; PLATFORM_DATA[40..47] = pe_size_reloc
    push    rdi                     ; PLATFORM_DATA[32..39] = pe_off_reloc
    push    rdx                     ; PLATFORM_DATA[24..31] = pe_image_base
    push    rcx                     ; PLATFORM_DATA[16..23] = leading_unused_bytes
    xor     eax, eax
    test    ebp, ebp
    sete    al                      ; Enable ENV_FLAGS_LINUX_STYLE_CHKSTK outside Windows
    push    rax                     ; PLATFORM_DATA[ 8..15] = env_flags (0=None, 1=Enable debug breakpoint)
    inc     eax
    push    rax                     ; PLATFORM_DATA[ 0.. 7] = env_id (1=Windows, 2=Linux)

; SERVICE_FUNCTIONS
    push    rsp                     ; SERVICE_FUNCTIONS[72..79] = ptr_platform
    sub     rsp, 120                ; digittobin

; Allocate memory for stub
    lea     rbx, [rel _svc_alloc_rwx_linux] ; Register svc_alloc_rwx on Linux
    test    ebp, ebp
    jz      _u
    add     rbx, _svc_alloc_rwx_windows_pre - _svc_alloc_rwx_linux  ; Register svc_alloc_rwx on Windows
    lea     rcx, [rbx + _kernel32 - _svc_alloc_rwx_windows_pre]
    call    r11
    push    rax
    pop     rcx
    lea     rdx, [rbx + _VirtualAlloc - _svc_alloc_rwx_windows_pre]
    call    r12
    push    rax
    pop     rdi                     ; pointer to VirtualAlloc
_u:
    mov     rcx, 0x1000
    call    rbx

; Windows: copy svc_alloc_rwx to the new buffer
    test    ebp, ebp
    jz      _x
    lea     rcx, [rbx + _svc_alloc_rwx_windows - _svc_alloc_rwx_windows_pre]
    push    rax
    pop     rbx                     ; rbx = new svc_alloc_rwx
_v:
    mov     dl, byte [rcx]
    mov     byte [rax], dl
    inc     rcx                     ; src
    inc     rax                     ; dst
    cmp     dl, 0xc3                ; 'ret' instruction
    jne      _v
    mov     qword [rbx+2], rdi      ; pointer to VirtualAlloc
_x:
    push    rax
    pop     rdi
    mov     r12, rax

; Initialize base85 decoder buffer
    lea     rax, [rel _b85]         ; rax = _b85
    xor     ecx, ecx
_2:
    movzx   edx, byte [rax]         ; edx = start
    inc     rax
    movzx   esi, byte [rax]         ; esi = end
    inc     rax
_2a:
    mov     byte [rsp+rdx-8], cl
    inc     ecx
    inc     edx
    cmp     edx, esi
    jbe     _2a
    cmp     ecx, 85
    jb      _2

; Decode stub (rsi -> rdi; rsp = digittobin (rsp+8 after call instruction))
    mov     rsi, r13                ; rsi = STUB_BASE85
;   mov     rdi, r12                ; rdi = stub memory (already saved)
    call    _3

; Decode binary (rsi -> rdi; rsp = digittobin (rsp+8 after call instruction))
    mov     rsi, r14                ; rsi = BINARY_BASE85
    push    rsi
    pop     rdi                     ; rdi = BINARY_BASE85 (in-place decoding)
    call    _3

; Call stub
    add     rsp, 16                 ; Discard digittobin
    mov     qword [rsp+ 96], rbx    ; SERVICE_FUNCTIONS[64..71] = ptr_alloc_rwx
    lea     rcx, qword [rsp+ 32]    ; rcx = SERVICE_FUNCTIONS table
    mov     rdx, r14                ; rdx = LZMA-compressed binary
    mov     r8, r15                 ; r8  = Entrypoint offset
    xor     r9d, r9d                ; r9  = 1 if debugging is enabled, otherwise 0
    call    r12

; Base85 decoder
_3:
;   push    85                      ; ecx is already set to 85 just before calling the decoder
;   pop     rcx
_4:
    xor     ebp, ebp
    xor     eax, eax
_5:
    mul     ecx
    movzx   edx, byte [rsi]
    cmp     edx, 93                 ; 93 = 0x5D = b']' denotes end of base85 stream
    je      _6
    movzx   edx, byte [rsp+rdx]
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

; svc_alloc_rwx for Windows
; rcx = size
; rdx = pointer to VirtualAlloc ('pre' only)
_svc_alloc_rwx_windows:
    mov     rax, 0x0123456789ABCDEF
_svc_alloc_rwx_windows_pre:
    sub     rsp, 40                 ; shadow space
    push    rcx
    pop     rdx                     ; size
    xor     ecx, ecx
    mov     r8d, 0x3000             ; MEM_COMMIT | MEM_RESERVE
    push    0x40
    pop     r9                      ; PAGE_EXECUTE_READWRITE
    call    rax                     ; kernel32!VirtualAlloc
    add     rsp, 40
    ret

_kernel32:
    dw      'k','e','r','n','e','l','3','2',0

; b85 table ([start, end] encoding)
    align 8, db 0
_b85:
    dq 0x21217A615A413930
    dq 0x403B2D2D2B282623
    dd 0x7E7B605E

_VirtualAlloc:
    db      "VirtualAlloc"
    db      0