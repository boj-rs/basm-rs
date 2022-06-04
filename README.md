# basm.rs

basm.rs는 Rust 코드를 BOJ에 제출 가능한 C 프로그램으로 성능 저하 없이 변환해 주는 프로젝트입니다.

C 외에 64bit Assembly도 지원합니다.

> 156KB의 자유를 누리십시오!

## 효과

- 메모리 사용량이 156KB부터 시작합니다. (C의 경우만 해당)

- **외부 crate를 사용할 수 있습니다.**

- AVX, AVX2, SSE 등의 SIMD를 사용할 수 있습니다.

- 다양한 최적화 옵션을 선택할 수 있습니다.

## 사용법

`basm.rs`는 그 자체로 완전한 Rust cargo 프로젝트입니다.

`src/solution.rs` main() 에 원하는 코드를 삽입하시고,
일반적인 cargo 프로젝트와 같은 방식으로 빌드 / 실행할 수 있습니다.

`release.sh`를 실행하면 제출 가능한 C 코드가 출력됩니다.

`release-asm.sh`를 실행하면 제출 가능한 64bit Assembly 코드가 출력됩니다.

## 주의사항

- Nightly Rust를 요구합니다.

- Python 3을 요구합니다.

- Binutils를 요구합니다.

- `std`를 사용할 수 없습니다.

- `libc`를 사용할 수 없습니다.

- Ubuntu 20.04 이외의 환경에서 테스트되지 않았습니다.

## 문제 해결

- `memcpy`, `memmove` 등의 함수를 찾을 수 없다고 나오는 경우

해당 함수를 직접 구현해주세요. `compiler_builtins` crate를 이용하시면 더 편합니다. 단, `#[no_mangle]`을 꼭 붙여 주세요.

- 이유를 알 수 없는 Segmentation Fault가 로컬에서 발생하는 경우

스택 크기를 확인해 주세요. 그래도 문제를 해결할 수 없는 경우 아직 발견된 해법이 없습니다.

