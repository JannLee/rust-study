# Rust 스터디 3회차

## 지난 과제 리뷰

### 1번 문제
 - 섭씨, 화씨 변환 문제 
 - 기덕님 [코드 리뷰](https://github.com/JannLee/rust-study/blob/main/GiDeokKim/chapter3/unit_convertor/src/main.rs) 

### 2번 문제
 - 피보나치 수열 출력 문제 
 - 실장님 [코드 리뷰](https://github.com/JannLee/rust-study/blob/main/jannlee/chapter3_fibonacci_numbers/src/main.rs)

### 3번 문제
 - 캐롤 가사 출력 문제
 - 명수님 [코드 리뷰](https://github.com/JannLee/rust-study/blob/main/larangms/problem/ch3/the_twelve_days_of_christmas/src/main.rs)

## 4. 소유권 이해하기
 러스트가 가비지 콜렉터 없이 메모리 안정성 보장을 하게 해줍니다. 
 
### 4.1 소유권이 뭔가요?

- 자바 등은 가비지 컬렉터를 가지고 있음
- C++ 등은 직접 할당 해제
- rust의 메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 소유권 시스템을 통해 관리됩니다. 소유권 기능들의 어떤 것도 런타임 비용이 발생하지 않습니다.

- 소유권 규칙
 1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
 2. 한번에 딱 하나의 오너만 존재할 수 있다.
 3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

- 변수의 스코프
```rust
{                      // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
    let s = "hello";   // s는 이 지점부터 유효합니다.

    // s를 가지고 뭔가 합니다.
}                      // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.

스코프 안에서 s가 등장하면, 유효합니다.
이 유효기간은 스코프 밖으로 벗어날 때까지 지속됩니다.
```

- String 타입
    - 스트링 리터럴과 차이
        - 리터럴은 불변
        - 실행파일에 직접 하드코딩됨
        - 컴파일 타임에 크기를 알 수 있다. 
    - 스트링
        - 스트링은 가변
        - 힙에 할당되므로 컴파일 타임에 메모리 크기를 알 수 없다
        - 런타임 중 메모리 할당 및 반납이 필요하다. 
- 메모리와 할당
    - 메모리는 변수가 소속되어 있는 스코프 밖으로 벗어나는 순간 자동으로 반납
    - 이때 ```drop```이라는 함수가 호출됨
    - C++의 RAII와 유사한 느낌
    
    - heap 메모리를 사용하는 데이터를 복사할 때 heap에 있는 데이터는 복사하지 않는다.
    - 컴파일 타임에 데이터의 크기를 알 수 없으므로 런타임에 할당, 반납 하는 작업이 성능 이슈가 될 수 있기 때문
    - 원본은 무효화하고, 얕은 복사를 한다. (move)
	```rust
	let s1 = String::from("hello"); // s1을 s2로 복사하였으므로 소유권이 이전됨(s1 무효화)
	let s2 = s1;
	
	println!("{}, world!", s1); // 이후 s1을 사용하려고 하면 컴파일 타음 에러 발생
	```
    
    - 깊은 복사를 원하면 clone 메소드를 사용하면 된다. 
    ```rust
	let s1 = String::from("hello");
	let s2 = s1.clone();
	
	println!("s1 = {}, s2 = {}", s1, s2);
	```
    
    - 스택에만 있는 데이터는 그냥 복사를 한다. 
    - 컴파일 타임에 데이터의 크기를 알 수 있기 때문에
    ```rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    ```
    - 러스트는 정수형과 같이 스택에 저장할 수 있는 타입에 대해 달수 있는 Copy 트레잇이라고 불리우는 특별한 어노테이션(annotation)을 가지고 있습니다.
    - 어떤 타입이 Copy 트레잇을 갖고 있다면, 대입 과정 후에도 예전 변수를 계속 사용할 수 있습니다. 
    - drop 트레잇이 구현되어 있는 타입은 copy 트레잇을 어노테이션 할 수 없다. 

- 소유권과 함수
    ``` rust
    fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다...
                                    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.

	} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
	  // 별다른 일이 발생하지 않습니다.
	
	fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
	    println!("{}", some_string);
	} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
	  // 해제되었습니다.
	
	fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
	    println!("{}", some_integer);
	} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.
    ```

- 반환 값과 스코프
    ```rust
    fn main() {
        let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
                                            // 이동시킵니다.

        let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

        let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
                                            // 이동되었고, 이 함수가 반환값을 s3으로도
                                            // 이동시켰습니다.

    } // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
      // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
      // 벗어나서 drop이 호출됩니다.

    fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
                                                 // 호출한 쪽으로 이동시킵니다.

        let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

        some_string                              // some_string이 반환되고, 호출한 쪽의
                                                 // 함수로 이동됩니다.
    }

    // takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
    fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
                                                          // 안으로 들어왔습니다.

        a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
    }
    ```
    
    - 튜플을 사용해 여러 값을 반환할 수도 있다. (예제를 위한 예제...)
    ```rust
	fn main() {
	    let s1 = String::from("hello");
	
	    let (s2, len) = calculate_length(s1);
	
	    println!("The length of '{}' is {}.", s2, len);
	}
	
	fn calculate_length(s: String) -> (String, usize) {
	    let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.
	
	    (s, length)
	}  
    ```
    
### 4.2 참조자와 빌림
- 값의 소유권을 넘기는 대신 개체에 대한 참조자(reference)를 인자로 사용할 수 있다. 
    ```rust
    fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    } // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
      // 때문에, 아무런 일도 발생하지 않습니다.
    ```
- 가변 참조자
    - 가변 참조자는 레이스 컨디션 문제를 방지하기 위해 하나만 만들 수 있다. 

- 참조자의 규칙
    1. 어떠한 경우이든 간에, 여러분은 아래 둘 다는 아니고 둘 중 하나만 가질 수 있습니다:
        - 하나의 가변 참조자
        - 임의 개수의 불변 참조자들
    2. 참조자는 항상 유효해야만 한다.

### 4.3 슬라이스
 소유권을 갖지 않는 또다른 데이터 타입은 슬라이스입니다. 

 ```rust
 let s = String::from("hello world");
 
 let hello = &s[0..5];
 let world = &s[6..11];
 ```
 
 함수 파라미터로 슬라이스 전달 
 ```rust
 fn main() {
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word(my_string_literal);
}
 ```

## 5. 연관된 데이터들을 구조체로 다루기
 구조체(struct)는 사용자들이 연관된 여러 값들을 묶어서 의미있는 데이터 단위를 정의할 수 있게 합니다.
### 5.1 구조체를 정의하고 생성하기
 
 - 튜플과는 달리 각 요소에 이름을 정할 수 있다. 
    ```rust
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    ```
 
 - 인스턴스 생성 및 필드 변경
    ```rust
	let mut user1 = User {
	    email: String::from("someone@example.com"),
	    username: String::from("someusername123"),
	    active: true,
	    sign_in_count: 1,
	};
	
	user1.email = String::from("anotheremail@example.com");
    ```
 - 함수에서 구조체의 인스턴스 초기화 
	```rust
	fn build_user(email: String, username: String) -> User {
	    User {
	        email,    // 파라미터 이름과 구조체의 필드 이름이 동일하면 초기화 구문을 생략해도된다. 
	        username,
	        active: true,
	        sign_in_count: 1,
	    }
	}
	```

 - 기존 인스턴스를 사용해 새 구조체 인스턴스 생성하기 
    ```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // email, username 외 필드는 user1 과 동일하게 초기화 
    };
    ```
 
 - 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체
    ```rust
	struct Color(i32, i32, i32);
	struct Point(i32, i32, i32);
	
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
 	```

 - 구조체 데이터의 소유권 
    구조체가 소유권이 없는 데이터의 참조를 저장하기 위해서는 추후 소개될 라이프타임을 사용해야함 
    ```rust
    struct User {
        username: &str,
        email: &str,
        sign_in_count: u64,
        active: bool,
    }

    fn main() {
        let user1 = User {
            email: "someone@example.com",
            username: "someusername123",
            active: true,
            sign_in_count: 1,
        };
    }
    ```
    ```
    error[E0106]: missing lifetime specifier
	 -->
	  |
	2 |     username: &str,
	  |               ^ expected lifetime parameter
	
	error[E0106]: missing lifetime specifier
	 -->
	  |
	3 |     email: &str,
	  |            ^ expected lifetime parameter
    ```

### 5.2 구조체를 이용한 예제 프로그램

- 최초 코드
```rust
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
```

- 튜플을 이용한 리팩토링
```rust
fn main() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

- 구조체로 의미 추가
```rust
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
```

- 파생 트레잇으로 유용한 기능 추가 
```rust 
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {}", rect1);    // 에러 발생 (포맷이 정의되어 있지 않아서)
}
```

 #[derive(Debug)] 어노테이션을 추가해서 출력하는 방법이 있다. 
 ```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
}
 ```

### 5.3 메소드 문법
 C++ 클래스 메소드와 동일한 기능 
 
```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {    // self 키워드를 사용해서 구조체 인스턴스의 소유권을 가지고온다. 
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

-> 연산자는 어디로 갔나요?
 러스트는 자동적으로 &나 &mut, 혹은 을 붙여서 object가 해당 메소드의 시그니처와 맞도록 합니다.

- 더 많은 파라미터를 가진 메소드
```rust
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

- 연관 함수
 구조체의 인스턴스 없이 호출할 수 있는 함수 
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```

## 6. 열거형과 패턴 매칭
 - 열거형은 하나의 타입이 가질 수 있는 값들을 열거 함으로써 타입을 정의할 수 있도록 합니다. 
 - Option 이라고 하는 특히 유용한 열거형을 자세히 볼 텐데, 이것은 어떤 값을 가질 수 도 있고, 갖지 않을 수 도 있습니다. 
 - match 표현식에서 패턴 매칭
 - if let 구문
 
### 6.1 열거형 정의하기
```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

V4 와 V6 variant 는 연관된 String 타입의 값을 가질 수 있다.
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

열거형은 각 variants 가 다른 타입과 다른 양의 값을 저장할 수 있다. 
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

구조체에 impl 을 사용해서 메소드를 정의한 것처럼, 열거형에도 정의할 수 있습니다. 
```rust
impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

 null 특성이 없습니다.
 이와 같이 러스트에는 null 이 없지만, 값의 존재 혹은 부재의 개념을 표현하기 위해 Option<T> 열거형을 사용합니다. 
 ```rust
 enum Option<T> {
     Some(T),
     None,
 }
 ```
    
 컴파일러가 미리 타입을 알아야하기 때문에 None 이라도 타입을 지정해주어야함
 ```rust
 let some_number = Some(5);
 let some_string = Some("a string"); 
 let absent_number: Option<i32> = None;
 ```

 Option<T> 와 T (T 는 어떤 타입이던 될 수 있음)는 다른 타입
 ```rust
 let x: i8 = 5;
 let y: Option<i8> = Some(5);
 
 let sum = x + y;    // 에러 발생
 ```
    
### 6.2 match 흐름 제어 연산자
 match의 힘은 패턴의 표현성으로부터 오며 컴파일러는 모든 가능한 경우가 다루어지는지를 검사합니다.
 if를 사용하는 경우, 해당 표현식은 부울린 값을 반환할 필요가 있습니다. 여기서는 어떤 타입이든 가능합니다. 
    
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
	
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}    // match 의 결과 값이 함수의 반환값으로 바로 반환됨. (표현식을 사용해서)
```

- 값들을 바인딩하는 패턴들
    
```rust
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
	match coin {
	    Coin::Penny => 1,
	    Coin::Nickel => 5,
	    Coin::Dime => 10,
	    Coin::Quarter(state) => {
	        println!("State quarter from {:?}!", state);
	        25
	    },
    }
}
```
    
 - Option<T>를 이용하는 매칭
    Some, None을 매칭할 수 있다. 
    
    ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
    	match x {
    	    None => None,
    	    Some(i) => Some(i + 1),
		}
	}
	
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
    ```
    
    - 매치는 하나도 빠뜨리지 않음
    ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),    // None이 없어서 컴파일 에러 발생
        }
    }
    ```
    
    - _변경자 (placeholder)
    ```rust
    let some_u8_value = 0u8;
	match some_u8_value {
	    1 => println!("one"),
	    3 => println!("three"),
	    5 => println!("five"),
	    7 => println!("seven"),
	    _ => (),    // 1, 3, 5, 7 외 나머지 (default)
	}
    ```
    
### 6.3 if let을 사용한 간결한 흐름 제어

if와 let을 조합하여 하나의 패턴만 매칭 시키고 나머지 경우는 무시하는 값을 다룰 수 
    
```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

를 아래와 같이 변경할 수 있다. 
    
```rust
if let Some(3) = some_u8_value {
println!("three");
}
```

if let과 else 표현식도 사용이 가능하다.
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
    
## 다음 스터디 준비
    
### 과제
- https://www.acmicpc.net/workbook/view/567
- 완전 제곱수 문제 rust로 풀어보기
    
### 범위
- 8장 일반적인 컬렉션까지 공부해오기
    
## 회고    
- 이지한: 범위가 다른 때 비해 좀 많았는데, 꽤 재미있었습니다. 소유권 개념 생각보다 쉬워서 아쉬웠고(?), enumerator는 생각보다 더 관심이 많이 가네요.
- 이명수: 놀다가 다시보니 해깔리네요 ㅎㅎㅎ   
- 허신: 컴파일러가 정말 똑똑한 것 같아요! ㅎㅎ
- 이재현: 소유권 개념이 어려울 줄 알았는데 생각보다 c++와 비슷해서 익숙하고 쉬워서 다행이다 싶었고.. 서기는 힘든거였네요 ㅠ      
- 유병조: 컴파일 타임에 해주는게 많은거 같네요. 신기한 문법이 많아서 재미있어요~
- 김기덕: 자동 참조 및 역참조 기능이 있어서 -> 연산자가 없는데, 이후에 소유권이랑 같이 다루면 생각을 많이 해야할 것 같습니다. 
- 정연집: 잘못 이해한 내용 바로바로 이야기해서 좋았습니다. 라이너 플러그인 잘쓰겠습니다~!
    
