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
#else
#include <unistd.h>
#include <sys/mman.h>
#endif

////////////////////////////////////////////////////////////////////////////////
//
// 매크로
//
////////////////////////////////////////////////////////////////////////////////

#ifndef MAP_ANONYMOUS
#define MAP_ANONYMOUS 0x20
#endif

// 기본 데이터 타입을 정의한 매크로
typedef uint16_t Elf32_Half;
typedef uint32_t Elf32_Off;
typedef uint32_t Elf32_Addr;
typedef uint32_t Elf32_Word;
typedef int32_t  Elf32_Sword;

// e_ident[]의 index 의미
#define EI_MAG0         0 
#define EI_MAG1         1
#define EI_MAG2         2
#define EI_MAG3         3
#define EI_CLASS        4
#define EI_DATA         5
#define EI_VERSION      6
#define EI_OSABI        7
#define EI_ABIVERSION   8
#define EI_PAD          9
#define EI_NIDENT       16

// e_ident[EI_MAGX] 
#define ELFMAG0         0x7F
#define ELFMAG1         'E'
#define ELFMAG2         'L'
#define ELFMAG3         'F'

// e_ident[EI_CLASS]
#define ELFCLASSNONE    0
#define ELFCLASS32      1
#define ELFCLASS64      2

// e_ident[EI_DATA]
#define ELFDATANONE     0 
#define ELFDATA2LSB     1 
#define ELFDATA2MSB     2 

// e_ident[OSABI]
#define ELFOSABI_NONE       0
#define ELFOSABI_HPUX       1
#define ELFOSABI_NETBSD     2
#define ELFOSABI_LINUX      3
#define ELFOSABI_SOLARIS    6
#define ELFOSABI_AIX        7
#define ELFOSABI_FREEBSD    9

// e_type
#define ET_NONE         0
#define ET_REL          1
#define ET_EXEC         2
#define ET_DYN          3
#define ET_CORE         4
#define ET_LOOS         0xFE00
#define ET_HIOS         0xFEFF
#define ET_LOPROC       0xFF00
#define ET_HIPROC       0xFFFF

// e_machine
#define EM_NONE         0
#define EM_M32          1
#define EM_SPARC        2
#define EM_386          3
#define EM_PPC          20
#define EM_PPC64        21
#define EM_ARM          40
#define EM_IA_64        50
#define EM_X86_64       62
#define EM_AVR          83
#define EM_AVR32        185
#define EM_CUDA         190

// 특별한 섹션 인덱스(Special Section Index)
#define SHN_UNDEF       0
#define SHN_LOERSERVE   0xFF00
#define SHN_LOPROC      0xFF00
#define SHN_HIPROC      0xFF1F
#define SHN_LOOS        0xFF20
#define SHN_HIOS        0xFF3F
#define SHN_ABS         0xFFF1
#define SHN_COMMON      0xFFF2
#define SHN_XINDEX      0xFFFF
#define SHN_HIRESERVE   0xFFFF

// sh_type
#define SHT_NULL        0
#define SHT_PROGBITS    1
#define SHT_SYMTAB      2
#define SHT_STRTAB      3
#define SHT_RELA        4
#define SHT_HASH        5
#define SHT_DYNAMIC     6
#define SHT_NOTE        7
#define SHT_NOBITS      8
#define SHT_REL         9
#define SHT_SHLIB       10
#define SHT_DYNSYM      11
#define SHT_LOOS        0x60000000
#define SHT_HIOS        0x6FFFFFFF
#define SHT_LOPROC      0x70000000
#define SHT_HIPROC      0x7FFFFFFF
#define SHT_LOUSER      0x80000000
#define SHT_HIUSER      0xFFFFFFFF

// sh_flags
#define SHF_WRITE       1
#define SHF_ALLOC       2
#define SHF_EXECINSTR   4
#define SHF_MASKOS      0x0FF00000
#define SHF_MASKPROC    0xF0000000

// Special Section Index
#define SHN_UNDEF       0
#define SHN_LORESERVE   0xFF00
#define SHN_LOPROC      0xFF00
#define SHN_HIPROC      0xFF1F
#define SHN_ABS         0xFFF1
#define SHN_COMMON      0xFFF2
#define SHN_HIRESERVE   0xFFFF

////////////////////////////////////////////////////////////////////////////////
//
// 구조체
//
////////////////////////////////////////////////////////////////////////////////
// 1바이트로 정렬
#pragma pack( push, 1 )

// ELF32 파일 포맷의 ELF 헤더 자료구조
typedef struct
{
    unsigned char e_ident[16];      // ELF 식별자(Identification)
    Elf32_Half e_type;              // 오브젝트 파일 형식
    Elf32_Half e_machine;           // 머신(Machine) 타입
    Elf32_Word e_version;           // 오브젝트 파일 버전
    Elf32_Addr e_entry;             // 엔트리 포인트 어드레스
    Elf32_Off e_phoff;              // 파일 내에 존재하는 프로그램 헤더 테이블의 위치
    Elf32_Off e_shoff;              // 파일 내에 존재하는 섹션 헤더 테이블의 위치
    Elf32_Word e_flags;             // 프로세서 의존적인(Processor-specific) 플래그
    Elf32_Half e_ehsize;            // ELF 헤더의 크기
    Elf32_Half e_phentsize;         // 프로그램 헤더 엔트리 한 개의 크기
    Elf32_Half e_phnum;             // 프로그램 헤더 엔트리의 개수
    Elf32_Half e_shentsize;         // 섹션 헤더 엔트리 한 개의 크기
    Elf32_Half e_shnum;             // 섹션 헤더 엔트리의 개수
    Elf32_Half e_shstrndx;          // 섹션 이름 문자열이 저장된 섹션 헤더의 인덱스
} Elf32_Ehdr;

// ELF32의 섹션 헤더 자료구조
typedef struct
{
    Elf32_Word sh_name;             // 섹션 이름이 저장된 오프셋
    Elf32_Word sh_type;             // 섹션 타입
    Elf32_Word sh_flags;           // 섹션 플래그
    Elf32_Addr sh_addr;             // 메모리에 로딩할 어드레스
    Elf32_Off sh_offset;            // 파일 내에 존재하는 섹션의 오프셋
    Elf32_Word sh_size;            // 섹션 크기
    Elf32_Word sh_link;             // 연결된 다른 섹션
    Elf32_Word sh_info;             // 부가적인 정보
    Elf32_Word sh_addralign;       // 어드레스 정렬
    Elf32_Word sh_entsize;         // 섹션에 들어있는 데이터 엔트리의 크기
} Elf32_Shdr;

#pragma pack(pop)


////////////////////////////////////////////////////////////////////////////////
//
// 구현
//
////////////////////////////////////////////////////////////////////////////////

/**
 *  응용프로그램의 섹션을 로딩하고 재배치를 수행
 */
static bool kLoadProgram( uint8_t* pbFileBuffer, 
        uint32_t* pqwApplicationMemoryAddress, uint32_t* pqwApplicationMemorySize, 
        uint32_t* pqwEntryPointAddress )
{
    Elf32_Ehdr* pstELFHeader;
    Elf32_Shdr* pstSectionHeader;
    Elf32_Word qwLastSectionSize;
    Elf32_Addr qwLastSectionAddress;
    int i;
    uint32_t qwMemorySize;
    uint8_t* pbLoadedAddress;

    //--------------------------------------------------------------------------
    // ELF 헤더 정보를 출력하고 분석에 필요한 정보를 저장
    //--------------------------------------------------------------------------
    pstELFHeader = ( Elf32_Ehdr* ) pbFileBuffer;
    pstSectionHeader = ( Elf32_Shdr* ) ( pbFileBuffer + pstELFHeader->e_shoff );
    
    // ELF의 ID와 클래스, 인코딩, 그리고 타입을 확인하여 올바른 응용프로그램인지 확인
    if( ( pstELFHeader->e_ident[ EI_MAG0 ] != ELFMAG0 ) ||
        ( pstELFHeader->e_ident[ EI_MAG1 ] != ELFMAG1 ) ||
        ( pstELFHeader->e_ident[ EI_MAG2 ] != ELFMAG2 ) ||
        ( pstELFHeader->e_ident[ EI_MAG3 ] != ELFMAG3 ) ||
        ( pstELFHeader->e_ident[ EI_CLASS ] != ELFCLASS32 ) ||
        ( pstELFHeader->e_ident[ EI_DATA ] != ELFDATA2LSB ) )
    {
        return false;
    }
    if ( pstELFHeader->e_type != ET_DYN )
    {
        return false;
    }

    //--------------------------------------------------------------------------
    // 모든 섹션 헤더의 로딩할 메모리 어드레스를 확인하여 가장 마지막에 있는 섹션을 찾음
    // 섹션의 정보도 같이 표시
    //--------------------------------------------------------------------------
    qwLastSectionAddress = 0;
    qwLastSectionSize = 0;
    for( i = 0 ; i < pstELFHeader->e_shnum ; i++ )
    {
        // 가장 마지막 섹션인지 확인, 이 값으로 프로그램이 사용할 전체 메모리 크기를
        // 알 수 있음
        if( ( pstSectionHeader[ i ].sh_flags & SHF_ALLOC ) &&                
            ( pstSectionHeader[ i ].sh_addr >= qwLastSectionAddress ) )
        {
            qwLastSectionAddress = pstSectionHeader[ i ].sh_addr;
            qwLastSectionSize = pstSectionHeader[ i ].sh_size;
        }
    }

    // 마지막 섹션의 위치로 최대 메모리 량을 계산, 4Kbyte 단위로 정렬
    qwMemorySize = ( qwLastSectionAddress + qwLastSectionSize + 0x1000 - 1 ) & 
        0xfffff000;

    // 응용프로그램에서 사용할 메모리를 할당
    // VirtualAlloc 및 mmap은 항상 page-aligned address를 반환함
#ifdef _WIN32
    pbLoadedAddress = ( uint8_t * ) VirtualAlloc(NULL, qwMemorySize, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if ( pbLoadedAddress == NULL )
    {
        return false;
    }
#else
    pbLoadedAddress = ( uint8_t * ) mmap(NULL, qwMemorySize,
        PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if( pbLoadedAddress == MAP_FAILED )
    {
        return false;
    }
#endif

    //--------------------------------------------------------------------------
    // 파일에 있는 내용을 메모리에 복사(로딩)
    //--------------------------------------------------------------------------
    for( i = 0 ; i < pstELFHeader->e_shnum ; i++ )
    {
        // 섹션 헤더에 로딩할 어드레스를 적용
        pstSectionHeader[ i ].sh_addr += ( Elf32_Addr ) pbLoadedAddress;        

        // 메모리에 올릴 필요가 없는 섹션이거나 Size가 0인 Section이면 복사할 필요 없음
        if( !( pstSectionHeader[ i ].sh_flags & SHF_ALLOC ) ||
            ( pstSectionHeader[ i ].sh_size == 0 ) )
        {
            continue;
        }
   
        // .bss와 같이 SHT_NOBITS가 설정된 섹션은 파일에 데이터가 없으므로 0으로 초기화
        if( pstSectionHeader[ i ].sh_type == SHT_NOBITS)
        {
            // 응용프로그램에게 할당된 메모리를 0으로 설정
            memset( (void *) pstSectionHeader[ i ].sh_addr, 0, pstSectionHeader[ i ].sh_size );
        }
        else
        {
            // 파일 버퍼의 내용을 응용프로그램에게 할당된 메모리로 복사
            memcpy( (void *) pstSectionHeader[ i ].sh_addr, 
                    pbFileBuffer + pstSectionHeader[ i ].sh_offset,
                    pstSectionHeader[ i ].sh_size );
        }
    }

    // 응용프로그램의 어드레스와 엔트리 포인트의 어드레스를 반환
    *pqwApplicationMemoryAddress = ( uint32_t ) pbLoadedAddress;
    *pqwApplicationMemorySize = qwMemorySize;
    *pqwEntryPointAddress = pstELFHeader->e_entry + ( uint32_t ) pbLoadedAddress;

    return true;
}


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
void b85tobin( uint8_t* dest, char const* src ) {

    for( char const* s = (char const*)src;; ) {

        uint32_t value = 0;
        for( uint32_t i = 0; i < sizeof pow85 / sizeof *pow85; ++i, ++s ) {
            uint32_t bin = digittobin[ (int) *s ];
            if ( bin == notadigit )
                return;
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

char ELF_binary_base85[] = "$$$$binary_base85$$$$"; // ELF linked as a static PIE encoded as base85 (PIE: position independent executable)
uint8_t ELF_binary[ $$$$len$$$$ ];
int ELF_binary_len = $$$$len$$$$;

int main() {
    uint32_t qwLoadedAddress;
    uint32_t qwMemorySize;
    uint32_t qwEntryPointAddress;

    b85tobin(ELF_binary, ELF_binary_base85);
    if (!kLoadProgram(ELF_binary, &qwLoadedAddress, 
            &qwMemorySize, &qwEntryPointAddress)) return 1;

    g_sf.ptr_imagebase    = (void *) qwLoadedAddress;
    g_sf.ptr_alloc        = (void *) svc_alloc;
    g_sf.ptr_alloc_zeroed = (void *) svc_alloc_zeroed;
    g_sf.ptr_dealloc      = (void *) svc_free;
    g_sf.ptr_realloc      = (void *) svc_realloc;
    g_sf.ptr_exit         = (void *) svc_exit;
    g_sf.ptr_read_stdio   = (void *) svc_read_stdio;
    g_sf.ptr_write_stdio  = (void *) svc_write_stdio;

    ((entry_ptr) qwEntryPointAddress)(&g_sf); // call the EntryPoint
    return 0; // should never be reached
}
//==============================================================================
// LOADER END
//==============================================================================