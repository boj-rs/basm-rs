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
// Code adapted from:
//     https://github.com/kkamagui/mint64os/blob/master/02.Kernel64/Source/Loader.c
//     https://github.com/rafagafe/base85/blob/master/base85.c
//==============================================================================

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#include <stdio.h>
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
#else // 32-bit machine, Windows or Linux or OS X
#define BASMCALL
#endif


////////////////////////////////////////////////////////////////////////////////
//
// Base85 decoder
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

BASMCALL void *svc_alloc_rwx(size_t size);
BASMCALL void *svc_alloc(size_t size, size_t align) {
#if defined(_WIN32) || defined(__linux__)
    return svc_alloc_rwx(size);
#else
    return malloc(size);
#endif
}
BASMCALL void *svc_alloc_zeroed(size_t size, size_t align) {
#if defined(_WIN32) || defined(__linux__)
    return svc_alloc_rwx(size);
#else
    return calloc(1, size);
#endif
}
BASMCALL void svc_free(void *ptr, size_t size, size_t align) {
#if defined(_WIN32)
    VirtualFree(ptr, 0, MEM_RELEASE);
#elif defined(__linux__)
    syscall(11, ptr, size);
#else
    free(ptr);
#endif
}
BASMCALL void *svc_realloc(void* memblock, size_t old_size, size_t old_align, size_t new_size) {
#if defined(_WIN32) || defined(__linux__)
    // this won't be called
    return NULL;
#else    
    return realloc(memblock, new_size);
#endif
}
BASMCALL void svc_exit(size_t status) {
#if defined(_WIN32)
    ExitProcess((UINT) status);
#elif defined(__linux__)
    _exit((int) status);
#else
    exit((int) status);
#endif
}
BASMCALL size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 0) return 0;
#if defined(_WIN32)
    return _read(_fileno(stdin), buf, (unsigned int) count);
#elif defined(__linux__)
    return (size_t)syscall(0, fd, buf, count);
#else
    return fread(buf, 1, count, stdin);
#endif
}
BASMCALL size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 1 && fd != 2) return 0;
#if defined(_WIN32)
    return _write(fd == 1 ? _fileno(stdout) : _fileno(stderr), buf, (unsigned int) count);
#elif defined(__linux__)
    return (size_t)syscall(1, fd, buf, count);
#else
    return fwrite(buf, 1, count, (fd == 1) ? stdout : stderr);
#endif
}

static uint32_t g_debug = 0;
#ifdef _WIN64
static size_t g_debug_base = 0x920000000ULL;
#else
static size_t g_debug_base = 0x20000000ULL;
#endif
BASMCALL void *svc_alloc_rwx(size_t size) {
    static int run_count = 0;
    if (run_count == 1 && g_debug) {
        run_count++;
#ifdef _WIN32
        return (void *) VirtualAlloc((LPVOID) g_debug_base, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
        void *ret = (void *) syscall(9, g_debug_base, size, 0x7, 0x22, -1, 0);
        return (ret == (void *)-1) ? NULL : ret;
#endif
    } else {
        if (run_count < 2) run_count++;
#ifdef _WIN32
        return (void *) VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
        void *ret = (void *) syscall(9, NULL, size, 0x7, 0x22, -1, 0);
        return (ret == (void *)-1) ? NULL : ret;
#endif
    }
}

PLATFORM_DATA g_pd;
SERVICE_FUNCTIONS g_sf;
typedef void * (BASMCALL *stub_ptr)(void *, void *, size_t, size_t);

const char *stub_base85 = $$$$stub_base85$$$$;
char binary_base85[][4096] = $$$$binary_base85$$$$;
const size_t entrypoint_offset = $$$$entrypoint_offset$$$$;

#if defined(__linux__)
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
    if (sizeof(size_t) < 8) {
#ifdef DEBUG
        printf("Error: sizeof(size_t) = %zu for amd64\n", sizeof(size_t));
#endif
        svc_exit(1);
    }
    if (argc >= 2 &&
        argv[1][0] == '-' && argv[1][1] == '-' && argv[1][2] == 'd' && argv[1][3] == 'e' &&
        argv[1][4] == 'b' && argv[1][5] == 'u' && argv[1][6] == 'g' && argv[1][7] == '\0') {
        g_debug = 1;
    }
    g_pd.env_flags          = 0; // not strictly necessary but for clarity
#if defined(_WIN32)
    g_pd.env_id             = ENV_ID_WINDOWS;
#elif defined(__linux__)
    g_pd.env_id             = ENV_ID_LINUX;
    // Linux's stack growth works differently than Windows.
    // However, we do make sure the stack grows since we cannot rely on
    //   Microsoft compiler's behavior on the stack usage.
    g_pd.env_flags          |= ENV_FLAGS_LINUX_STYLE_CHKSTK;
#else
    g_pd.env_id             = ENV_ID_UNKNOWN;
#endif
    g_pd.pe_image_base      = $$$$pe_image_base$$$$ULL;
    g_pd.pe_off_reloc       = $$$$pe_off_reloc$$$$ULL;
    g_pd.pe_size_reloc      = $$$$pe_size_reloc$$$$ULL;
#if defined(_WIN32)
    g_pd.win_GetModuleHandleW   = (uint64_t) GetModuleHandleW;
    g_pd.win_GetProcAddress     = (uint64_t) GetProcAddress;
#endif
    g_sf.ptr_imagebase      = NULL;
    g_sf.ptr_alloc          = (void *) svc_alloc;
    g_sf.ptr_alloc_zeroed   = (void *) svc_alloc_zeroed;
    g_sf.ptr_dealloc        = (void *) svc_free;
    g_sf.ptr_realloc        = (void *) svc_realloc;
    g_sf.ptr_exit           = (void *) svc_exit;
    g_sf.ptr_read_stdio     = (void *) svc_read_stdio;
    g_sf.ptr_write_stdio    = (void *) svc_write_stdio;
    g_sf.ptr_alloc_rwx      = (void *) svc_alloc_rwx;
    g_sf.ptr_platform       = (void *) &g_pd;

    stub_ptr stub = (stub_ptr) svc_alloc_rwx(0x1000);
    b85tobin((void *) stub, stub_base85);
    b85tobin(binary_base85, (char const *)binary_base85);

    stub(&g_sf, binary_base85, entrypoint_offset, (size_t) g_debug);
    return 0; // never reached
}
//==============================================================================
// LOADER END
//==============================================================================