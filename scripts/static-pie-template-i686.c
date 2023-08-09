// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box ☆ https://doc.rust-lang.org/book/

//==============================================================================
// SOLUTION BEGIN
//==============================================================================
$$$$solution_src$$$$
//==============================================================================
// SOLUTION END
//==============================================================================

//==============================================================================
// LOADER BEGIN
//==============================================================================

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


////////////////////////////////////////////////////////////////////////////////
//
// Base85 decoder. Code adapted from:
//     https://github.com/rafagafe/base85/blob/master/base85.c
//
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
//
// Service functions
//
////////////////////////////////////////////////////////////////////////////////


#pragma pack(push, 1)

typedef struct {
    uint64_t    env_id;
    uint64_t    env_flags;
    uint64_t    pe_image_base;
    uint64_t    pe_off_reloc;
    uint64_t    pe_size_reloc;
    uint64_t    win_GetModuleHandleW;   // pointer to kernel32::GetModuleHandleW
    uint64_t    win_GetProcAddress;     // pointer to kernel32::GetProcAddress
} PLATFORM_DATA;

typedef struct {
    void *ptr_imagebase;            // pointer to data
    void *ptr_alloc;                // pointer to function
    void *ptr_alloc_zeroed;         // pointer to function
    void *ptr_dealloc;              // pointer to function
    void *ptr_realloc;              // pointer to function
    void *ptr_exit;                 // pointer to function
    void *ptr_read_stdio;           // pointer to function
    void *ptr_write_stdio;          // pointer to function
    void *ptr_alloc_rwx;            // pointer to function
    void *ptr_platform;             // pointer to data
} SERVICE_FUNCTIONS;

#pragma pack(pop)

#define ENV_ID_UNKNOWN              0
#define ENV_ID_WINDOWS              1
#define ENV_ID_LINUX                2
#define ENV_FLAGS_LINUX_STYLE_CHKSTK    0x0001

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
static uint32_t g_debug = 0;
void *svc_alloc_rwx(size_t size) {
    static int run_count = 0;
    if (run_count == 1 && g_debug) {
        run_count++;
#ifdef _WIN32
        return (void *) VirtualAlloc((LPVOID) 0x20000000, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
        void *ret = (void *) mmap((void *) 0x20000000, size, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
        return (ret == MAP_FAILED) ? NULL : ret;
#endif
    } else {
        if (run_count < 2) run_count++;
#ifdef _WIN32
        return (void *) VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
        void *ret = (void *) mmap(NULL, size, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        return (ret == MAP_FAILED) ? NULL : ret;
#endif
    }
}

typedef void * (*stub_ptr)(void *, void *, size_t, size_t);

const char *stub_base85 = $$$$stub_base85$$$$;
char binary_base85[][$$$$min_len_4096$$$$] = $$$$binary_base85$$$$;
const size_t entrypoint_offset = $$$$entrypoint_offset$$$$;

int main(int argc, char *argv[]) {
    PLATFORM_DATA pd;
    SERVICE_FUNCTIONS sf;
    if (argc >= 2 && !strcmp("--debug", argv[1])) {
        g_debug = 1;
    }
    pd.env_flags            = 0; // not strictly necessary but for clarity
#if defined(_WIN32)
    pd.env_id               = ENV_ID_WINDOWS;
#elif defined(__linux__)
    pd.env_id               = ENV_ID_LINUX;
    // Linux's stack growth works differently than Windows.
    // However, we do make sure the stack grows since we cannot rely on
    //   Microsoft compiler's behavior on the stack usage.
    pd.env_flags            |= ENV_FLAGS_LINUX_STYLE_CHKSTK;
#else
    pd.env_id               = ENV_ID_UNKNOWN;
#endif
    pd.pe_image_base        = $$$$pe_image_base$$$$ULL;
    pd.pe_off_reloc         = $$$$pe_off_reloc$$$$ULL;
    pd.pe_size_reloc        = $$$$pe_size_reloc$$$$ULL;
#if defined(_WIN32)
    pd.win_GetModuleHandleW = (uint64_t) GetModuleHandleW;
    pd.win_GetProcAddress   = (uint64_t) GetProcAddress;
#endif
    sf.ptr_imagebase        = NULL;
    sf.ptr_alloc            = (void *) svc_alloc;
    sf.ptr_alloc_zeroed     = (void *) svc_alloc_zeroed;
    sf.ptr_dealloc          = (void *) svc_free;
    sf.ptr_realloc          = (void *) svc_realloc;
    sf.ptr_exit             = (void *) svc_exit;
    sf.ptr_read_stdio       = (void *) svc_read_stdio;
    sf.ptr_write_stdio      = (void *) svc_write_stdio;
    sf.ptr_alloc_rwx        = (void *) svc_alloc_rwx;
    sf.ptr_platform         = (void *) &pd;

    stub_ptr stub = (stub_ptr) svc_alloc_rwx(0x1000);
    b85tobin((void *) stub, stub_base85);
    b85tobin(binary_base85, (char const *)binary_base85);
    stub(&sf, binary_base85, entrypoint_offset, (size_t) g_debug);
    return 0; // never reached
}
//==============================================================================
// LOADER END
//==============================================================================