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
void b91tobin(void *dest, char const *src) {
    u8 *p = (u8 *)dest;
    u32 eax = 0x1f;
    while (1) {
        while (*src == '\0') src++;
        u32 x = (u32) *src++;
        if (x < 0x24) return;
        while (*src == '\0') src++;
        u32 y = (u32) *src++;
        eax <<= 13;
        eax += (y - 0x24) * 91 + (x - 0x24);
        do {
            *p++ = (u32) eax;
            eax >>= 8;
        } while (eax & (1 << 12));
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
char payload[][$$$$min_len_4096$$$$] = $$$$binary_base91_chunked$$$$;
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
    b91tobin(stubbuf, "H;|DR:$$$|7x6E69i$6',&%Q$$?@GjeBmVodz$C?$$c7h{.>j<g9%Q$$Q80&F$$$f5U$5L@=aT8S92:|1&.C!");
    b91tobin(stubbuf + 68, $$$$stub_base91$$$$);
    size_t base = ((size_t)main) & 0xFFFFFFFFFFFFF000ULL;
    *(u64 *)(stubbuf + 0x08) = (u64) base;
    *(u32 *)(stubbuf + 0x11) = (u32) 4096;
    base = ((size_t)stubbuf) & 0xFFFFFFFFFFFFF000ULL;
    size_t len = (((size_t)stubbuf) + 68 + $$$$stub_len$$$$) - base;
    len = ((len + 0xFFF) >> 12) << 12;
    syscall(10, base, len, 0x7);
    pd.fn_table[0] = (void *) (stubbuf + 0x1c);
    b91tobin(payload, (char const *)payload);
    return ((stub_ptr) stubbuf)(&pd, payload);
}