// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

// SOLUTION BEGIN
$$$$solution_src$$$$
// SOLUTION END

// LOADER BEGIN
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#else
#include <sys/mman.h>
#ifndef MAP_ANONYMOUS
#define MAP_ANONYMOUS 0x20
#endif
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

void *svc_alloc(size_t size, size_t align) {
    return malloc(size);
}
void *svc_alloc_zeroed(size_t size, size_t align) {
    return calloc(1, size);
}
void svc_free(void *ptr, size_t size, size_t align) {
    free(ptr);
}
void *svc_realloc(void* memblock, size_t old_size, size_t old_align, size_t new_size) {
    return realloc(memblock, new_size);
}
void svc_exit(size_t status) {
    exit((int) status);
}
size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 0) return 0;
    return fread(buf, 1, count, stdin);
}
size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 1 && fd != 2) return 0;
    return fwrite(buf, 1, count, (fd == 1) ? stdout : stderr);
}
void *svc_alloc_rwx(size_t size) {
#ifdef _WIN32
    size_t ret = (size_t) VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
    size_t ret = (size_t) mmap(NULL, size, 0x7, 0x22, -1, 0);
    if (ret == (size_t)-1) ret = 0;
#endif
    return (void *) ret;
}

typedef int (*stub_ptr)(void *, void *);

const char *stub_base85 = $$$$stub_base85$$$$;
char payload[][$$$$min_len_4096$$$$] = $$$$binary_base85$$$$;

int main(int argc, char *argv[]) {
    PLATFORM_DATA pd;
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
    pd.ptr_alloc            = (void *) svc_alloc;
    pd.ptr_alloc_zeroed     = (void *) svc_alloc_zeroed;
    pd.ptr_dealloc          = (void *) svc_free;
    pd.ptr_realloc          = (void *) svc_realloc;
    pd.ptr_read_stdio       = (void *) svc_read_stdio;
    pd.ptr_write_stdio      = (void *) svc_write_stdio;

    stub_ptr stub = (stub_ptr) svc_alloc_rwx(4096);
    b85tobin((void *) stub, stub_base85);
    b85tobin(payload, (char const *)payload);
    return stub(&pd, payload);
}
// LOADER END