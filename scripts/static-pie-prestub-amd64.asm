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
; [rsp+ 32, rsp+144): PLATFORM_DATA
; [rsp+  0, rsp+ 32): (shadow space for win64 calling convention)
    push    rbx
    enter   80, 0
    and     rsp, 0xFFFFFFFFFFFFFFF0

; PLATFORM_DATA
    push    rax                     ; PLATFORM_DATA[24..31] = win_GetProcAddress
    push    rcx                     ; PLATFORM_DATA[16..23] = win_kernel32
    xor     edx, edx
    test    rax, rax
    sete    dl                      ; Enable ENV_FLAGS_LINUX_STYLE_CHKSTK outside Windows
    push    rdx                     ; PLATFORM_DATA[ 8..15] = env_flags (0=None, 1=ENV_FLAGS_LINUX_STYLE_CHKSTK)
    inc     edx
    push    rdx                     ; PLATFORM_DATA[ 0.. 7] = env_id (1=Windows, 2=Linux)
    sub     rsp, 32                 ; shadow space

; Allocate memory for stub
    lea     rsi, [rel _svc_alloc_rwx]   ; Register svc_alloc_rwx
    test    rax, rax
    jz      _u
    lea     rdx, [rsi + _VirtualAlloc - _svc_alloc_rwx]
    call    rax                     ; after the call, rax = pointer to VirtualAlloc
_u:
    push    rax
    pop     rbx                     ; rbx = pointer to VirtualAlloc
    push    1
    pop     rcx                     ; rcx = 1 -> will be rounded up to the nearest page size, which is 0x1000 (4K)
    call    rsi                     ; svc_alloc_rwx

; Copy svc_alloc_rwx to the new buffer
; Current state: rax = new buffer, rbx = pointer to VirtualAlloc, rsi = svc_alloc_rwx
    mov     qword [rsp+56+32], rax  ; PLATFORM_DATA[56..63] = ptr_alloc_rwx (on the new buffer)
    xchg    rax, rdi                ; rdi = new buffer
    mov     ax, 0xB848              ; mov rax, STRICT QWORD imm64
    stosw
    xchg    rax, rbx                ; rax = pointer to VirtualAlloc
    stosq
    push    _svc_alloc_rwx_end - _svc_alloc_rwx
    pop     rcx
    rep     movsb                   ; this progresses rsi to _decode
    push    rsi
    pop     rbx                     ; rbx = _decode
    push    rdi
    push    r14

; Decode stub (rsi -> rdi)
; Current state: rdi = stub memory
    mov     rsi, r13                ; rsi = STUB_BASE91
    call    rbx

; Decode binary (rsi -> rdi)
    pop     rsi                     ; rsi = BINARY_BASE91
    push    rsi
    pop     rdi                     ; rdi = BINARY_BASE91 (in-place decoding)
    push    rdi
    call    rbx

; Call stub
    pop     rdx                     ; rdx = LZMA-compressed binary
    pop     rax                     ; rax = stub entrypoint
    lea     rcx, qword [rsp+32]     ; rcx = PLATFORM_DATA table
    call    rax
    leave
    pop     rbx
    jmp     _end_of_everything

; svc_alloc_rwx for Windows and Linux
; rcx = size
; rax = pointer to VirtualAlloc (must be supplied before prepending the mov instruction)
_svc_alloc_rwx:
    test    rax, rax
    jz      _svc_alloc_rwx_linux
_svc_alloc_rwx_windows:
    push    rcx
    pop     rdx                     ; size
    xor     ecx, ecx
    mov     r8d, 0x3000             ; MEM_COMMIT | MEM_RESERVE
    push    0x40
    pop     r9                      ; PAGE_EXECUTE_READWRITE
    jmp     rax                     ; kernel32!VirtualAlloc
_svc_alloc_rwx_linux:
    push    rsi                     ; save rsi
    mov     al, 9                   ; syscall id of x64 mmap (safe since we have ensured rax=0)
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
    pop     rsi                     ; restore rsi
_ret:
    ret
_svc_alloc_rwx_end:

; Base91 decoder
_decode:
    push    0x1f
    pop     rax
_decode_loop:
    shl     eax, 13
    lodsb
    sub     al, 0x24
    jc      _ret
    cdq
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

align 8, db 0

_VirtualAlloc:
    db      "VirtualAlloc"
    db      0

_end_of_everything: