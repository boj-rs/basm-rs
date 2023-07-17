# basm.rs

basm.rs는 Rust 코드를 BOJ에 제출 가능한 C 프로그램으로 성능 저하 없이 변환해 주는 프로젝트입니다.

~~C 외에 64bit Assembly, Rust (메모리 사용량 감소)도 지원합니다.~~

> ~~156KB의 자유를 누리십시오!~~

현재 C와 Rust 코드 출력이 지원됩니다. 코드 구조 수정으로 인해 Assembly 코드로 변환하는 기능은 아직 지원되지 않습니다. 추후 구현 예정입니다.

## 효과

- 입력이 매우 간편하고 직관적입니다.

공백으로 구분된 a와 b를 받아 더한 결과를 출력하는 프로그램은 다음과 같이 작성할 수 있습니다.

```rust
let mut s = String::new();
std::io::stdin().read_to_string(&mut s).unwrap();
let mut input = s.split_whitespace().flat_map(str::parse);
let a: usize = input.next().unwrap();
let b: usize = input.next().unwrap();
println!("{}", a + b);
```

이를 basm에서는 다음과 같이 작성할 수 있습니다.

```rust
use basm::io::{Reader, Writer};

let mut reader: Reader = Default::default();
let mut writer: Writer = Default::default();
let a = reader.next_usize();
let b = reader.next_usize();
writer.write_usize(a + b);
```

- 표시되는 메모리 사용량이 줄어듭니다.

~~C의 경우 156KB부터, Rust의 경우 2188KB부터, Assembly의 경우 4212KB부터 시작합니다.~~
현재 구현은 C runtime을 사용하기 때문에 1144KB부터 시작합니다. 156KB보다는 크지만 Rust의 2188KB보다는 많이 작은 값입니다.

- **외부 crate를 사용할 수 있습니다.**

- **백준 온라인 저지와 코드포스 폴리곤에서 테스트되었습니다.**

- AVX, AVX2, SSE 등의 SIMD를 사용할 수 있습니다.

- 다양한 최적화 옵션을 선택할 수 있습니다.

- 이미 구현된 자료구조와 알고리즘을 쉽게 가져다 쓸 수 있습니다.

  - Jagged Array (인접 리스트에 사용할 수 있습니다)
  
  - Union-Find (by rank / rem algorithm)

  - DFS (매크로)

  - KMP (Iterator)

  - Fenwick Tree

  - Segment Tree

## 사용법

`basm.rs`는 그 자체로 완전한 Rust cargo 프로젝트입니다.

`src/solution.rs` main() 에 원하는 코드를 삽입하시고,
일반적인 cargo 프로젝트와 같은 방식으로 빌드 / 실행할 수 있습니다.

`release.sh` 또는 `release-64bit.sh`를 실행하면 64비트 환경(백준 온라인 저지 등)에 제출 가능한 C 코드가 출력됩니다.

`release-32bit.sh`를 실행하면 32비트 환경(코드포스 폴리곤 등)에 제출 가능한 C 코드가 출력됩니다.

`release-rs.sh`를 실행하면 64비트 리눅스 환경(백준 온라인 저지 등)에 제출 가능한 Rust 코드가 출력됩니다.

~~`release-asm.sh`를 실행하면 제출 가능한 64bit Assembly 코드가 출력됩니다.~~ 추후 구현 예정입니다.

## 주의사항

- Nightly Rust를 요구합니다.

- Python 3을 요구합니다.

- Binutils를 요구합니다.

- `std`를 사용할 수 없습니다.

- `libc`를 사용할 수 없습니다.

- ~~백준 채점 환경인 Ubuntu 16.04를 기준으로 동작합니다.~~ 백준 온라인 저지와 코드포스 폴리곤에서 테스트되었습니다.

- 디버깅 지원은 추후 보강할 예정입니다.

## 문제 해결

- ~~이유를 알 수 없는 Segmentation Fault가 로컬에서 발생하는 경우~~ 이 버그는 현재 해결된 상태입니다.

- 빌드 결과 생성된 실행 파일(ELF)을 직접 실행하면 작동하지 않습니다. 이는 해당 실행 파일이 로더(scripts/static-pie-template-\*.c) 코드의 기능에 의존하기 때문입니다. 생성된 코드를 컴파일하여 실행하셔야 합니다.

- 생성되는 코드가 느리다면 Cargo.toml에서 opt-level을 기본값인 "z" (크기 우선 최적화)에서 3 (속도 우선 최적화)으로 변경해보세요. 다만 생성되는 코드의 길이가 늘어날 수 있습니다.

- ~~생성되는 코드의 크기는 추후 줄일 예정입니다.~~ 현재 LZMA compression이 적용되어 있습니다.

- 문의사항이 있으시면 원본 저장소인 [https://github.com/kiwiyou/basm-rs](https://github.com/kiwiyou/basm-rs)에 이슈를 남겨주세요.

## 예제: 큰 수 A+B ([BOJ 10757](https://www.acmicpc.net/problem/10757))

이 프로젝트를 다운로드 또는 클론한 다음, 위의 "주의사항"에 나열된 대로 Nightly Rust를 셋업합니다.

그런 다음, Cargo.toml의 [dependencies] 항목에 다음을 추가합니다.

```
dashu = { git = "https://github.com/cmpute/dashu.git", rev = "22f3935", default-features = false, features = [] }
```

src/solution.rs를 다음과 같이 수정합니다.

```rust
use basm::io::{Reader, Writer};
use crate::alloc::string::ToString;
use core::str::FromStr;
use dashu::Integer as Int;

#[inline(always)]
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = Int::from_str(&reader.next_string()).unwrap();
    let b = Int::from_str(&reader.next_string()).unwrap();
    let ans = &a + &b;
    writer.write(ans.to_string().as_bytes());
    writer.write(b"\n");
}
```

로컬에서 빌드합니다. 64비트 환경(백준 온라인 저지 등)에 제출 가능한 C 코드가 출력됩니다.

```
./release-64bit.sh > output.c
```

생성된 코드를 컴파일하여 실행합니다. (이를 실행하는 로컬 환경은 64비트라고 가정하고 있습니다)

```
gcc output.c -o output
chmod +x ./output
./output
```

32비트 환경(코드포스 폴리곤 등)에 제출 가능한 C 코드로 빌드하려면 다음과 같이 할 수 있습니다.
```
./release-32bit.sh > output-32.c
```

마찬가지로 생성된 코드를 컴파일하여 실행합니다. (이를 실행하는 로컬 환경은 64비트라고 가정하고 있습니다)

```
gcc output-32.c -o output-32 -m32
chmod +x ./output-32
./output-32
```

## 예제: 할 수 있다([BOJ 1287](https://www.acmicpc.net/problem/1287))

이 프로젝트를 다운로드 또는 클론한 다음, 위의 "주의사항"에 나열된 대로 Nightly Rust를 셋업합니다.

그런 다음, Cargo.toml의 [dependencies] 항목에 다음을 추가합니다.

```
nom = { version = "7.1.3", default-features = false, features = ["alloc"] }
dashu = { git = "https://github.com/cmpute/dashu.git", rev = "22f3935", default-features = false, features = [] }
```

src/solution.rs를 다음과 같이 수정합니다.


```rust
use basm::io::{Reader, Writer};
use alloc::string::ToString;
use core::str::FromStr;
use dashu::Integer;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_digit,
    character::complete::one_of,
    combinator::{all_consuming, map_res},
    multi::many0,
    sequence::{delimited, pair},
};


fn number_literal(input: &str) -> IResult<&str, Integer> {
    map_res(take_while(|x: char| is_digit(x as u8)), |s: &str| Integer::from_str(s))(input)
}
fn op(input: &str) -> IResult<&str, char> {
    one_of("+-*/")(input)
}
fn wrapped(input: &str) -> IResult<&str, Integer> {
    delimited(tag("("), expr, tag(")"))(input)
}
fn number(input: &str) -> IResult<&str, Integer> {
    alt((number_literal, wrapped))(input)
}
fn expr(input: &str) -> IResult<&str, Integer> {
    map_res(
        pair(number, many0(pair(op, number))),
        |x| -> Result<Integer, usize> {
            let (mut a, mut b) = (Integer::ZERO, x.0);
            for p in x.1 {
                match p.0 {
                    '+' => { (a, b) = (a + b, p.1); },
                    '-' => { (a, b) = (a + b, -p.1); },
                    '*' => { b *= p.1; },
                    '/' => if p.1 == Integer::ZERO { return Err(1); } else { b /= p.1; },
                    _ => unreachable!()
                }
            }
            Ok(a + b)
        }
    )(input)
}

#[inline(always)]
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let input = reader.next_string();
    if let Ok((_, ans)) = all_consuming(expr)(&input) {
        writer.write(ans.to_string().as_bytes());
    } else {
        writer.write(b"ROCK");
    }
    writer.write(b"\n");
}
```

이후 실행 과정은 위의 "큰 수 A+B"와 동일하게 진행하면 됩니다.

## Open Source Attributions

[Micro LZMA decoder](https://github.com/ilyakurdyukov/micro-lzmadec)
```
Copyright (c) 2022, Ilya Kurdyukov
All rights reserved.

Micro LZMA decoder for x86 (static)
Micro LZMA decoder for x86_64 (static)

This software is distributed under the terms of the
Creative Commons Attribution 3.0 License (CC-BY 3.0)
http://creativecommons.org/licenses/by/3.0/

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```