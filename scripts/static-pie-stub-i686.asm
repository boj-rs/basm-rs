; -*- tab-width: 4 -*-
; Copyright (c) 2022, Ilya Kurdyukov
; Copyright (c) 2023, Byeongkeun Ahn
; All rights reserved.
;
; Micro LZMA decoder for x86 (static)
;
; This software is distributed under the terms of the
; Creative Commons Attribution 3.0 License (CC-BY 3.0)
; http://creativecommons.org/licenses/by/3.0/
;
; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
; OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
; FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
; AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
; LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
; OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
; THE SOFTWARE.

; build: nasm -f bin -O9 static-pie-stub-i686.asm -o static-pie-stub-i686.bin

BITS 32
ORG 0
section .text

%assign loc_pos 0
%macro LOC 1-3 4, dword
%assign loc_pos loc_pos+%2
%ifidn %3, none
%xdefine %1 [ebp-loc_pos]
%else
%xdefine %1 %3 [ebp-loc_pos]
%endif
%endmacro
LOC Code
%assign loc_code loc_pos
LOC Range
%assign loc_range loc_pos
LOC _rep0
LOC _rep1
LOC _rep2
LOC _rep3
%assign loc_rep loc_pos
LOC _state

%define _rc_bit edi
; src + 1+4+8+1+4
%define Src dword [ebp+16]
%define Dest dword [ebp+12]
%define Temp dword [ebp+8]


; [ebp + 12] = Src
; [ebp +  8] = the PLATFORM_DATA table
; [esp + 32] = Dest (original)
; [esp + 28] = the first 4 bytes of the LZMA stream     (callee: [ebp + 36])
; [esp + 24] = tsize                                    (callee: [ebp + 32])
; [esp + 20] = (1 << pb) - 1                            (callee: [ebp + 28])
; [esp + 16] = (1 << lp) - 1                            (callee: [ebp + 24])
; [esp + 12] = lc                                       (callee: [ebp + 20])
; [esp +  8] = Src + 18                                 (callee: [ebp + 16])
; [esp +  4] = Dest                                     (callee: [ebp + 12])
; [esp +  0] = Temp                                     (callee: [ebp +  8])
_start:
    push    ebp
    mov     ebp, esp
    sub     esp, 40
    mov     esi, dword [ebp + 12]
    movzx   eax, byte [esi + 0]
    xor     edx, edx
    xor     ecx, ecx
    mov     cl, 45
    div     ecx
    xor     ebx, ebx
    bts     ebx, eax
    dec     ebx
    mov     dword [esp + 20], ebx   ; [esp + 20] = (1 << pb) - 1
    mov     eax, edx
    xor     edx, edx
    mov     cl, 9
    div     ecx             ; eax = lp, edx = lc
    lea     ecx, [eax + edx + 8]
    xor     ebx, ebx
    bts     ebx, eax
    dec     ebx
    mov     dword [esp + 16], ebx   ; [esp + 16] = (1 << lp) - 1
    mov     dword [esp + 12], edx   ; [esp + 12] = lc

    mov     al, 3
    shl     eax, cl
    add     eax, 2048
    mov     dword [esp + 24], eax   ; [esp + 24] = tsize

    mov     ebx, dword [ebp + 8]    ; ebx = PLATFORM_DATA table
    mov     esi, dword [ebp + 12]
    mov     edi, dword [esi + 5]    ; edi = decompressed size of payload
    sub     esp, 12
    push    edi                     ; svc_alloc_rwx: size of memory
    call    dword [ebx + 32]        ; allocate the Dest memory
    add     esp, 16
    mov     dword [esp + 4], eax    ; [esp +  4] = Dest
    mov     dword [esp + 32], eax   ; [esp + 32] = Dest

    mov     edi, dword [esp + 24]
    shl     edi, 1
    sub     esp, 8
    push    1                       ; svc_alloc: alignment (required by Rust)
    push    edi                     ; svc_alloc: size of memory
    call    dword [ebx + 36]        ; allocate the Temp memory
    add     esp, 16
    mov     dword [esp + 0], eax    ; [esp +  0] = Temp

    mov     esi, dword [ebp + 12]
    mov     edi, dword [esi + 14]
    bswap   edi                     ; edi = initial 32 bits of the stream
                                    ; Note: the first byte of the LZMA stream is always the zero byte (ignored)
    mov     dword [esp + 28], edi   ; [esp + 28] = initial 32 bits of the stream
    add     esi, 18                 ; esi = Src + 18
    mov     dword [esp + 8], esi    ; [esp +  8] = Src + 18

    call    _lzma_dec

    mov     edi, dword [esp + 0]
    mov     eax, dword [esp + 24]
    shl     eax, 1
    sub     esp, 4
    push    1                       ; svc_free: alignment of memory to be freed (required by Rust)
    push    eax                     ; svc_free: size of memory to be freed (required by Rust)
    push    edi                     ; svc_free: ptr to be freed
    mov     ebx, dword [ebp + 8]    ; ebx = PLATFORM_DATA table (since _lzma_dec clobbers ebx)
    call    dword [ebx + 44]        ; free the Temp memory
    add     esp, 16

    mov     edx, dword [esp + 4]    ; edx = (End of the decompressed data)
    mov     ecx, dword [esp + 32]   ; ecx = Dest
    add     ecx, dword [edx - 8]    ; add entrypoint_offset
    mov     dword [esp + 0], ebx    ; the PLATFORM_DATA table
    call    ecx                     ; call the entrypoint of the binary
    add     esp, 40
    pop     ebp
    ret

_lzma_dec:
    enter   0, 0
    mov     eax, dword [ebp + 36]
    push    eax         ; Code
_rel_code:
    push    -1          ; Range
    xor     eax, eax
    inc     eax
    push    eax
    push    eax
    push    eax
    push    eax
    mov     ecx, dword [ebp + 32]
_rel_tsize:
    mov     edi, Temp
    shl     eax, 10
    rep     stosw
    push    ecx         ; _state
    ; bh=4, but it doesn't matter
    xchg    ebx, eax    ; Prev = 0
    call    _loop1
_rc_bit1:
    push    edx
    push    esi
    call    _rc_norm
    shl     esi, 1
    add     esi, Temp
    movzx   eax, word [esi]
    mov     edx, Range
    shr     edx, 11
    imul    edx, eax    ; bound
    sub     Range, edx
    sub     Code, edx
    jae     .1
    mov     Range, edx
    add     Code, edx
    cdq
    sub     eax, 2048-31
.1: shr     eax, 5        ; eax >= 0
    sub     [esi], ax
    neg     edx
    pop     esi
    pop     edx
    ret

_rc_norm:
    cmp     byte [ebp-loc_range+3], 0
    jne     .1
    shl     Range, 8
    shl     Code, 8
    mov     eax, Src
    mov     al, [eax]
    inc     Src
    mov     [ebp-loc_code], al
.1: ret

_loop1:
    pop     _rc_bit
_loop:
    mov     ecx, Dest
    mov     bh, byte [ebp + 24]
_rel_lp:
    pop     esi        ; _state
    push    esi
    and     bh, cl
    and     ecx, dword [ebp + 28]    ; posState
_rel_pb:
    shl     esi, 5        ; state * 16

    ; probs + state * 16 + posState
    lea     esi, [esi+ecx*2+64]
    call    _rc_bit
    cdq
    pop     eax
    jc      _case_rep
    mov     ecx, dword [ebp + 20]
    shl     ebx, cl
_rel_lc:
    mov     bl, 0
    lea     ecx, [ebx+ebx*2+2048]
_case_lit:
    lea     ebx, [edx+1]
    ; state = 0x546543210000 >> state * 4 & 15;
    ; state = state < 4 ? 0 : state - (state > 9 ? 6 : 3)
.4: add     al, -3
    sbb     dl, dl
    and     al, dl
    cmp     al, 7
    jae     .4
    push    eax        ; _state
%if 0    ; -2 bytes, but slower
    ; will read one byte before Dest
    add     al, -4
    sbb     bh, bh
%else
    cmp     al, 7-3
    jb      .2
    mov     bh, 1     ; offset
%endif
    mov     eax, Dest
    sub     eax, _rep0
    ; dl = -1, dh = 0, bl = 1
    xor     dl, [eax]
.1: xor     dh, bl
    and     bh, dh
.2: shl     edx, 1
    mov     esi, ebx
    and     esi, edx
    add     esi, ebx
    add     esi, ecx
    call    _rc_bit
    adc     bl, bl
    jnc     .1
    cdq     ; _len
    jmp     _copy.2

_case_rep:
    mov     ebx, esi
    lea     esi, [edx+eax*4+16]    ; IsRep
    add     al, -7
    sbb     al, al
    and     al, 3
    push    eax        ; _state
    call    _rc_bit
    jc      .2
    pop     eax        ; _state
    pop     ebx        ; r3
    pop     ebx        ; ebx = r2
    pop     esi        ; esi = r1
    push    _rep0        ; r1 = r0
    push    esi        ; r2 = r1
    push    ebx        ; r3 = r2
    push    eax        ; _state
    ; state = state < 7 ? 0 : 3
    mov     dl, 819/9    ; LenCoder
    jmp     _case_len

.2: inc     esi
    call    _rc_bit
    jc      .3
    lea     esi, [ebx+1]    ; IsRep0Long
    call    _rc_bit
    jc      .5
    ; state = state < 7 ? 9 : 11
    or      _state, 9
    ; edx = 0, _len
    jmp     _copy.1

.3: mov     dl, 3
    mov     ebx, _rep0
.6: inc     esi
    dec     edx
    xchg    [ebp-loc_rep+edx*4], ebx
    je      .4
    call    _rc_bit
    jc      .6
.4: mov     _rep0, ebx
.5:     ; state = state < 7 ? 8 : 11
    or      _state, 8
    mov     dl, 1332/9    ; RepLenCoder
_case_len:
    lea     esi, [edx*8+edx]
    cdq
    call    _rc_bit
    inc     esi
    lea     ebx, [esi+ecx*8]    ; +1 unnecessary
    mov     cl, 3
    jnc     .4
    inc     edx    ; edx = 8/8
    call    _rc_bit
    jnc     .3
    ; the first byte of BitTree tables is not used,
    ; so it's safe to add 255 instead of 256 here
    lea     ebx, [esi+127]
    mov     cl, 8
    add     edx, 16/8-(1<<8)/8    ; edx = -29
.3: sub     ebx, -128    ; +128
.4:    ; BitTree
    push    1
    pop     esi
    push    esi
.5: push    esi
    add     esi, ebx
    call    _rc_bit
    pop     esi
    adc     esi, esi
    loop    .5
    lea     ebx, [esi+edx*8+2-8-1]
    cmp     _state, 4
    pop     edx    ; edx = 1
    push    ebx    ; _len
    jae     _copy
_case_dist:
    add     _state, 7
    sub     ebx, 3+2-1
    sbb     eax, eax
    and     ebx, eax
    lea     ebx, [edx-1+ebx*8+(432+16-128)/8+(3+2)*8]    ; PosSlot
    ; BitTree
    push    edx
.5: lea     esi, [edx+ebx*8]
    call    _rc_bit
    adc     edx, edx
    mov     ecx, edx
    sub     ecx, 1<<6
    jb      .5
    pop     ebx    ; ebx = 1
_case_model:
    cmp     ecx, 4
    jb      .9
    mov     esi, ebx
    shr     ecx, 1
    rcl     ebx, cl
    dec     ecx
    not     dl    ; 256-edx-1
    mov     dh, 2
    add     edx, ebx
;   lea     edx, [edx+ebx+688+16+64-256*3]    ; SpecPos
    cmp     ecx, 6
    jb      .4
.1: dec     ecx
    call    _rc_norm
    shr     Range, 1
    mov     edx, Range
    cmp     Code, edx
    jb      .3
    sub     Code, edx
    bts     ebx, ecx
.3: cmp     ecx, 4
    jne     .1
    cdq        ; Align
.4:
.5: push    esi
    add     esi, edx
    call    _rc_bit
    pop     esi
    adc     esi, esi
    loop    .5
.6: adc     ecx, ecx
    shr     esi, 1
    jne     .6
    add     ecx, ebx
.9: inc     ecx
    mov     _rep0, ecx
    je      _end
_copy:
    pop     edx
.1: mov     eax, Dest
    sub     eax, _rep0
    movzx   ebx, byte [eax]
.2: mov     eax, Dest
    mov     [eax], bl    ; Dict + Pos
    inc     Dest
    dec     edx
    jns     .1
    jmp     _loop
_end:
    leave
_code_end:
    ret