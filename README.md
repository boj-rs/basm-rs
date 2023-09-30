# basm.rs

basm.rs는 Rust 코드를 BOJ에 제출 가능한 C 프로그램으로 성능 저하 없이 변환해 주는 프로젝트입니다.

C 외에 Rust (메모리 사용량 감소)도 지원합니다.

> 156KB의 자유를 누리십시오!

## 효과

- 입력이 매우 간편하고 직관적입니다.

공백으로 구분된 a와 b를 받아 더한 결과를 출력하는 프로그램은 다음과 같이 작성할 수 있습니다.

```rust
use std::io::Read;
fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut input = s.split_whitespace().flat_map(str::parse);
    let a: usize = input.next().unwrap();
    let b: usize = input.next().unwrap();
    println!("{}", a + b);
}
```

이를 basm에서는 다음과 같이 작성할 수 있습니다.

> basm에서는 `main()` 함수가 반드시 `pub`으로 선언되어야 컴파일이 가능함에 주의해 주세요.

```rust
// src/solution.rs
use basm::platform::io::{Reader, Writer};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.usize();
    let b = reader.usize();
    writer.usize(a + b);
}
```

- 표시되는 메모리 사용량이 줄어듭니다.
  - C의 경우 156KB부터, Rust의 경우 2188KB부터 시작합니다.
  - 위의 예시 코드는 기본 설정에 따라 입출력 버퍼를 크게 할당합니다. 대부분의 상황에서는 기본 설정이 적절하지만, 156KB 메모리 사용량을 달성하려면 버퍼 크기를 줄여야 합니다. 다음 코드에서는 입출력 버퍼를 각각 128바이트로 설정하여 메모리 사용량을 줄입니다.
```rust
// src/solution.rs
use basm::platform::io::{Reader, Writer};
pub fn main() {
    let mut reader = Reader::<128>::new();
    let mut writer = Writer::<128>::new();
    let a = reader.usize();
    let b = reader.usize();
    writer.usize(a + b);
}
```

- **외부 crate를 사용할 수 있습니다.**

- **백준 온라인 저지(64비트), 코드포스(64비트), 코드포스 폴리곤(32비트 및 64비트)에서 테스트되었습니다.**

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

`src/solution.rs` main() 에 원하는 코드를 삽입하시고, 일반적인 cargo 프로젝트와 같은 방식으로 빌드 / 실행할 수 있습니다.

> cargo run 및 cargo run --release로 프로그램을 실행할 수 있고 cargo test나 cargo bench를 이용하여 테스트 및 성능 측정을 할 수 있습니다. 다만 로컬 환경에서 개발이 끝난 후 온라인 저지에 제출할 수 있는 형태로 빌드하기 위해서는 반드시 아래에 설명된 전용 스크립트를 사용해야 합니다.

Windows 환경에서 빌드하는 방법입니다.

* Windows 환경에서의 작동은 Python 3 라이브러리인 `pefile`을 필요로 하므로 `pip install pefile`로 설치하십시오.

* `release-64bit-windows.cmd`를 Windows 64비트 환경에서 실행하면 64비트 환경(백준 온라인 저지, 코드포스 등)에 제출 가능한 C 코드가 출력됩니다.

* `release-64bit-windows-rs.cmd`를 Windows 64비트 환경에서 실행하면 64비트 환경(백준 온라인 저지, 코드포스 등)에 제출 가능한 Rust 코드가 출력됩니다. 생성된 코드는 Windows와 Linux에서 모두 컴파일 가능합니다. 단, Windows에서 컴파일할 경우 DLL 대신 EXE를 생성하기 위해 생성된 코드 맨 앞의 `cdylib`를 `bin`으로 변경하거나 rustc 호출 시 `--crate-type=bin` 옵션을 추가해주세요.

* VS Code의 `build-release-amd64-win-submit` Task를 실행하면 릴리즈 모드 빌드 후 64비트 환경에 제출 가능한 C 코드가 VS Code 편집기에서 열립니다.

* VS Code의 `build-release-amd64-win-rs-submit` Task를 실행하면 릴리즈 모드 빌드 후 64비트 환경에 제출 가능한 Rust 코드가 VS Code 편집기에서 열립니다.

Linux (WSL 포함) 환경에서 빌드하는 방법입니다.

* `release.sh` 또는 `release-64bit.sh`를 실행하면 64비트 환경(백준 온라인 저지, 코드포스 등)에 제출 가능한 C 코드가 출력됩니다.

* `release-32bit.sh`를 실행하면 32비트 환경(코드포스 등)에 제출 가능한 C 코드가 출력됩니다.

* `release-rs.sh`를 실행하면 64비트 리눅스 환경(백준 온라인 저지 등)에 제출 가능한 Rust 코드가 출력됩니다. 생성된 코드를 Windows에서 컴파일하려면 crate type을 `cdylib`에서 `bin`으로 변경해야 합니다.

* VS Code의 `build-release-amd64-submit` Task를 실행하면 릴리즈 모드 빌드 후 64비트 환경에 제출 가능한 C 코드가 VS Code 편집기에서 열립니다.

* VS Code의 `build-release-amd64-rs-submit` Task를 실행하면 릴리즈 모드 빌드 후 64비트 환경에 제출 가능한 Rust 코드가 VS Code 편집기에서 열립니다.

~~`release-asm.sh`를 실행하면 제출 가능한 64bit Assembly 코드가 출력됩니다.~~ 추후 구현 예정입니다.

## 디버깅

> Windows 11 64비트, Windows Subsystems for Linux 2 (WSL2)에서 테스트되었습니다. 다른 환경에서 작동에 문제가 있을 시 이슈를 남겨주세요.

1. 64비트 리눅스에서 Visual Studio Code를 설치하신 다음, rust-analyzer 확장 및 CodeLLDB 확장을 설치해주세요. WSL을 사용하시는 경우 반드시 WSL 내부에 확장을 설치하셔야 합니다.

2. Launch configuration에서 `Debug executable 'basm' (amd64)`를 선택하여 실행하시면 생성된 코드의 진입점(EntryPoint)에 중단점(breakpoint)이 잡힙니다. 여기서부터 디버깅을 진행하시면 됩니다.

3. 기술적인 문제로 인해, 진입점에 중단점이 잡히기 전에 소스 코드에 설정한 중단점은 바로 적용되지 않습니다. 이를 해결하시려면 중단점 목록을 보여주는 `Breakpoints` 뷰에서 우측 상단의 동그라미 두 개가 겹쳐 있는 아이콘(`Toggle Breakpoints`)을 두 번 눌러주세요. 만약 그래도 중단점이 제대로 적용되지 않으면 이슈를 남겨주세요.

4. Launch configuration에서 `Debug executable 'basm' (i686)`을 선택하여 실행하시면 32비트로 빌드된 프로그램을 디버깅하실 수 있습니다.

5. 디버깅이 완료된 후에는 위의 "사용법"에 기술된 대로 `release.sh` 등을 실행하시면 Release 모드로 최종 프로그램을 빌드하실 수 있습니다.

6. 네이티브 Windows 64비트 환경에서는 Launch configuration에서 `Debug executable 'basm' (amd64-win)`를 선택하여 실행하시면 디버깅이 가능합니다. 단, Visual Studio Code를 `x64 Native Tools Command Prompt for Visual Studio 2022` 등에서 실행하셔야 합니다(로컬 환경에 따라 연도는 2022가 아닐 수 있음). 해당 명령 프롬프트에서 `code`라고 입력하여 Visual Studio Code를 실행하신 다음 1-5를 동일하게 진행하시면 됩니다.

## 주의사항

- Nightly Rust를 요구합니다.

- Python 3을 요구합니다.

- Linux에서 Binutils를 요구합니다.

- Windows에서 Python 3 라이브러리 `pefile`을 요구합니다.

- Windows에서 빌드하기 위해서 Microsoft C/C++ 컴파일러가 필요합니다. 가장 간단한 방법은 최신 버전의 Visual Studio를 설치하는 것입니다. 아래 링크를 참고하시면 도움이 됩니다.
  - https://learn.microsoft.com/ko-kr/windows/dev-environment/rust/setup
  - https://rust-lang.github.io/rustup/installation/windows-msvc.html

- `std`를 사용할 수 없습니다. 단, `cargo test` 시에는 `std`를 사용할 수 있습니다.

- `libc`를 사용할 수 없습니다.

- ~~백준 채점 환경인 Ubuntu 16.04를 기준으로 동작합니다.~~ 백준 온라인 저지, 코드포스, 코드포스 폴리곤에서 테스트되었으며, 네이티브 64비트 Windows 환경도 지원합니다.

## 문제 해결

- ~~이유를 알 수 없는 Segmentation Fault가 로컬에서 발생하는 경우~~ 이 버그는 현재 해결된 상태입니다.

- 빌드 결과 생성된 실행 파일(PE 및 ELF)을 직접 실행하면 작동하지 않습니다. 이는 해당 실행 파일이 로더(scripts/static-pie-template-\*.c 및 *.rs) 코드의 기능에 의존하기 때문입니다. 생성된 코드를 컴파일하여 실행하셔야 합니다.

- 생성되는 코드가 느리다면 Cargo.toml에서 opt-level을 기본값인 "z" (크기 우선 최적화)에서 3 (속도 우선 최적화)으로 변경해보세요. 다만 생성되는 코드의 길이가 늘어날 수 있습니다.

- ~~생성되는 코드의 크기는 추후 줄일 예정입니다.~~ 현재 LZMA compression이 적용되어 있습니다.

- 메모리 할당을 C runtime 없이 구현하기 위해 [dlmalloc](https://github.com/alexcrichton/dlmalloc-rs)이 적용되어 있습니다. 대부분의 경우 잘 작동하지만, 만약 실행시간이나 메모리 사용량이 2-3배 이상 과도하게 증가하는 등의 문제를 겪으신다면 꼭(!) 이슈를 남겨주세요.

- Windows도 아니고 Linux도 아닌 환경에서는 테스트되지 않았습니다. 이러한 환경에서는 현재 구현상 C runtime의 malloc을 사용하므로 메모리 할당이 정렬되지 않기 때문에 문제가 발생할 수 있습니다. 문제를 겪으시는 경우 이슈를 남겨주세요.

- Linux 환경에서 빌드하여 출력된 코드를 Windows 환경에서 컴파일하여 실행하는 경우 정상 작동을 보장할 수 없습니다. 이는 Linux 컴파일러가 Windows에서 사용하는 `__chkstk` 메커니즘을 지원하지 않기 때문입니다. Windows 환경에서 컴파일하여 실행해야 하는 경우 가급적 Windows 환경에서 빌드해 주세요. 이것이 어렵다면 하나의 함수 내에서 스택을 한 번에 4KB를 초과하여 이용하지 않도록 주의해주세요. 한편, Windows 환경에서 빌드하여 출력된 코드는 `__chkstk` 메커니즘을 포함하고 있으나 Windows가 아닌 환경에서 실행되는 경우 이를 비활성화하도록 구현되어 있기 때문에 Windows 및 Linux에서 모두 정상 작동이 가능합니다.

- Rust 코드 형태로 빌드한 경우 Windows 환경에서 컴파일하여 실행하기 위해서는 코드 상단에 crate type을 `cdylib`로 지정하는 부분을 제거해 주세요. (코드포스 등)

- 코드 구조 수정으로 인해 Assembly 코드로 변환하는 기능은 지원되지 않습니다.

- 현재 ARM은 32비트/64비트 둘 다 지원되지 않습니다. 지원이 필요하시면 이슈를 남겨주세요.

- 기타 빌드 및 실행 또는 디버깅 등에 문제가 있는 경우 이슈를 남겨주세요.

- 문의사항이 있으시면 원본 저장소인 [https://github.com/kiwiyou/basm-rs](https://github.com/kiwiyou/basm-rs)에 이슈를 남겨주세요.

## 예제: 큰 수 A+B ([BOJ 10757](https://www.acmicpc.net/problem/10757))

이 프로젝트를 다운로드 또는 클론한 다음, 위의 "주의사항"에 나열된 대로 Nightly Rust를 셋업합니다.

그런 다음, Cargo.toml의 [dependencies] 항목에 다음을 추가합니다.

```
dashu = { git = "https://github.com/cmpute/dashu.git", rev = "22f3935", default-features = false, features = [] }
```

src/solution.rs를 다음과 같이 수정합니다.

```rust
use basm::platform::io::{Reader, Writer};
use alloc::string::ToString;
use core::str::FromStr;
use dashu::Integer;

pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = Integer::from_str(&reader.word()).unwrap();
    let b = Integer::from_str(&reader.word()).unwrap();
    let ans = &a + &b;
    writer.bytes(ans.to_string().as_bytes());
    writer.byte(b'\n');
}
```

이제 로컬에서 빌드합니다.

### Windows

다음 스크립트를 실행하면 64비트 환경(백준 온라인 저지 등)에 제출 가능한 C 코드가 출력됩니다.

```
./release-64bit-windows.cmd > output.c
```

생성된 코드를 컴파일하여 실행합니다. (이를 실행하는 로컬 환경은 64비트라고 가정하고 있습니다)

```
cl output.c /F268435456
output
```

Microsoft C/C++ 컴파일러 `cl`이 `PATH`에 있어야 하며 기타 환경 변수가 잘 설정되어 있어야 컴파일이 문제없이 진행됩니다. 따라서 가급적 `x64 Native Tools Command Prompt for VS 2022` (로컬 환경에 따라 연도는 2022가 아닐 수 있음)에서 실행하는 것이 좋습니다.

### Linux

다음 스크립트를 실행하면 64비트 환경(백준 온라인 저지 등)에 제출 가능한 C 코드가 출력됩니다.

```
./release-64bit.sh > output.c
```

생성된 코드를 컴파일하여 실행합니다. (이를 실행하는 로컬 환경은 64비트라고 가정하고 있습니다)

```
gcc output.c -o output
chmod +x ./output
./output
```

32비트 환경(코드포스, 코드포스 폴리곤 등)에 제출 가능한 C 코드로 빌드하려면 다음과 같이 할 수 있습니다.
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
use basm::platform::io::{Reader, Writer};
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

pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let input = reader.word();
    if let Ok((_, ans)) = all_consuming(expr)(&input) {
        writer.bytes(ans.to_string().as_bytes());
    } else {
        writer.bytes(b"ROCK");
    }
    writer.byte(b'\n');
}
```

이후 실행 과정은 위의 "큰 수 A+B"와 동일하게 진행하면 됩니다.

## Open Source Attributions

[base85](https://github.com/rafagafe/base85/blob/master/base85.c)
```
Copyright (c) 2016-2018 Rafa Garcia <rafagarcia77@gmail.com>.

Permission is hereby  granted, free of charge, to any  person obtaining a copy
of this software and associated  documentation files (the "Software"), to deal
in the Software  without restriction, including without  limitation the rights
to  use, copy,  modify, merge,  publish, distribute,  sublicense, and/or  sell
copies  of  the Software,  and  to  permit persons  to  whom  the Software  is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE  IS PROVIDED "AS  IS", WITHOUT WARRANTY  OF ANY KIND,  EXPRESS OR
IMPLIED,  INCLUDING BUT  NOT  LIMITED TO  THE  WARRANTIES OF  MERCHANTABILITY,
FITNESS FOR  A PARTICULAR PURPOSE AND  NONINFRINGEMENT. IN NO EVENT  SHALL THE
AUTHORS  OR COPYRIGHT  HOLDERS  BE  LIABLE FOR  ANY  CLAIM,  DAMAGES OR  OTHER
LIABILITY, WHETHER IN AN ACTION OF  CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE  OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

[MINT64 OS](https://github.com/kkamagui/mint64os/blob/master/02.Kernel64/Source/Loader.c)
```
/**
 *  file    ApplicationLoader.c
 *  date    2009/12/26
 *  author  kkamagui
 *          Copyright(c)2008 All rights reserved by kkamagui
 *  brief   응용프로그램을 로드하여 실행하는 로더(Loader)에 관련된 함수를 정의한 소스 파일
 */
(brief in English:
    source file defining functions for loader that loads and runs applications)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

========

The ELF parsing and relocation routines in basm-rs were adapted
from the following implementation of MINT64OS, licensed under GPLv2+:
    https://github.com/kkamagui/mint64os/blob/master/02.Kernel64/Source/Loader.c

Unlike all other parts of basm-rs, which are under the MIT license,
the files implementing ELF parsing and relocation are exceptionally
licensed under GPLv2+ since it is derived from an existing GPLv2+
implementation, "Loader.c" (see above). Although GPLv2+ mandates
licensing the project in its entirety as GPLv2+, the original author
has kindly granted us permission to confine the GPLv2+ license to
the parts explicitly derived from "Loader.c".

There are currently three files licensed under GPLv2+:
    scripts/static-pie-elf2bin.py
    src/platform/loader/amd64_elf.rs
    src/platform/loader/i686_elf.rs
```

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

[dlmalloc-rs](https://github.com/alexcrichton/dlmalloc-rs)
```
Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```