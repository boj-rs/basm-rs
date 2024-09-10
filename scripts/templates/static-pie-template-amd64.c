// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

// SOLUTION BEGIN
$$$$solution_src$$$$
// SOLUTION END

// LOADER BEGIN
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#include <io.h>
#elif defined(__linux__)
#include <unistd.h>
#ifndef MAP_ANONYMOUS
#define MAP_ANONYMOUS 0x20
#endif
#else
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#endif
#ifdef DEBUG
#include <stdio.h>
#endif

#ifndef UINT32_MAX
typedef unsigned char uint8_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
#endif

// Use cdecl on x86 (32bit), Microsoft x64 calling convention on amd64 (64bit)
#if defined(__LP64__) // LP64 machine, OS X or Linux
#define BASMCALL __attribute__((ms_abi))
#elif defined(_WIN64) // LLP64 machine, Windows
#if defined(_MSC_VER)
#define BASMCALL
#else
#define BASMCALL __attribute__((ms_abi))
#endif
#else // 32-bit machine, Windows or Linux or OS X -> forbid compilation
#error "The current file can only be compiled for amd64."
#define BASMCALL
#endif

// Base85 decoder. Code adapted from:
//     https://github.com/rafagafe/base85/blob/master/base85.c
const char *b85 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>\?@^_`{|}~";
void b85tobin(void *dest, char const *src) {
    uint32_t *p = (uint32_t *)dest;
    uint8_t digittobin[256];
    for (uint8_t i=0; i<85; i++) digittobin[(uint8_t)b85[i]] = i;
    while (1) {
        while (*src == '\0') src++;
        if (*src == ']') break;
        uint32_t value = 0;
        for (uint32_t i=0; i<5; i++) {
            value *= 85;
            value += digittobin[(uint8_t)*src++];
        }
        *p++ = (value >> 24) | ((value >> 8) & 0xff00) | ((value << 8) & 0xff0000) | (value << 24);
    }
}

#pragma pack(push, 1)
typedef struct {
    uint64_t    env_id;
    uint64_t    env_flags;
    uint64_t    win_kernel32;       // handle of kernel32.dll
    uint64_t    win_GetProcAddress; // pointer to kernel32!GetProcAddress
    void       *ptr_alloc_rwx;      // pointer to function
    void       *ptr_alloc;          // pointer to function
    void       *ptr_alloc_zeroed;   // pointer to function
    void       *ptr_dealloc;        // pointer to function
    void       *ptr_realloc;        // pointer to function
    void       *ptr_read_stdio;     // pointer to function
    void       *ptr_write_stdio;    // pointer to function
} PLATFORM_DATA;
#pragma pack(pop)

#define ENV_ID_UNKNOWN              0
#define ENV_ID_WINDOWS              1
#define ENV_ID_LINUX                2
#define ENV_ID_WASM                 3
#define ENV_ID_MACOS                4
#define ENV_FLAGS_LINUX_STYLE_CHKSTK    0x0001  // disables __chkstk in binaries compiled with Windows target
#define ENV_FLAGS_NATIVE                0x0002  // indicates the binary is running without the loader
#define ENV_FLAGS_NO_EXIT               0x0004  // do not call SYS_exitgroup on Linux (support fn-impl scenarios)

#if !defined(_WIN32) && !defined(__linux__)
BASMCALL void *svc_alloc(size_t size, size_t align) {
    return malloc(size);
}
BASMCALL void *svc_alloc_zeroed(size_t size, size_t align) {
    return calloc(1, size);
}
BASMCALL void svc_free(void *ptr, size_t size, size_t align) {
    free(ptr);
}
BASMCALL void *svc_realloc(void* memblock, size_t old_size, size_t old_align, size_t new_size) {
    // This won't be called in loader stub.
    // Also, the main executable will directly call OS APIs/syscalls
    return realloc(memblock, new_size);
}
BASMCALL size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 0) return 0;
    return fread(buf, 1, count, stdin);
}
BASMCALL size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 1 && fd != 2) return 0;
    return fwrite(buf, 1, count, (fd == 1) ? stdout : stderr);
}
#endif

BASMCALL void *svc_alloc_rwx(size_t size) {
#ifdef _WIN32
    size_t ret = (size_t) VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
    size_t ret = (size_t) syscall(9, NULL, size, 0x7, 0x22, -1, 0);
    if (ret == (size_t)-1) ret = 0;
#endif
    return (void *) ret;
}

typedef int (BASMCALL *stub_ptr)(void *, void *);

#define STUB_RAW $$$$stub_raw$$$$
#if defined(__GNUC__)
__attribute__ ((section (".text#"))) const char stub_raw[] = STUB_RAW;
stub_ptr get_stub() {
    return (stub_ptr) stub_raw;
}
#else
const char stub_raw[] = STUB_RAW;
stub_ptr get_stub() {
    char *stub = (char *) svc_alloc_rwx(4096);
    for (size_t i = 0; i < sizeof(stub_raw); i++) stub[i] = stub_raw[i];
    return (stub_ptr) stub;
}
#endif
char payload[][$$$$min_len_4096$$$$] = $$$$binary_base85$$$$;

#if defined(__linux__) && (defined(BOJ) || defined(BASM_CI))
int main() {}
#ifdef __cplusplus
extern "C"
#endif
int __libc_start_main(
    void *func_ptr,
    int argc,
    char* argv[],
    void (*init_func)(void),
    void (*fini_func)(void),
    void (*rtld_fini_func)(void),
    void *stack_end) {
#else
int main(int argc, char *argv[]) {
#endif
    PLATFORM_DATA pd;
    if (sizeof(size_t) != 8) {
        // Cannot run amd64 binaries on non-64bit environment
        return 1;
    }
    pd.env_flags            = 0; // necessary since pd is on stack
#if defined(_WIN32)
    pd.env_id               = ENV_ID_WINDOWS;
#elif defined(__linux__)
    pd.env_id               = ENV_ID_LINUX;
    // Linux's stack growth works differently than Windows.
    // Hence, we disable the __chkstk mechanism on Linux.
    pd.env_flags            |= ENV_FLAGS_LINUX_STYLE_CHKSTK;
#else
    pd.env_id               = ENV_ID_UNKNOWN;
#endif
#if defined(_WIN32)
    pd.win_kernel32         = (uint64_t) GetModuleHandleW(L"kernel32");
    pd.win_GetProcAddress   = (uint64_t) GetProcAddress;
#endif
    pd.ptr_alloc_rwx        = (void *) svc_alloc_rwx;
#if !defined(_WIN32) && !defined(__linux__)
    pd.ptr_alloc            = (void *) svc_alloc;
    pd.ptr_alloc_zeroed     = (void *) svc_alloc_zeroed;
    pd.ptr_dealloc          = (void *) svc_free;
    pd.ptr_realloc          = (void *) svc_realloc;
    pd.ptr_read_stdio       = (void *) svc_read_stdio;
    pd.ptr_write_stdio      = (void *) svc_write_stdio;
#endif

    stub_ptr stub = get_stub();
#if defined(__linux__)
    uint8_t stubbuf[68 + $$$$stub_len$$$$] = "QMd~L002n8@6D@;XGJ3cz5oya01pLO>naZmS5~+Q0000n|450>x(5IN07=KfA^-pYO)<bp|Hw@-$qxlyU&9Xz]";
    b85tobin(stubbuf, (char const *)stubbuf);
    /* prepend thunk and relocate stub onto stack */
    for (size_t i = 0; i < $$$$stub_len$$$$; i++) stubbuf[68 + i] = (uint8_t)stub_raw[i];
    size_t base = ((size_t)stub_raw) & 0xFFFFFFFFFFFFF000ULL; // page-aligned pointer to munmap in thunk
    size_t len = (((size_t)stub_raw) + sizeof(stub_raw)) - base;
    len = ((len + 0xFFF) >> 12) << 12;
    *(uint64_t *)(stubbuf + 0x08) = (uint64_t) base;
    *(uint32_t *)(stubbuf + 0x11) = (uint32_t) len;
    base = ((size_t)stubbuf) & 0xFFFFFFFFFFFFF000ULL;
    len = (((size_t)stubbuf) + 68 + $$$$stub_len$$$$) - base;
    len = ((len + 0xFFF) >> 12) << 12;
    syscall(10, base, len, 0x7); // mprotect: make the stub on stack executable
    pd.ptr_alloc_rwx = (void *) (stubbuf + 0x1c); // thunk implements its own svc_alloc_rwx
    stub = (stub_ptr) stubbuf;
#endif
    b85tobin(payload, (char const *)payload);
    return stub(&pd, payload);
}
// LOADER END