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

BASMCALL void *svc_alloc(size_t size) {
    return malloc(size);
}
BASMCALL void *svc_alloc_zeroed(size_t size) {
    return calloc(1, size);
}
BASMCALL void svc_free(void *ptr) {
    free(ptr);
}
BASMCALL void *svc_realloc(void* memblock, size_t size) {
    return realloc(memblock, size);
}
BASMCALL void svc_exit(size_t status) {
    exit((int) status);
}
BASMCALL size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 0) return 0;
    return fread(buf, 1, count, stdin);
}
BASMCALL size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 1 && fd != 2) return 0;
    return fwrite(buf, 1, count, (fd == 1) ? stdout : stderr);
}
static uint32_t g_debug = 0;
BASMCALL void *svc_alloc_rwx(size_t size) {
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

PLATFORM_DATA g_pd;
SERVICE_FUNCTIONS g_sf;
typedef void * (BASMCALL *stub_ptr)(void *, void *, size_t, size_t);

const char *stub_base85 = $$$$stub_base85$$$$;
char binary_base85[][4096] = $$$$binary_base85$$$$;
const size_t entrypoint_offset = $$$$entrypoint_offset$$$$;

int main(int argc, char *argv[]) {
    if (argc >= 2 && !strcmp("--debug", argv[1])) {
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