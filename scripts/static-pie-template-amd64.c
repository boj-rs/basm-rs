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
#else // 32-bit machine, Windows or Linux or OS X
#define BASMCALL
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
    uint64_t    leading_unused_bytes;
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
#endif

static uint32_t g_debug = 0;
#ifdef _WIN64
static const size_t g_debug_base = 0x920000000ULL;
#else
static const size_t g_debug_base = 0x20000000ULL;
#endif
BASMCALL void *svc_alloc_rwx(size_t size) {
    size_t preferred_addr = 0;
    size_t off = 0;
    if (!(size >> 63) && g_debug) {
        preferred_addr = g_debug_base;
        off = $$$$leading_unused_bytes$$$$;
        size += off;
    }
    size &= (1ULL << 63) - 1;
#ifdef _WIN32
    size_t ret = (size_t) VirtualAlloc((LPVOID) preferred_addr, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
    size_t ret = (size_t) syscall(9, preferred_addr, size, 0x7, 0x22, -1, 0);
    if (ret == (size_t)-1) ret = 0;
#endif
    return (void *) (!ret ? ret : ret + off);
}

typedef void * (BASMCALL *stub_ptr)(void *, void *, size_t, size_t);

#define STUB_RAW $$$$stub_raw$$$$
#define STUB_LEN $$$$stub_len$$$$
#if defined(__GNUC__)
__attribute__ ((section (".text#"))) const char stub_raw[] = STUB_RAW;
stub_ptr get_stub() {
    return (stub_ptr) stub_raw;
}
#else
const char stub_raw[] = STUB_RAW;
stub_ptr get_stub() {
    char *stub = (char *) svc_alloc_rwx((1ULL << 63) | 0x1000);
    for (size_t i = 0; i < sizeof(stub_raw); i++) stub[i] = stub_raw[i];
    return (stub_ptr) stub;
}
#endif
char binary_base85[][$$$$min_len_4096$$$$] = $$$$binary_base85$$$$;
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
    PLATFORM_DATA pd;
    SERVICE_FUNCTIONS sf;
    if (sizeof(size_t) != 8) {
#ifdef DEBUG
        printf("Error: sizeof(size_t) = %zu for amd64\n", sizeof(size_t));
#endif
        return 1;
    }
    if (argc >= 2 &&
        argv[1][0] == '-' && argv[1][1] == '-' && argv[1][2] == 'd' && argv[1][3] == 'e' &&
        argv[1][4] == 'b' && argv[1][5] == 'u' && argv[1][6] == 'g' && argv[1][7] == '\0') {
        g_debug = 1;
    }
    pd.env_flags            = 0; // necessary since pd is on stack
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
    pd.leading_unused_bytes = $$$$leading_unused_bytes$$$$ULL;
    pd.pe_image_base        = $$$$pe_image_base$$$$ULL;
    pd.pe_off_reloc         = $$$$pe_off_reloc$$$$ULL;
    pd.pe_size_reloc        = $$$$pe_size_reloc$$$$ULL;
#if defined(_WIN32)
    pd.win_GetModuleHandleW = (uint64_t) GetModuleHandleW;
    pd.win_GetProcAddress   = (uint64_t) GetProcAddress;
#endif
    sf.ptr_imagebase        = NULL;
#if !defined(_WIN32) && !defined(__linux__)
    sf.ptr_alloc            = (void *) svc_alloc;
    sf.ptr_alloc_zeroed     = (void *) svc_alloc_zeroed;
    sf.ptr_dealloc          = (void *) svc_free;
    sf.ptr_realloc          = (void *) svc_realloc;
    sf.ptr_exit             = (void *) svc_exit;
    sf.ptr_read_stdio       = (void *) svc_read_stdio;
    sf.ptr_write_stdio      = (void *) svc_write_stdio;
#endif
    sf.ptr_alloc_rwx        = (void *) svc_alloc_rwx;
    sf.ptr_platform         = (void *) &pd;

    b85tobin(binary_base85, (char const *)binary_base85);

    stub_ptr stub = get_stub();
#if defined(__linux__)
    uint8_t stubbuf[68 + STUB_LEN] = { 0x51, 0xB8, 0x0B, 0x00, 0x00, 0x00, 0x48, 0xBF, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01, 0xBE, 0x00, 0x10, 0x00, 0x00, 0x0F, 0x05, 0x59, 0xEB, 0x2A, 0x0F, 0x0B, 0x57, 0x56, 0xB8, 0x09, 0x00, 0x00, 0x00, 0x31, 0xFF, 0x48, 0x89, 0xCE, 0xBA, 0x07, 0x00, 0x00, 0x00, 0x49, 0xC7, 0xC2, 0x22, 0x00, 0x00, 0x00, 0x4D, 0x31, 0xC0, 0x49, 0xFF, 0xC8, 0x4D, 0x31, 0xC9, 0x0F, 0x05, 0x5E, 0x5F, 0xC3, 0x0F, 0x0B, };
    if (!g_debug) {
        /* prepend thunk and relocate stub onto stack */
        for (size_t i = 0; i < STUB_LEN; i++) stubbuf[68 + i] = (uint8_t)stub_raw[i];
        size_t base = ((size_t)stub_raw) & 0xFFFFFFFFFFFFF000ULL; // page-aligned pointer to munmap in thunk
        size_t len = (((size_t)stub_raw) + sizeof(stub_raw)) - base;
        len = ((len + 0xFFF) >> 12) << 12;
        *(uint64_t *)(stubbuf + 0x08) = (uint64_t) base;
        *(uint32_t *)(stubbuf + 0x11) = (uint32_t) len;
        base = ((size_t)stubbuf) & 0xFFFFFFFFFFFFF000ULL;
        len = (((size_t)stubbuf) + 68 + STUB_LEN) - base;
        len = ((len + 0xFFF) >> 12) << 12;
        syscall(10, base, len, 0x7); // mprotect: make the stub on stack executable
        sf.ptr_alloc_rwx = (void *) (stubbuf + 0x1c); // thunk implements its own svc_alloc_rwx
        stub = (stub_ptr) stubbuf;
    }
#endif
    stub(&sf, binary_base85, entrypoint_offset, (size_t) g_debug);
    return 0; // never reached
}
//==============================================================================
// LOADER END
//==============================================================================