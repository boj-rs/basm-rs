// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!
// SOLUTION BEGIN
$$$$solution_src$$$$
// SOLUTION END
#include <unistd.h>
typedef unsigned char u8;
typedef unsigned int u32;
typedef unsigned long long u64;
#define BASMCALL __attribute__((ms_abi))
// Base85 decoder. Code adapted from:
//     https://github.com/rafagafe/base85/blob/master/base85.c
const char *b85 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>\?@^_`{|}~";
void b85tobin(void *dest, char const *src) {
    u32 *p = (u32 *)dest;
    u8 digittobin[256];
    for (u8 i=0; i<85; i++) digittobin[(u8)b85[i]] = i;
    while (1) {
        while (*src == '\0') src++;
        if (*src == ']') break;
        u32 value = 0;
        for (u32 i=0; i<5; i++) {
            value *= 85;
            value += digittobin[(u8)*src++];
        }
        *p++ = (value >> 24) | ((value >> 8) & 0xff00) | ((value << 8) & 0xff0000) | (value << 24);
    }
}
#pragma pack(push, 1)
typedef struct {
    u64 env_id;
    u64 env_flags;
    u64 win[2];
    void *fn_table[6];
} PLATFORM_DATA;
#pragma pack(pop)
typedef int (BASMCALL *stub_ptr)(void *, void *);
char payload[][$$$$min_len_4096$$$$] = $$$$binary_base85$$$$;
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
    PLATFORM_DATA pd;
    pd.env_id = 2;
    pd.env_flags = 1;
    u8 stubbuf[68 + $$$$stub_len$$$$];
    b85tobin(stubbuf, "QMd~L002n8@6D@;XGJ3cz5oya01pLO>naZmS5~+Q0000n|450>x(5IN07=KfA^-pYO)<bp|Hw@-$qxlyU&9Xz]");
    b85tobin(stubbuf + 68, $$$$stub_base85$$$$);
    size_t base = ((size_t)main) & 0xFFFFFFFFFFFFF000ULL;
    *(u64 *)(stubbuf + 0x08) = (u64) base;
    *(u32 *)(stubbuf + 0x11) = (u32) 4096;
    base = ((size_t)stubbuf) & 0xFFFFFFFFFFFFF000ULL;
    size_t len = (((size_t)stubbuf) + 68 + $$$$stub_len$$$$) - base;
    len = ((len + 0xFFF) >> 12) << 12;
    syscall(10, base, len, 0x7);
    pd.fn_table[0] = (void *) (stubbuf + 0x1c);
    b85tobin(payload, (char const *)payload);
    return ((stub_ptr) stubbuf)(&pd, payload);
}