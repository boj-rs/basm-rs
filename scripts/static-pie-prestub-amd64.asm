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
; [rsp+136, rsp+192): PLATFORM_DATA
; [rsp+ 56, rsp+136): SERVICE_FUNCTIONS
; [rsp- 16, rsp+112): digittobin
; [rsp+  0, rsp+ 32): (shadow space for win64 calling convention)
    and     rsp, 0xFFFFFFFFFFFFFFF0

; PLATFORM_DATA
    push    rdi                     ; PLATFORM_DATA[48..55] = win_GetProcAddress
    push    rax                     ; PLATFORM_DATA[40..47] = win_GetModuleHandleW
    push    rsi                     ; PLATFORM_DATA[32..39] = pe_size_reloc
    push    rdx                     ; PLATFORM_DATA[24..31] = pe_off_reloc
    push    rcx                     ; PLATFORM_DATA[16..23] = pe_image_base
    xor     ecx, ecx
    test    rdi, rdi
    sete    cl                      ; Enable ENV_FLAGS_LINUX_STYLE_CHKSTK outside Windows
    mov     ebp, ecx
    push    rcx                     ; PLATFORM_DATA[ 8..15] = env_flags (0=None, 1=ENV_FLAGS_LINUX_STYLE_CHKSTK)
    inc     ecx
    push    rcx                     ; PLATFORM_DATA[ 0.. 7] = env_id (1=Windows, 2=Linux)

; SERVICE_FUNCTIONS
    push    rsp                     ; SERVICE_FUNCTIONS[72..79] = ptr_platform
    add     rsp, -128               ; digittobin

; Allocate memory for stub
    lea     rbx, [rel _svc_alloc_rwx_linux] ; Register svc_alloc_rwx on Linux
    test    ebp, ebp
    jnz     _u
    add     rbx, _svc_alloc_rwx_windows_pre - _svc_alloc_rwx_linux  ; Register svc_alloc_rwx on Windows
    lea     rcx, [rbx + _kernel32 - _svc_alloc_rwx_windows_pre]
    call    rax
    push    rax
    pop     rcx
    lea     rdx, [rbx + _VirtualAlloc - _svc_alloc_rwx_windows_pre]
    call    rdi
    push    rax
    pop     rdi                     ; pointer to VirtualAlloc
_u:
    xor     ecx, ecx
    mov     ch, 0x10                ; rcx = 0x1000 (4K)
    call    rbx

; Windows: copy svc_alloc_rwx to the new buffer
    xchg    rax, rdi                ; rax = pointer to VirtualAlloc / rdi = new buffer
    test    ebp, ebp
    jnz     _x
    lea     rsi, [rbx + _svc_alloc_rwx_windows - _svc_alloc_rwx_windows_pre]
    push    rdi
    pop     rbx                     ; mov rbx, rdi
    push    _svc_alloc_rwx_windows_end - _svc_alloc_rwx_windows
    pop     rcx
    rep     movsb
    mov     qword [rbx+2], rax      ; pointer to VirtualAlloc
_x:
    mov     qword [rsp+120], rbx    ; SERVICE_FUNCTIONS[64..71] = ptr_alloc_rwx
    push    rdi

; Initialize base85 decoder buffer
    lea     rsi, [rel _b85]         ; rsi = _b85
    lea     rbx, [rsi + _3 - _b85]
    xor     ecx, ecx
_2:
    lodsb
    movzx   edx, al                 ; edx = start
    lodsb                           ; al = end
_2a:
    mov     byte [rsp+rdx-8], cl
    inc     ecx
    inc     edx
    cmp     dl, al
    jbe     _2a
    cmp     ecx, 85
    jb      _2

; Decode stub (rsi -> rdi; rsp = digittobin-8 (rsp+0 after call instruction))
    mov     rsi, r13                ; rsi = STUB_BASE85
;   mov     rdi, r12                ; rdi = stub memory (already saved)
    call    rbx

; Decode binary (rsi -> rdi; rsp = digittobin-8 (rsp+0 after call instruction))
    mov     rsi, r14                ; rsi = BINARY_BASE85
    push    rsi
    pop     rdi                     ; rdi = BINARY_BASE85 (in-place decoding)
    call    rbx

; Call stub
    pop     rax
    lea     rcx, qword [rsp+ 56]    ; rcx = SERVICE_FUNCTIONS table
    mov     rdx, r14                ; rdx = LZMA-compressed binary
    mov     r8, r15                 ; r8  = Entrypoint offset
    xor     r9d, r9d                ; r9  = 1 if debugging is enabled, otherwise 0
    call    rax

; Base85 decoder
_3:
;   push    85                      ; ecx is already set to 85 just before calling the decoder
;   pop     rcx
_4:
    xor     ebp, ebp
    xor     eax, eax
_5:
    mul     ecx
    xchg    eax, edx
    lodsb
    cmp     al, 93                  ; 93 = 0x5D = b']' denotes end of base85 stream
    je      _ret
    movzx   eax, byte [rsp+rax]
    add     eax, edx
    inc     ebp
    cmp     ebp, 5
    jl      _5
    bswap   eax
    stosd                           ; stores eax to dword [rdi] and increment rdi by 4
    jmp     _4

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
_ret:
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
_svc_alloc_rwx_windows_end:

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