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
#ifdef _WIN32
#if defined(_WIN64) && defined(_MSC_VER)
#error "64bit target on Windows is not supported with the Microsoft compiler; please use gcc or other non-Microsoft compilers"
#endif
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
    void *ptr_imagebase;            // pointer to data
    void *ptr_alloc;                // pointer to function
    void *ptr_alloc_zeroed;         // pointer to function
    void *ptr_dealloc;              // pointer to function
    void *ptr_realloc;              // pointer to function
    void *ptr_exit;                 // pointer to function
    void *ptr_read_stdio;           // pointer to function
    void *ptr_write_stdio;          // pointer to function
    void *ptr_alloc_rwx;            // pointer to function
} SERVICE_FUNCTIONS;

#pragma pack(pop)

void *svc_alloc(size_t size) {
    return malloc(size);
}
void *svc_alloc_zeroed(size_t size) {
    return calloc(1, size);
}
void svc_free(void *ptr) {
    free(ptr);
}
void *svc_realloc(void* memblock, size_t size) {
    return realloc(memblock, size);
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
    return (void *) VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
    void *ret = (void *) mmap(NULL, size, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    return (ret == MAP_FAILED) ? NULL : ret;
#endif
}

SERVICE_FUNCTIONS g_sf;
typedef void * (*stub_ptr)(void *, void *, size_t);

const char *stub_base85 = $$$$stub_base85$$$$;
char binary_base85[][4096] = $$$$binary_base85$$$$;
const size_t entrypoint_offset = $$$$entrypoint_offset$$$$;

int main() {
    g_sf.ptr_imagebase      = NULL;
    g_sf.ptr_alloc          = (void *) svc_alloc;
    g_sf.ptr_alloc_zeroed   = (void *) svc_alloc_zeroed;
    g_sf.ptr_dealloc        = (void *) svc_free;
    g_sf.ptr_realloc        = (void *) svc_realloc;
    g_sf.ptr_exit           = (void *) svc_exit;
    g_sf.ptr_read_stdio     = (void *) svc_read_stdio;
    g_sf.ptr_write_stdio    = (void *) svc_write_stdio;
    g_sf.ptr_alloc_rwx      = (void *) svc_alloc_rwx;

    stub_ptr stub = (stub_ptr) svc_alloc_rwx(0x1000);
    b85tobin((void *) stub, stub_base85);
    b85tobin(binary_base85, (char const *)binary_base85);
    stub(&g_sf, binary_base85, entrypoint_offset);
    return 0; // never reached
}
//==============================================================================
// LOADER END
//==============================================================================