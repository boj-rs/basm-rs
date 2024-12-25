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

#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#if defined(__cplusplus)
#include <vector>
#include <string>
#include <utility>
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

static int g_loaded = 0;
static PLATFORM_DATA g_pd;

void basm_on_loaded();
size_t basm_load_module() {
    if (!g_loaded) {
        g_pd.env_flags            = ENV_FLAGS_NO_EXIT;
#if defined(_WIN32)
        g_pd.env_id               = ENV_ID_WINDOWS;
#elif defined(__linux__)
        g_pd.env_id               = ENV_ID_LINUX;
        // Linux's stack growth works differently than Windows.
        // Hence, we disable the __chkstk mechanism on Linux.
        g_pd.env_flags            |= ENV_FLAGS_LINUX_STYLE_CHKSTK;
#else
        g_pd.env_id               = ENV_ID_UNKNOWN;
#endif
#if defined(_WIN32)
        g_pd.win_kernel32         = (uint64_t) GetModuleHandleW(L"kernel32");
        g_pd.win_GetProcAddress   = (uint64_t) GetProcAddress;
#endif
        g_pd.ptr_alloc_rwx        = (void *) svc_alloc_rwx;
#if !defined(_WIN32) && !defined(__linux__)
        g_pd.ptr_alloc            = (void *) svc_alloc;
        g_pd.ptr_alloc_zeroed     = (void *) svc_alloc_zeroed;
        g_pd.ptr_dealloc          = (void *) svc_free;
        g_pd.ptr_realloc          = (void *) svc_realloc;
        g_pd.ptr_read_stdio       = (void *) svc_read_stdio;
        g_pd.ptr_write_stdio      = (void *) svc_write_stdio;
#endif
        stub_ptr stub = get_stub();
        b85tobin(payload, (char const *)payload);
        stub(&g_pd, payload);
        g_loaded = 1;
        basm_on_loaded();
    }
    return (size_t) g_pd.ptr_alloc_rwx;
}

#define BASM_LOADER_IMAGEBASE (basm_load_module())

// Ser
template <typename T> struct ser { using impl = void; };

template <typename T, typename Impl = typename ser<T>::impl>
void do_ser(std::vector<uint8_t>& buf, T val) {
    Impl(buf, val);
}

#define SER_RAW(ty) template<> struct ser<ty> { using impl = ser_impl_raw<ty>; }
#define SER_RAW_PTR(ty) template<> struct ser<ty> { using impl = ser_impl_raw_ptr<ty>; }
#define SER_INT(ty) SER_RAW(ty); SER_RAW_PTR(const ty *); SER_RAW_PTR(ty *)

template <typename T>
class ser_impl_raw {
    public:
        ser_impl_raw(std::vector<uint8_t>& buf, T val) {
            for (size_t i = 0; i < sizeof(T); i++) buf.emplace_back((uint8_t) (val >> ((sizeof(T) - i - 1) * 8)) & 0xFF);
        }
};

template <typename T>
class ser_impl_raw_ptr {
    public:
        ser_impl_raw_ptr(std::vector<uint8_t>& buf, T val) {
            for (size_t i = 0; i < sizeof(T); i++) buf.emplace_back((uint8_t) (((size_t)val) >> ((sizeof(T) - i - 1) * 8)) & 0xFF);
        }
};

class ser_impl_bool {
    public:
        ser_impl_bool(std::vector<uint8_t>& buf, bool val) {
            buf.emplace_back(val ? 1 : 0);
        }
};

template <typename T1, typename T2>
class ser_impl_pair {
    public:
        ser_impl_pair(std::vector<uint8_t>& buf, std::pair<T1, T2> val) {
            do_ser(buf, val.first);
            do_ser(buf, val.second);
        }
};

template <typename T>
class ser_impl_vec {
    public:
        ser_impl_vec(std::vector<uint8_t>& buf, std::vector<T> val) {
            do_ser(buf, val.size());
            for (auto e : val) do_ser(buf, e);
        }
};
template <> struct ser<bool> { using impl = ser_impl_bool; };

SER_INT(char);
SER_INT(unsigned char);
SER_INT(short int);
SER_INT(unsigned short int);
SER_INT(int);
SER_INT(unsigned int);
SER_INT(long int);
SER_INT(unsigned long int);
SER_INT(long long int);
SER_INT(unsigned long long int);
SER_RAW_PTR(const bool*);
SER_RAW_PTR(bool*);
template <typename T1, typename T2> struct ser<std::pair<T1, T2>> { using impl = ser_impl_pair<T1, T2>; };
template <typename T> struct ser<std::vector<T>> { using impl = ser_impl_vec<T>; };

void do_ser_end(std::vector<uint8_t>& buf) {
    size_t len = buf.size() - sizeof(size_t);
    for (size_t i = 0; i < sizeof(size_t); i++) buf[i] = (uint8_t) (len >> ((sizeof(size_t) - i - 1) * 8)) & 0xFF;
}

// De
template <typename T> struct de { using impl = void; };

template <typename T, typename Impl = typename de<T>::impl>
T do_de(size_t& ptr) {
    return Impl::impl_de(ptr);
}

#define DE_RAW(ty) template<> struct de<ty> { using impl = de_impl_raw<ty>; }
#define DE_RAW_PTR(ty) template<> struct de<ty> { using impl = de_impl_raw_ptr<ty>; }
#define DE_INT(ty) DE_RAW(ty); DE_RAW_PTR(const ty *); DE_RAW_PTR(ty *)

template <typename T>
class de_impl_raw {
    public:
        static T impl_de(size_t& ptr) {
            T val = 0;
            for (size_t i = 0; i < sizeof(T); i++) val = (val << 8) | (T) *((uint8_t *)(ptr++));
            return val;
        }
};

template <typename T>
class de_impl_raw_ptr {
    public:
        static T impl_de(size_t& ptr) {
            size_t val = 0;
            for (size_t i = 0; i < sizeof(T); i++) val = (val << 8) | (T) *((uint8_t *)(ptr++));
            return (T) val;
        }
};

class de_impl_bool {
    public:
        static bool impl_de(size_t& ptr) {
            uint8_t val = *((uint8_t *)(ptr++));
            return val != 0;
        }
};
template <> struct de<bool> { using impl = de_impl_bool; };

DE_INT(char);
DE_INT(unsigned char);
DE_INT(short int);
DE_INT(unsigned short int);
DE_INT(int);
DE_INT(unsigned int);
DE_INT(long int);
DE_INT(unsigned long int);
DE_INT(long long int);
DE_INT(unsigned long long int);
DE_RAW_PTR(const bool*);
DE_RAW_PTR(bool*);

template <typename T1, typename T2>
class de_impl_pair {
    public:
        static std::pair<T1, T2> impl_de(size_t& ptr) {
            T1 val1 = do_de<T1>(ptr);
            T2 val2 = do_de<T2>(ptr);
            return std::make_pair(val1, val2);
        }
};
template <typename T1, typename T2> struct de<std::pair<T1, T2>> { using impl = de_impl_pair<T1, T2>; };

template <typename T>
class de_impl_vec {
    public:
        static std::vector<T> impl_de(size_t& ptr) {
            size_t length = do_de<size_t>(ptr);
            std::vector<T> val;
            for (size_t i = 0; i < length; i++) val.push_back(do_de<T>(ptr));
            return val;
        }
};
template <typename T> struct de<std::vector<T>> { using impl = de_impl_vec<T>; };

class de_impl_string {
    public:
        static std::string impl_de(size_t& ptr) {
            size_t length = do_de<size_t>(ptr);
            std::string val;
            for (size_t i = 0; i < length; i++) val.push_back(do_de<char>(ptr));
            return val;
        }
};
template <> struct de<std::string> { using impl = de_impl_string; };

$$$$exports_cpp$$$$