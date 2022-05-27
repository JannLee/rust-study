# Rust 스터디 1회차
- [Rust 레퍼런스 번역 문서](https://rinthel.github.io/rust-lang-book-ko/) 스터디

## 들어가기 앞서
- 어려운 메모리 관리, 데이터 표현, 그리고 동시성에 대한 저수준의 디테일을 다루는 “시스템 레벨” 관련 일을 --> 길을 잃지 않도록 다양한 지원을 제공
- 러스트 프로그래밍은 권한 분산에 관한 것
- 메모리 관리, 동시성에 대한 저수준을 다루는 시스템 레벨에서 실수하지 않도록 도와준다.
- 읽어본 결과, 번역이 매끄럽지 못한 부분도 있는 듯 했음

## 소개
- 누구에게 유용한가?

### 개발자 팀
- 버그 추적보다 프로그램 로직 자체에 집중할 수 있음
- 그러므로 리소스를 효율적으로 사용할 수 있다.
- 유용한 툴이 제공된다
    - Cargo (패키지 매니저)
    - Rustfmt (코딩 스타일 툴)
    - Rust Language Serve (자동완성, 인라인 에러메시지)

### 속도와 안정성을 소중하게 생각하는 사람
- 생산성이 좋다(개발 및 동작 속도와 안정성 확보)

### 이 책을 이용하는 방법
 관련해서 3장 보고 2장의 소스를 나름대로 수정해보는건 어떨까요? 
 
## 설치하기
- rustup을 통해 설치 https://www.rust-lang.org/en-US/install.html
- 버전 정보 확인
    ```cmd=
    > rustc --version
    ```

## Hello, World!

### 코드
```rust
fn main() {
    println!("Hello, world!");
}
```

### 실행하기
```cmd
> rustc main.rs
> ./main
Hello, world!
```

### main 함수
- main 함수는 특별하다.
- 첫번째로 실행되는 함수
- C언어와 비슷하니 금방 와닿을 듯
- rustfmt라는 자동 포맷팅 도구가 개발된 것 같음
    - 검색해보니 이미 다 나와 있는 것 같음
- 러스트는 탭이 아닌 4개의 들여쓰기를 사용
- `println!`은 러스트 매크로
- 각각의 라인은 `;`으로 끝남

### 컴파일과 실행은 개별적인 단계입니다
- Windows에서 컴파일하면 pdb 파일(심볼 파일)이 같이 생성됨
    - main.exe
    - main.pdb
    - main.rs
    ```cmd=
    > rustc main.rs
    > .\main.exe
    ```

## Hello, Cargo!
- 빌드 시스템 및 패키지 매니저
    ```cmd
    > cargo --version
    ```

- 프로젝트 생성
    ```cmd
    > cargo new hello_cargo --bin
    > cd hello_cargo
    ```
    - --bin 옵션은 실행파일 프로젝트

- `cargo new`를 통해 생성한 Cargo.toml 파일
    ```toml
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    authors = ["Your Name <you@example.com>"]

    [dependencies]
    ```
    - `[package]` 이후 문장은 패키지 환경설정
    - `[dependencies]` 이후는 프로젝트 의존성
- crate 를 사용해 코드 패키지를 추가할 수 있다. 

### Cargo 프로젝트를 빌드하고 실행하기

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
- `cargo build` 시에 target/debug 폴더 하위에 빌드 결과물이 생김

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
- `cargo run` 빌드와 실행을 한번에 함

```
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
- cargo check
    - 컴파일이 되는지 확인 용도
    - 실행파일을 원치 않을때(실행파일을 생성하지 않으므로 빠름)

- Cargo를 사용하면 어떤 OS를 사용하든 커맨드가 동일하다!

### 관례로서의 Cargo
- 프로그램이 복잡할 수록 rustc보다 Cargo 사용이 좋다 (dependencies를 이용한 패키지 관리의 편리함)

## 추리 게임
### 새로운 프로젝트를 준비하기
```
cargo new guessing_game --bin
```
### 추리값을 처리하기

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
```rust
use std::io;
```
use 키워드
- 사용자 입력을 받기 위해 `io`라이브러리를 사용해야함.

let 키워드
- 변수 선언(ex: let guess )
- rust의 변수는 기본이 불변

mut 키워드
- mut 키워드를 써야 변경 가능 (ex: let mut var)

연관 함수
- "::" == 정적 메소드

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```
- read_line 함수는 표준 입력에 입력될때마다 문자열에 추가하므로 값을 저장할 문자열을 가변(&mut)으로 받는다 


### Result 타입으로 잠재된 실패 다루기
- Result 타입
    - 열거형(enumerations)
    - 정해진 값들을 가질 수 있음 -> 열거형의 variants
    - 성공시 Ok, 실패시 Err


- expect
    - 프로그램 작동을 멈춤(exception)
    - 지정한 문자열을 출력
    - expect를 사용하지 않을 경우 경고
        - 개발자가 예외가 발생할 수 있는 상황을 인지하게함
    

### println! 변경자(placeholder)를 이용한 값 출력
```rust
println!("x = {} and y = {}", x, y);
```
- C++ 20에 추가된 [std::format](https://en.cppreference.com/w/cpp/utility/format/format) 과 유사

### 비밀번호를 생성하기
- Crate는 러스트에서 패키지를 말함

#### 패키지 추가
```toml
[dependencies]

rand = "0.3.14"
```

- 기본적으로 cargo 버전은 SemVer 규칙을 따름
- Cargo는 Crates.io 데이터 복제본인 레지스트리에서 모든것을 가져온다

### Cargo.lock
- 누구라도 같은 산출물이 나오도록 보장하는 메타 정보가 기록되는 파일
- 명시적으로 업그레이드하지 않은 이상 Cargo.lock 정보를 기반으로 해당 버전을 사용

### 크레이트를 새로운 버전으로 업그레이드하기

```
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```
#### Crate 버전 업데이트
- toml 파일에서 버전 수정
```toml
[dependencies]

rand = "0.4.0"
```

### 임의의 숫자 생성하기
```rust
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
- `cargo doc --open` 명령어로 사용중인 패키지가 제공하는 문서를 브라우저에 표시해줌

### 비밀번호와 추리값을 비교하기

```rust
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

```rust
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

- C/C++ 등의 switch-case-break 문 사용 방식과 유사한데, 매우 심플한 듯
- match 표현식은 arm으로 이루어짐.(arm: 하나의 패턴)

### Shadowing
- 이전 값을 가리는 것을 허용

```rust
let mut guess = String::new();

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```
- Shadowing의 유용함에 대한 의문..? 
    - 더이상 불필요한 변수를 Shadowing으로 소멸시키는 것과 관련 있지 않을까?
    - 효율적인 성능과 메모리 관리 관점?

### 반복문을 이용하여 여러 번의 추리 허용
#### loop 키워드
- 무한 루프를 제공
- break 문으로 탈출

### 잘못된 입력값 처리하기

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
#### 완성된 코드
```rust
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## 회고
- 이지한: 첫번째 시간인데 나름 잘 진행된 것 같습니다.
- 허신: mut, & 과 같은 것들을 명확하게 하는 것이 인상적이었습니다.
- 정연집: crate가 편해보입니다.
- 유병조: 코틀린이랑 비슷한 시기에 나와서 그런지 유사점이 있는거 같습니다(match, 불변 변수 권장),   함수형 지원이 어떤지 궁금하네요
- 이명수: 용어가 좀 낯설지만 재미지네요 ㅎㅎ
- 김기덕: 새로운 것들을 공부하는게 좋았습니다.
- 이재현: 기본 변수가 불변이라는게 좀 특이하다고 생각했습니다. 
