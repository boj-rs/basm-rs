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

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#include <io.h>
#else
#include <unistd.h>
#include <sys/mman.h>
#endif


////////////////////////////////////////////////////////////////////////////////
//
// Base85 decoder
//
////////////////////////////////////////////////////////////////////////////////

/** Escape values. */
enum escape {
    notadigit = 85u /**< Return value when a non-digit-base-85 is found. */
};

/** Lookup table to convert a base 85 digit in a binary number. */
static unsigned char const digittobin[] = {
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 62, 85, 63, 64, 65, 66, 85, 67, 68, 69, 70, 85, 71, 85, 85,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 85, 72, 73, 74, 75, 76,
    77, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 85, 85, 85, 78, 79,
    80, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 81, 82, 83, 84, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
    85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
};

/* Some powers of 85. */
#define p850 1ul           /*< 85^0 */
#define p851 85ul          /*< 85^1 */
#define p852 (p851 * p851) /*< 85^2 */
#define p853 (p851 * p852) /*< 85^3 */
#define p854 (p851 * p853) /*< 85^4 */


/** Powers of 85 list. */
static unsigned long const pow85[] = { p854, p853, p852, p851, p850 };

/** Helper constant to get the endianness of the running machine. */
static short const endianness = 1;

/** Points to 1 if little-endian or points to 0 if big-endian. */
static char const* const littleEndian = (char const*)&endianness;

/** Copy a unsigned long in a big-endian array of 4 bytes.
  * @param dest Destination memory block.
  * @param value Value to copy.
  * @return  dest + 4 */
static uint8_t* ultobe( uint8_t* dest, uint32_t value ) {

    uint8_t* const d = (uint8_t*)dest;
    uint8_t const* const s = (uint8_t*)&value;

    for( int i = 0; i < 4; ++i )
        d[ i ] = s[ *littleEndian ? 3 - i : i ];

    return d + 4;
}

/* Convert a base85 string to binary format. */
uint8_t* b85tobin( uint8_t* dest, char const* src ) {

    for( char const* s = (char const*)src;; ) {

        uint32_t value = 0;
        for( uint32_t i = 0; i < sizeof pow85 / sizeof *pow85; ++i, ++s ) {
            uint32_t bin = digittobin[ (int) *s ];
            if ( bin == notadigit )
                return dest;
            value += bin * pow85[ i ];
        }

        dest = ultobe( dest, value );
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
#ifdef _WIN32
size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    int fd_os;
    switch (fd) {
    case 0: fd_os = _fileno(stdin); break;
    case 1: fd_os = _fileno(stdout); break;
    case 2: fd_os = _fileno(stderr); break;
    default: return 0; // we only support stdin(=0), stdout(=1), stderr(=2)
    }
    return _read(fd_os, buf, count);
}
size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    int fd_os;
    switch (fd) {
    case 0: fd_os = _fileno(stdin); break;
    case 1: fd_os = _fileno(stdout); break;
    case 2: fd_os = _fileno(stderr); break;
    default: return 0; // we only support stdin(=0), stdout(=1), stderr(=2)
    }
    return _write(fd_os, buf, count);
}
#else
size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    int fd_os;
    switch (fd) {
    case 0: fd_os = STDIN_FILENO; break;
    case 1: fd_os = STDOUT_FILENO; break;
    case 2: fd_os = STDERR_FILENO; break;
    default: return 0; // we only support stdin(=0), stdout(=1), stderr(=2)
    }
    return read(fd_os, buf, count);
}
size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    int fd_os;
    switch (fd) {
    case 0: fd_os = STDIN_FILENO; break;
    case 1: fd_os = STDOUT_FILENO; break;
    case 2: fd_os = STDERR_FILENO; break;
    default: return 0; // we only support stdin(=0), stdout(=1), stderr(=2)
    }
    return write(fd_os, buf, count);
}
#endif

SERVICE_FUNCTIONS g_sf;
typedef void (*entry_ptr)(void *);

const char *ELF_binary_base85[] = $$$$binary_base85$$$$; // ELF linked as a static PIE encoded as base85 (PIE: position independent executable)
uint8_t ELF_binary[ $$$$len$$$$ ];
int ELF_binary_len = $$$$len$$$$;
uint32_t qwEntryPointOffset = $$$$entrypoint_offset$$$$;

int main() {
    uint8_t *qwLoadedAddress;
    uint64_t qwMemorySize;
    uint64_t qwEntryPointAddress;

    qwMemorySize = (ELF_binary_len + 0x1000 - 1) & 0xfffff000;

    // 응용프로그램에서 사용할 메모리를 할당
    // VirtualAlloc 및 mmap은 항상 page-aligned address를 반환함
#ifdef _WIN32
    qwLoadedAddress = ( uint8_t * ) VirtualAlloc(NULL, qwMemorySize, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if ( qwLoadedAddress == NULL )
    {
        return 1;
    }
#else
    qwLoadedAddress = ( uint8_t * ) mmap(NULL, qwMemorySize,
        PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if( qwLoadedAddress == MAP_FAILED )
    {
        return 1;
    }
#endif

    uint8_t *dest = qwLoadedAddress;
    for (size_t p = 0; p < sizeof(ELF_binary_base85)/sizeof(ELF_binary_base85[0]); p++) {
        dest = b85tobin(dest, ELF_binary_base85[p]);
    }

    g_sf.ptr_imagebase    = (void *) qwLoadedAddress;
    g_sf.ptr_alloc        = (void *) svc_alloc;
    g_sf.ptr_alloc_zeroed = (void *) svc_alloc_zeroed;
    g_sf.ptr_dealloc      = (void *) svc_free;
    g_sf.ptr_realloc      = (void *) svc_realloc;
    g_sf.ptr_exit         = (void *) svc_exit;
    g_sf.ptr_read_stdio   = (void *) svc_read_stdio;
    g_sf.ptr_write_stdio  = (void *) svc_write_stdio;

    ((entry_ptr) (qwLoadedAddress + qwEntryPointOffset))(&g_sf); // call the EntryPoint
    return 0; // should never be reached
}
//==============================================================================
// LOADER END
//==============================================================================