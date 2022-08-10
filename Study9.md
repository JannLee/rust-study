# 9차 러스트 스터디

## 담당
- 발표: 유병조
- 리뷰: 이지한
- 서기: 정연집

## 과제 리뷰
### [얼마?](https://www.acmicpc.net/problem/9325)
이지한 실장님 : [코드리뷰]()



## 14. Cargo와 Crates.io 더 알아보기
14장에서 다룰 내용은 아래와 같다.
- 릴리즈 프로필을 이용해 빌드 커스터마이징하기
- crates.io 에 라이브러리 배포하기
- 대규모 작업을 위한 작업공간 구성하기
- crates.io 에서 바이너리 설치하기
- 커스텀 명령어로 Cargo 확장하기


### 14.1. 릴리즈 프로필을 이용해 빌드 커스터마이징하기
- Cargo의 메인 프로필
    ``` toml
    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3
    ```
- 프로필 설정법
    - [profile.*]키워드를 통해 가능
    - opt-level: 최저적화 수치이며 0~3사이값을 가짐.
- [공식문서 Profile 설명 페이지](https://doc.rust-lang.org/cargo/reference/profiles.html)


### 14.2. Crates.io에 크레이트 배포하기
 * Crates.io에 크레이트를 배포해 코드를 다른 사람들과 공유 할 수도 있습니다.
#### 유용한 문서화 주석 만들기
- 문서화 주석
    - 키워드: '///'
    - 마크 다운 문법을 사용할 수 있다. 

    - 자주 사용되는 문서화 주석
    ```
	 * Panics: 문서화된 기능이 패닉을 일으킬 수 있는 시나리오입니다. 함수를 호출하는 사람들에게 "프로그램이 패닉을 일으키지 않게 하려면 이러한 상황에서는 이 함수를 호출하지 않아야 합니다" 라는 내용을 알려줍니다.
	 
     * Errors: 해당 함수가 Result 를 반환할 경우에는 발생할 수 있는 에러의 종류와 해당 에러들이 발생하는 조건을 설명해 주어서 호출하는 사람이 여러 에러를 여러 방법으로 처리할 수 있도록 해야합니다.
	 
     * Safety: 함수가 안전하지 않을(unsafe) 경우에 (19장에서 다루는 내용입니다) 왜 이 함수가 안전하지 않은지와 이 함수가 호출하는 사람에게 지키길 기대하는 불변성에 대해 알려주는 구절이 있어야 합니다.
	 ```
#### 테스트로서의 문서화 주석
- 문서화 주석내의 test코드 실행
    - 키워드: cargo test
    - 테스트 실행을 통해 사용하려는 라이브러리가 정상인지 체크를 하고 사용할 수 있다.


#### 주석을 포함하는 항목을 문서화하기
 - 키워드: '///'
 - [공식 문서화 안내 페이지](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)

#### pub use 를 이용해 공개 API 를 편리한 형태로 export 하기
- export함수 가져오기
    - pub use를 통해 쉽게 가져올 수 있다.

	``` rust
	//! # Art
	//!
	//! A library for modeling artistic concepts.
	
	pub use kinds::PrimaryColor;
	pub use kinds::SecondaryColor;
	pub use utils::mix;
	
	pub mod kinds {
	    // --snip--
	}
	
	pub mod utils {
	    // --snip--
	}
	```
- 크레이트 문서화
    - 문서 보기 명령: cargo doc
    - 첫화면에 보여주기: pub use

#### 새 크레이트에 Metadata 추가하기
 - crate.io에 접속해 id를 만들고 api 키를 받은 뒤 로그인 하고 올리면 된다. 
 ```
 $ cargo login abcdefghijklmnopqrstuvwxyz012345
 ```
 - name, license를 필수로 입력해주어야 한다.
 - SPDX 에 없는 license를 사용하는 경우
     1. 해당 license의 텍스트를 파일 생성
     2. 자신의 프로젝트에 해당 파일을 포함
     3. license 대신 license-file을 추가해 해당 파일의 이름 넣기
 ```
 [package]
 name = "guessing_game"
 license = "MIT"
 ```
 
 - Crates.io에 배포하기
 ```
 cargo publish
 ```

#### cargo yank 를 이용해 Crates.io 에서 버전 제거하기
- `cargo yank`: 특정 버전을 종속성을 막아주는 기능.
- yank의미
    - Cargo.lock 을 가진 모든 프로젝트는 문제가 없을 것이며, 추후에 새로 생성될 Cargo.lock 파일은 끌어내려진 버전을 사용하지 않을 것
- `cargo yank --vers --undo`를 통해 yank취소 가능

### 14.3. Cargo 작업공간
함께 개발된 여러개의 관련된 패키지를 관리하도록 함.

#### 작업공간 생성
- 작업 공간: 동일한 Cargo.lock 과 출력 디렉토리를 공유하는 패키지들의 집합

 ##### add 작업 공간에 adder, add_one 2개의 작업공간 만들기
 ```
 $ mkdir add
 $ cd add
 ```

 ``` toml
// Filename: add\\Cargo.toml
 [workspace]

members = [
    "adder",
    "add-one",
]
 ```
 
 ```
 $ cargo new --bin adder
 $ cargo new add-one
 ```
 
 - 작업공간 구조
 ```
 ├── Cargo.lock
 ├── Cargo.toml
 ├── add-one
 │   ├── Cargo.toml
 │   └── src
 │       └── lib.rs
 ├── adder
 │   ├── Cargo.toml
 │   └── src
 │       └── main.rs
 └── target
 ```

 ##### adder 에서 add_one에 있는 함수 사용하기 

 ```
// Filename: adder/Cargo.toml
 
 [dependencies]
 add-one = { path = "../add-one" }
 ```
 
 ```
 extern crate add_one;

 fn main() {
     let num = 10;
     println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
 }
 ```
- cargo run -p adder를 통해 작업공간내에서 특정 크레이터를 실행할 수도 있다.

#### 작업공간의 외부 크레이트 의존성 갖기
 - Cargo.toml의 내부에 '[dependencies]' 절에 추가 크레이트를 작성한뒤, 코드내에 'extern crate [crate명]'을 통해 사용.
 - 작업공간 최상위 Cargo.lock에 종속성에 대한 정보가 포함되어 서로 다른 크레이트 간 외부 크레이트 호환성을 보장한다.

#### 작업공간에 테스트 추가하기
 - 크레이트 함수에 대한 테스트 코드 추가하면 'cargo test'실행 시,동작한다.
 - 특정 크레이트 테스트 코드실행을 위해서 -p옵션을 명시한다.
 - 작업공간내 여러 크레이터 배포 시, 각각을 'cargo publish'해야한다.


### 14.4. cargo install을 이용해 Crates.io에서 바이너리 설치하기
 - 외부 툴 같은 바이너리를 설치하는 방법
     - cargo install ripgrep 
 - 따로 설정을 건들지 않으셨다면 $HOME/.cargo/bin 폴더에 저장

### 14.5. 커스텀 명령어로 Cargo 확장하기
- 커스텀 명령어: $PATH 내 어떤 바이너리의 이름이 cargo-something으로 되어있으면 확장명령이 가능하다.
- 'cargo --list' 명령어로 커스텀 명령어들을 볼 수 있다.

## 차주 일정
- 스터디 장: 15장
- 날짜: 8월 10일
- 과제: 플러그(https://www.acmicpc.net/problem/2010)

## 벌금현황
- 이지한: 0
- 김기덕: 0
- 유병조: 0
- 이명수: 0
- 이재현: 0
- 정연집: 0
- 허신: 10,000

## 회고
- 이지한: 과제 문제가 너무 쉬운 것 같음. 다음에는 [이런 곳](https://www.acmicpc.net/workbook/view/6783)도 고민해보기
- 김기덕: 이번 장 공부하면서 나중에 수학 관련 크레이트 만들어 보고 싶다는 생각을 했습니다~!
- 유병조: 라이브러리 만들어서 올려보고싶네요~
- 이명수: 과제로 gui쪽 해보는 것도 재미있을 거 같아요~~
    > [name=이지한] https://www.youtube.com/watch?v=sDLrNAB7neY 요런 영상이 있네영


- 이재현: 작업 공간을 생성하고, 라이브러리를 깔끔하게 정리해보고 싶었는데 이번에 해당 내용이 나와서 좋았어요 그리고 gui 과제도 재밌을 것 같아요
- 정연집: 나중에 크레이트 꼭 만들어보고 싶다는 생각이 들었습니다~
- 허신: workspace 유용할 것 같아요, [코딩 테스트 준비 - 기초](https://www.acmicpc.net/workbook/view/9380) 이쪽 문제 괜찮을 것 같아요!
