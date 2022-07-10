# Rust 스터디 5회차

## 과제 리뷰
### 1. 평균값, 중간값, 최빈값 
- 발표: 유병조

### 2. pig latin 변환
- 발표: 정연집

### 3. 부서별 직원 목록 추가 및 조회
- 발표: 김기덕

## 챕터 9 에러 처리
- 발표: 이재현
- 

### 9.1 panic!과 함께하는 복구 불가능한 에러
* 러스트에는 **복구 가능한(recoverable) 에러**와 **복구 불가능한(unrecoverable)에러**가 있음.
* 복구 불가능한 오류가 발생했을 때 실행을 멈추는 panic!
* 되감기 없이 프로그램 끝내는 대안: 'abort'(Cargo.Toml을 통해 설정)

#### panic이 발생할 수 있는 상황들.
* 유효하지 않은 인덱스 접근(buffer overread)
``` rust
let v = vec![1, 2, 3];
v[99];
```

#### panic! 백트레이스 사용하기
* RUST_BACKTRACE 환경변수를 설정하여 사용.
```
$ RUST_BACKTRACE=1 cargo run
```


### 9.2 Result와 함께하는 복구 가능한 에러
``` rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

#### 서로 다른 에러에 대해 매칭하기
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```
* `if error.kind() == ErrorKind::NotFound`
    * 매치 가드(match guard) 라고 부름.


#### 에러가 났을 때 패닉을 위한 숏컷: unwrap과 expect

- unwrap
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
* `Result` 값이 `Ok`라면 값 반환하고 `Err`라면 `panic!`

- expect
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```
* `unwrap`과 유사하지만 `Err`이면 `panic!`과 함께 에러 메시지를 담을 수 있음.

#### 에러 전파하기
``` rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
```


#### 에러를 전파하기 위한 숏컷: ?
``` rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // Err 시 from 함수를 통해 에러 타입 변환 후 반환.
    f.read_to_string(&mut s)?; 
    Ok(s)
}
```
* match와 '?'의 차이
- match: 함수 반환값에 맞춰 코드를 작성해주어야함.
- '?': 함수의 정의된 반환값으로 자동 반환.



#### ?는 Result를 반환하는 함수에서만 사용될 수 있습니다
``` rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```
* main 함수는 반환 타입이 `()` 이므로 **컴파일 에러** 발생함.

### 9.3 panic!이냐, panic!이 아니냐, 그것이 문제로다
#### 컴파일러보다 여러분이 더 많은 정보를 가지고 있을 때
``` rust
use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();
```
* 위와 같이 `Err` variant가 절대 발생하지 않는다는 것을 확신할 수 있을 때만 `unwrap` 사용 권장.

#### 에러 처리를 위한 가이드라인


#### 유효성을 위한 커스텀 타입 생성하기


## 챕터 10 제네릭 타입, 트레잇, 그리고 라이프타임
- 발표: 이명수


### 10.1 제네릭 데이터 타입
* generic을 사용하여 중복된 코드를 합칠 수 있다.
* C++의 템플릿과 유사
```rust
fn largest<T>(list: &[T]) -> T {
```

* 제네릭은 컴파일 타임에 타입이 정해지기때문에 성능 상 별 차이가 없다.

### 10.2 트레잇: 공유 동작을 정의하기
* 다른 언어의 interface와 유사

```rust
pub trait Summarizable {
    fn summary(&self) -> String;
}
```

* 오버라이딩된 구현으로부터 기본 구현을 호출하는 것은 불가능하다는 점을 기억해주세요.

#### 트레잇 바운드
*  제네릭 타입에 제약을 가해서 이 제네릭 타입에 따라 특정 트레잇을 구현해 필요한 동작을 수행할 수 있도록 해줄 수 있다. 

### 10.3 라이프타임을 이용한 참조자 유효화
* 러스트에서 모든 참조자는 라이프타임(lifetime) 을 갖는데, 이는 해당 참조자가 유효한 스코프입니다. 

* 라이프타임의 주목적은 댕글링 참조자(dangling reference)를 방지하는 것.

#### 라이프타임 명시 문법
``` rust
 &i32        // a reference
 &'a i32     // a reference with an explicit lifetime
 &'a mut i32 // a mutable reference with an explicit lifetime
```
#### 함수 시그니처 내의 라이프타임 명시
``` rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### 라이프타임 생략 규칙
```
1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 갖습니다. 바꿔 말하면, 하나의 파라미터를 갖는 함수는 하나의 라이프타임 파라미터를 갖고: fn foo<'a>(x: &'a i32), 두 개의 파라미터를 갖는 함수는 두 개의 라이프타임 파라미터를 따로 갖고: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 이와 같은 식입니다.

2. 만일 정확히 딱 하나의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입됩니다: fn foo<'a>(x: &'a i32) -> &'a i32.

3. 만일 여러 개의 입력 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 &self 혹은 &mut self라고 한다면, self의 라이프타임이 모든 출력 라이프타임 파라미터에 대입됩니다. 이는 메소드의 작성을 더욱 멋지게 만들어줍니다.
```

## 다음 주: 10장 테스트 
## 과제: 첼시를 도와줘! (https://www.acmicpc.net/problem/11098)

## 회고
- 이지한: 이번에는 과제랑 스터디에 좀 충실하지 못했던 것 같습니다. 다음에는 좀더 열심히 준비해보겠습니다.
- 김기덕: 과제하면서 소유권이나 라이프타임 개념이 함께 적용되다 보니 너무 어려웠습니다...ㅠ
- 유병조: 라이프사이클 개념이 아리까리하네요. 다시 읽어봐야겠어요ㅠ
- 이명수: 자꾸 까먹네요.. 다시 1장부터 봐야겠어요 ㅎㅎ
- 이재현: 라이프사이클을 명시한다는 개념이 잘 와닿지 않아서 힘들었던 것 같네요.. 
- 정연집: 이번에 과제하면서 전에 했던 개념들을 까먹다 보니 한참헤맸네요..ㅠㅠㅠ
- 허신: 내용 이해가 완전히 되지 않은 상태에서 서기 하려니 너무 힘드네요 ㅠㅠ
