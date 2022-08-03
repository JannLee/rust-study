# 7차 러스트 스터디


## 담당
- 발표: 정연집
- 리뷰: 이지한
- 서기: 유병조

## 과제 리뷰
### [생일](https://www.acmicpc.net/problem/5635)
이지한 님 : [코드리뷰](https://github.com/JannLee/rust-study/blob/main/jannlee/beakjoon5635_birthday/src/main.rs)


## I/O 프로젝트: 커맨드 라인 프로그램 만들기

### 커맨드라인 인자 허용하기
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```
- `std::env::args()`를 통해 환경 변수 목록을 가져옴
- 반복자는 하나의 연속된 값을 생성
- collect로 벡터 변환 가능

```shell
$ cargo run
["target/debug/greprs"]

$ cargo run needle haystack
...snip...
["target/debug/greprs", "needle", "haystack"]
```
- 첫번째 환경 변수는 바이너리 이름


### 변수에 인자 값들 저장하기

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```
```shell
$ cargo run test sample.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs test sample.txt`
Searching for test
In file sample.txt
```
- cargo run 명령어로 환경 변수를 전달 가능


## 파일 읽기

```rust
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```
- `File::open` 함수로 파일 핸들을 얻음
- `read_to_string` 함수로 파일 핸들로 부터 문자열을 반환

```shell
$ cargo run the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```


## 모듈성과 에러처리의 향상을 위한 리팩토링

1. 메인 함수에서 여러 작업을 하고 있어 다른 함수로 분리해야 합니다.
2. 각각의 변수를 추적하기 힘들어집니다. 목적을 분명히 하기 위해 설정 변수를 그룹화할 필요가 있습니다
3. 파일 열기가 실패 할 경우expect를 사용하여 오류 메시지를 출력해주는데, 에러 메시지가 Something went wrong reading the file 밖에 없습니다.

4. 우리는 서로 다른 오류를 다루기 위해 expect를 반복적으로 사용하고 있습니다.

### 인자 파서의 추출
```rust
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```
- 커맨드 라인 인자를 분석하는 기능을 별도의 함수로 분리

### 설정 변수들을 그룹짓기
```rust
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```
- 튜플을 반환하는 것은 개별 정보를 확인하기 어려움
- 구조체를 생성해서 처리

### Config를 위한 생성자 만들기.
```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // ...snip...
}

// ...snip...

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```
- impl로 내장 함수 구현

### 에러 처리 수정하기

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1',  /stable-dist-rustc/build/src/libcollections/vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
- 인자값에 대한 검증을 하고 있지 않음
- 좀더 도움되는 형태로 에러 메시지를 향상시켜보자.

### 에러 메시지 향상시키기
```rust
// ...snip...
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // ...snip...

```
```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs`
thread 'main' panicked at 'not enough arguments', src/main.rs:29
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
- panic을 통해 보다 명시적인 에러 메시지를 제공함

### new에서 panic!을 호출하는 대신 Result를 반환하기.
```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

### Config::new를 호출하고 에러 처리하기
- unwrap_or_else()
    - unwrap과 유사한데 closure를 추가 활용하여 에러 처리 가능

### run 함수 추출하기
- 로직을 lib.rs로 분리함
- main문은 간결해지고 다른 로직을 테스트하기 쉬워짐 

### run 함수에서 에러 반환하기
```rust
use std::error::Error;

// ...snip...

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

- `Box<Error>`는 함수가 Error 특성을 구현하는 타입을 반환한다는 것만 알면 된다. 다양한 에러 상황에 다른 타입의 오류 값을 반환 할 수 있는 유연성을 확보할 수 있습니다.
- 유닛 타입: c++의 void
- 런타임에 메서드를 호출해야 하는 dyn Trait경우 vtable을 참조하여 함수 포인터를 가져온 다음 해당 함수 포인터가 호출됩니다.

### main안의 run에서 반환되는 에러 처리하기
```rust
fn main() {
    // ...snip...

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```
    
### 라이브러리 크레이트로 코드를 나누기
- lib.rs로 코드를 옮기고 main.rs에서 extern crate로 가져와 호출하도록 변경

## 테스트 주도 개발로 라이브러리의 기능 개발하기
- TDD 프로세스
    1. 실패할 테스트를 작성하고, 의도한 대로 실패하는지 실행해보세요.
    2. 새 테스트를 통과하기 충분할 정도로 코드를 작성하거나 수정하세요.
    3. 추가하거나 수정하는 정도의 리팩토링을 해보고, 여전히 테스트를 통과하는지 확인해보세요.
    4. 1단계로 반복!

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```
- search함수로 반환되는 데이터는 search함수로 전달된 contents인자만큼 오래 유지될 것
- Rust는 두 인자 중에 우리가 필요한 쪽이 어느건지 알 수 없기 때문에, 우리가 알려줘야 합니다.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```
- search 메소드를 구현하고 테스트가 통과되는지 확인

## 환경 변수들을 활용하기

### 대소문자를 구분하는 search 함수의 실패 케이스 작성하기
```rust

#[cfg(test)]
mod test {
    use super::*;

 
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### search_case_insensitive 함수 구현하기
```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```
```rust
use std::env;
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

## 표준출력 대신 표준에러로 에러메시지 출력하기
- eprintln! 매크로를 통해 표준 에러로 출력

## 다음 과제
- [백준 1408(24)](https://www.acmicpc.net/problem/1408)

## 벌금현황
- 이지한: 0
- 김기덕: 0
- 유병조: 0
- 이명수: 0
- 이재현: 0
- 정연집: 0
- 허신: 10,000

## 회고
- 이지한: 라이프타임이 계속 발목을 잡는 느낌.. 주말에 날잡고 깊게 파봐야겠습니다.
- 김기덕: 이번 챕터는 프로젝트로 만드는 내용이 많아서 재미있었습니다! 하지만 라이프타임은 여전히 어렵네요 ㅠㅠ
- 유병조: 라이프타임이 헷갈리는데 예시 프로젝트로 보니까 좀 이해가 되는거 같아요
- 이명수: 이전 챕터보다 많이 유용했던거 같네요..
- 이재현: 이전에 했던걸 돌아볼 수 있어서 좀 더 문법 이해에 도움이 되서 좋았습니다.
- 정연집: 이해 안되는 부분 이야기하고 이해할수있어서 좋았습니다.
- 허신: 불참1
