# Rust 스터디 4회차

## 과제 리뷰
### 완전 제곱수
 유병조 님 : [코드 리뷰](https://github.com/JannLee/rust-study/blob/main/cola314/baekjoon/%EC%99%84%EC%A0%84%EC%A0%9C%EA%B3%B1%EC%88%98_1977/src/main.rs)

## 7. 모듈 
- mod라는 키워드로 선언
- 함수, 타입, 상수를 가짐
- 기본 private
- pub 키워드로 공개 가능(안하고 사용 안하면 경고 발생)
- use 키워드는 모듈이나 모듈 내의 정의들을 스코프 안으로 가져와서 이들을 더 쉽게 참조할 수 있게 해줌 

### 7.1 mod와 파일 시스템
- 바이너리 크레이트가 아닌 라이브러리 크레이트를 만들 수 있다.
```
$ cargo new communicator --lib
$ cd communicator
```
- 생성 시 테스트 코드가 자동으로 만들어진다. 
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

- mod 키워드로 모듈을 정의할 수 있다. 
```rust
mod network {
    fn connect() {
    }
}
```
- 외부에서 접근하기 위해서는 ::를 사용한다.
```rust
network::connect();
```
- 나란히 다른 namespace에 동일한 이름의 메소드 정의 가능
- 나란히 둔 경우의 계층 구조
```
communicator
 └── network
 └── client
```

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```
- mod를 중첩할 수 있다. 
```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}

network::client::connect
```
- 중첩 시 계층 구조
```
communicator
 └── network
     └── client
```

#### 모듈을 다른 파일로 옮기기 

한 파일에 있는 아래 모듈 구조를 여러 파일로 분할하기 
```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

- mod 들을 파일로 분할 
``` rust
// Filename: src/lib.rs
mod client;
mod network;
```

```rust
// Filename: src/client.rs
fn connect() {
}
```

- 파일을 분할할 때에는 서브 mod의 계층 구조와 파일 구조가 동일해아한다.

```rust
// Filename: src/network.rs
// or        src/newwork/mod.rs 
fn connect() {
}

mod server;
```

``` rust
// Filename: src/network/server.rs
// sub 모듈의 계층을 맞춰주어야함. src/server.rs 로 하면 빌드 에러
fn connect() {
}
```

- 위 까지 진행해도 아래와 같이 warning이 발생하는데, 이는 아래 pub를 사용해 제거할 수 있다. 
```
warning: function is never used: `connect`, #[warn(dead_code)] on by default
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

#### 모듈 파일 시스템의 규칙
- `foo`라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, `foo.rs`에 `foo`에 대한 선언을 집어넣어야 합니다.
- `foo`가 서브모듈을 가지고 있다면, `foo/mod.rs`에 foo에 대한 선언을 집어넣어야 합니다.

### 7.2 pub으로 가시성 제어하기
모듈이 기본적으로 private 이므로 내부에서 사용하지 않으면 사용되지 않아서 warning을 발생시킨다. 

- 아래 경고 발생
```
warning: function is never used: `connect`, #[warn(dead_code)] on by default
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

- 생성한 라이브러리를 가져오기 위해 extern crate 명령어를 사용한다. 
```rust
// Filename: src/main.rs
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

#### 함수를 공개로 만들기
- pub 키워드로 함수를 공개할 수 있음
```rust
// Filename: src/network/mod.rs
pub mod client;

mod network;
```
- 아래와 같은 에러 발생
```
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
- 메소드가 private여서 발생하는 것이므로 pub로 메소드 지정
- 메소드 뿐만 아니라 mod도 public으로 변경해주어야함

```rust
// Filename: src/lib.rs
pub mod client;

pub mod network;
```

#### 비공개 규칙(Privacy Rules)
1. 아이템이 공개라면, 부모 모듈의 어디에서건 접근 가능
2. 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능


#### 비공개 예제(Privacy Examples)
```rust
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {} // 에러 발생

    mod inside {
        pub fn inner_function() {} // 에러 발생

        fn secret_function() {} // 에러 발생
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

### 7.3 use로 이름 가져오기
#### use를 이용한 간결한 가져오기
- 러스트의 use 키워드는 여러분이 스코프 내에서 호출하고 싶어하는 함수의 모듈을 가져옴으로써 긴 함수 호출을 줄여줍니다.
```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of; // use로 스코프 내에서 사용하려는 것을 지정

fn main() {
    a::series::of::nested_modules();
}
```

- 열거형 또한 모듈과 비슷한 일종의 이름공간을 형성하고 있기 때문에, 열거형의 variant 또한 use를 이용하여 가져올 수 있습니다.
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow}; 

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green; // 위 use 에서 Green을 지정하지 않아 직접 지정해주어야함
}
```

#### *를 이용한 모두(glob) 가져오기
- 이름공간 내의 모든 아이템을 가져오기 위해서는 * 문법을 이용할 수 있습니다. 예를 들면:
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*; // 모든 아이템 가져오기 

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

#### super를 사용하여 부모 모듈에 접근하기

```rust
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    // use super::client; 도 사용 가능 
    
    #[test]
    fn it_works() {
        client::connect();  // 컴파일 오류 
                            // 원인은 경로가 항상 현재 모듈을 기준으로 상대적인데, 여기는 test이기 때문입니다.
                            // 따라서 super::client::connect(); 를 사용해야함 
                            // ::client::connect(); 는 책에 된다고 나와있으나 되지않음...
    }
}
```
- 모듈 계층 구조
```
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

## 8. 컬렉션 
표준 라이브러리가 제공해주는 다른 종류의 컬렉션에 대해 알고 싶으시면, the documentation를 봐 주세요.

### 8.1 벡터
#### 새 벡터 만들기
```rust=
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
```

#### 벡터 갱신
```rust=
let mut v = Vec::new(); // 러스트는 아래 데이터 타입을 추론하므로 명시할 필요가 없음

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

#### 벡터 요소 읽기 
```rust=
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[100]; // 벡터의 끝을 넘어서는 요소에 접근하는 시도를 하면 프로그램이 죽게끔 하는 치명적 에러를 발생
let third: Option<&i32> = v.get(100); // 범위를 벗어나면 None을 반환해서 해당 케이스의 로직을 정의할 수 있음
```

#### 벡터 내의 값들에 대한 반복처리
- 벡터의 각 요소를 변경하기 위해서는 역참조 연산자 (*)를 사용하여 값을 얻어야 합니다.
```rust=
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

#### 열거형을 사용하여 여러 타입을 저장하기
```rust=
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
### 8.2 스트링
 스트링이 컬렉션 장에 있는 이유는 스트링이 바이트의 컬렉션 및 이 바이트들을 텍스트로 통역(interpreted)할때 유용한 기능을 제공하는 몇몇 메소드로 구현되어 있기 때문입니다.

#### 스트링 생성하기 
```rust
let mut s = String::new();

let s = "initial contents".to_string();
let s = String::from("initial contents");
```

#### 스트링 갱신하기
```rust=
// push_str 사용
let mut s = String::from("foo");
s.push_str("bar");

// + 사용
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요

// format! 매크로 사용 
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

#### 스트링 내부의 인덱싱
rust의 스트링은 인덱싱 문법을 제공하지 않는다. 
```rust=
let s1 = String::from("hello");
let h = s1[0]; // 컴파일 에러 
```
- 기능을 제공하지 않는 이유
    - 스트링의 바이트들 안의 인덱스는 유효한 유니코드 스칼라 값과 항상 대응되지는 않기 때문에  
    - 인덱스 연산이 언제나 상수 시간(O(1))에 실행될 것으로 기대받기 때문입니다. 그러나 이를 보장해줄 수 없음

#### 스트링 슬라이싱하기
```rust=
let hello = "Здравствуйте";

let s = &hello[0..4]; // => “Зд”
let s = &hello[0..1]; // -> 에러
```

#### 스트링 내에서 반복적으로 실행되는 메소드
- chars 메소드 : 개별적인 유니코드 스칼라 값을 반환
```rust=
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```
```
न
म
स
्
त
े
```

- bytes 메소드 : 가공되지 않은 각각의 바이트를 반환
```rust=
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```
```
224
164
168
224
// ... etc
```

### 8.3 해쉬맵

#### 해쉬맵 생성
```rust
use std::collections::HashMap; // 사용 빈도가 낮아 자동으로 가져오지 않기때문에 추가해주어야함. 

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- 이미 존재하는 Vec와 collect 메소드를 사용해서도 생성이 가능하다. 
```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

#### 해쉬맵과 소유권
- i32와 같이 Copy 트레잇을 구현한 타입에 대하여, 그 값들은 해쉬맵 안으로 복사됩니다.
- String과 같이 소유된 값들에 대해서는, 값들이 이동되어 해쉬맵이 그 값들에 대한 소유자가 될 것입니다

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name과 field_value은 이 지점부터 유효하지 않습니다.
// 이들을 이용하는 시도를 해보고 어떤 컴파일러 에러가 나오는지 보세요!
```

#### 해쉬맵 내의 값 접근하기
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

#### 해쉬맵 갱신하기
- 값을 덮어쓰기
```rust=
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

- 키에 할당된 값이 없을 경우에만 삽입하기
```rust=
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```
- 예전 값을 기초로 값을 갱신하기
```rust=
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

#### 해쉬 함수
- 기본적으로, HashMap은 서비스 거부 공격(Denial of Service(DoS) attack)에 저항 기능을 제공할 수 있는 암호학적으로 보안되는 해쉬 함수를 사용합니다.
- 여러분의 코드를 프로파일하여 기본 해쉬 함수가 여러분의 목표에 관해서는 너무 느리다면, 다른 해쉬어(hasher) 를 특정하여 다른 함수로 바꿀 수 있습니다.

## 과제 (8장)
- 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 평균값(mean, average), 중간값(median, 정렬했을 때 가장 가운데 위치한 값), 그리고 최빈값(mode, 가장 많이 발생한 값; 해쉬맵이 여기서 도움이 될 것입니다)를 반환해보세요.

- 스트링을 피그 라틴(pig Latin)으로 변경해보세요. 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 “ay”를 붙이므로, “first”는 “irst-fay”가 됩니다. 모음으로 시작하는 단어는 대신 끝에 “hay”를 붙입니다. (“apple”은 “apple-hay”가 됩니다.) UTF-8 인코딩에 대해 기억하세요!

- 해쉬맵과 벡터를 이용하여, 사용자가 회사 내의 부서에 대한 피고용인 이름을 추가할 수 있도록 하는 텍스트 인터페이스를 만들어보세요. 예를들어 “Add Sally to Engineering”이나 “Add Amir to Sales” 같은 식으로요. 그후 사용자가 각 부서의 모든 사람들에 대한 리스트나 알파벳 순으로 정렬된 부서별 모든 사람에 대한 리스트를 조회할 수 있도록 해보세요.

## 회고 

- 이지한: 스터디 진행 방식이 뭔가 좀.. 아쉬운 듯 한데..(서기가 제일 고생한다거나...) 개선 방법을 고민해봐야 겠습니다.
- 이재현: 컬렉션을 해보니 이것저것 문제를 조금 풀어보고 싶었는데 마침 과제가 많아서 재밌을 것 같네요, 그리고 서기는 오늘도 힘들었어요 ㅋㅋㅋ
- 유병조: String 사용방법이 좀 익숙하지가 않네요. 컬렉션에서의 소유권 개념도 헷갈려요 
- 정연집: 소유권 너무 어려워요 조금더 익숙해져야 할것같아요
- 허신: 특히 과제 풀면서 많은 공부가 된 것 같습니다. 이번 과제도 기대되네요 ㅎㅎ
- 김기덕: 아직 애매하게 아는 것들이 많아서 좀 더 공부를 해야할 것 같습니다!
- 이명수: 서기가 안걸려서 다행이네요 ㅎㅎㅎ
