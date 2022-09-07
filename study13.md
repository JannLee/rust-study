# 13차 러스트 스터디

## 담당
- 발표: 유병조 님
- 리뷰: 허신 님
- 서기: 정연집 님

## 과제 리뷰
### [디자인패턴 만들어보기](https://github.com/JannLee/rust-study)


## 18. 값의 구조와 매칭되는 패턴
- 패턴은 단순하거나 복잡한 타입의 구조에 값들을 비교하기 위한 러스트의 특별한 문법
- 패턴의 조합
    - 리터럴 값(Literals)
    - 분해한 배열(Array), 열거형(Enum), 구조체(Struct), 튜플(Tuple)
    - 변수(Variable)
    - 와일드카드(Wildcard)
    - 임시 값(Placeholders)
### 18.1. 패턴이 사용될 수 있는 모든 곳
#### match 갈래
- match표현은 match 키워드, 대응 시킬 값, 갈래들로 이루어지고, 각 갈래는 갈래를 나타내는 패턴, 해당 패턴에 값이 대응 될때 실행할 표현으로 구성됩니다.
``` rust
match 값 {
    패턴 => 표현,
    패턴 => 표현,
    패턴 => 표현,
}
```
- match 표현을 사용하기 위해 지켜야할 사항
    - match 표현에 대응 시킬 값이 가질 수 있는 모든 경우의 수를 빠짐 없이 표현해야 함.
- '_' 키워드
    - 명시 하지 않은 값들을 무시할 때 유용

#### if let 조건 표현
- 'if let', 'else if', 'else if let' 표현을 섞어서 사용 가능
- if let표현의 단점
    - match표현과 다르게 컴파일러가 해당 구문이 모든 경우를 다뤘는지 판단하지 않는다는 점
- match와 차이점
    - 'if let'의 경우, 모든 경우수를 매칭시키지 않아도 된다.
``` rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

```

#### while let 조건 루프
- 조건에 만족하면 계속 반복하게 하는 구문
``` rust
#![allow(unused)]
fn main() {
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}

```

#### for 루프
- 패턴 예시
    - for x in y에서 x가 패턴
``` rust
#![allow(unused)]
fn main() {
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
} // 튜플을 분해할 수 있음
```
- enumerate 메소드
    - 해당 반복자를 자신의 값과 값의 index를 포함한 튜플들을 순회
#### let 구문
- let PATTERN = EXPRESSION;
- 변수명 자체가 하나의 패턴이 된다. 
```rust
let PATTERN = EXPRESSION;
```
- 튜플 매칭시 갯수가 서로 맞아야 한다.
- _를 사용해 값을 무시할 수 있음

#### 함수의 매개변수
- 함수 매개변수는 패턴
``` rust
fn foo(x: i32) { // x가 패턴
    // code goes here
}


```
- 클로저도 동일하게 사용이 가능하다

### 18.2. 반증 가능성: 패턴의 매칭이 실패할 수도 있는 경우

- 반증 가능 패턴
    - 주어진 값에 대응이 실패할 수 있는 패턴.
    - ex) if let Some(x) = a_value; // a_value가 None일때 대응하지 못해 실패

- 반증 불가 패턴
    - 주어진 어떠한 값에도 대응되는 패턴 (절대 실패할 수 없는 패턴)
    - ex) let x = 5; 
- 'let', 'for'구문
    - 반증불가한 패턴만 허용
- 'if let', 'while let'
    - 반증 가능 패턴만 허용
- let 에서 반증 불가 패턴 사용시,
``` rust
let Some(x) = some_option_value; // => if let을 사용
```

### 18.3. 패턴 문법의 모든 것

#### 18.3.1 리터럴 매칭
- 패턴과 리터럴을 직접 매칭하는 코드
``` rust
#![allow(unused)]
fn main() {
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
}

```

#### 18.3.2 명명 변수 매칭

- 갈래에서 y 변수를 새로 만들어 기존의 것이 가려지도록 한 match 표현
```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),    // 정의된 변수 x 와 매칭되지 않으니, 해당 코드는 실행되지 않고 넘어갑니다.
        Some(y) => println!("Matched, y = {:?}", y), // 기존의 y가 아닌 새 변수 y이며 x의 값인 5가 대입됨
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y); // 여기서 y는 기존 변수 y이므로 y = 10
}

// 출력
// Matched, y = 5
// at the end: x = Some(5), y = 10
```

#### 18.3.3 다중 패턴
- 여러 패턴 패칭
    - '|'키워드: or를 의미
```rust=
#![allow(unused)]
fn main() {
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
}

```

#### 18.3.4 `...`를 이용한 값의 범위 매칭

- `1 | 2 | 3 | 4 | 5` 대신 `1 ... 5`로 간편하게 표현 가능
- 숫자 값이나 char 값에만 사용할 수 있습니다.
```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```
- char 값 매칭 예제
``` rust
#![allow(unused)]
fn main() {
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
}
```
#### 18.3.5 값을 해체하여 분리하기
- 패턴을 이용해 구조체, 열거형, 튜플, 참조자 등의 값들을 해체(destructuring)

##### 구조체 해체하기
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };    //p 변수의 필드인 x 와 y 에 각각 대응되는 a 와 b 변수를 생성

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

```
- 패턴내 변수의 이름이 꼭 구조체 내 필드명과 일치할 필요 없음. 하지만 기억하기 쉽도록 필드명을 일치시키는 것이 일반적이다.

##### 열거형 해체
- 열거형을 해체하기위한 패턴은 해당 열거형에 내장된 데이터의 정의 방식이 일치해야한다.
- 다른 종류의 값들을 갖는 열거형 variant 해체
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

// 출력
// Change the color to red 0, green 160, and blue 255
```

##### 참조자 해체
- 패턴내에서 '&'키워드를 이용해 참조자를 해체해야함.
```rust
#![allow(unused)]
fn main() {
struct Point {
    x: i32,
    y: i32,
}

let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];

let sum_of_squares: i32 = points
    .iter()
    .map(|&Point { x, y }| x * x + y * y)
    .sum();
}

```
-  &Point { x, y } 에서 & 를 뺀다면 타입 불일치(type mismatch) 오류가 발생
```result
error[E0308]: mismatched types
  -->
   |
14 |         .map(|Point { x, y }| x * x + y * y)
   |               ^^^^^^^^^^^^ expected &Point, found struct `Point`
   |
   = note: expected type `&Point`
              found type `Point`

```

-  
##### 구조체와 튜플 해체
``` rust
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

#### 18.3.6 패턴 내에서 값 무시하기
- 패턴 내 값을 _, ..을 사용해 무시할 수 있다.

##### _ 를 이용해 전체 값 무시하기
- '_'언더스코어 와일드카드 패턴
    - 어떤 값과도 매치되지만 값으로 바인드(bind) 하지는 않는 것으로 사용
- 함수 시그니처에서 _ 사용하기
- 사용하지 않는 매개 변수에 _를 사용하면 컴파일러가 경고를 하지 않음 
- 언더 스코어가 아닌 이름을 사용할 경우 경고.
```rust
fn foo(_: i32, y: i32)
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4); // 첫번째 인자로 전달된 값인 3 을 완벽히 무시
}

// 출력
// This code only uses the y parameter: 4
```

##### 중첩된 _ 를 이용해 값의 일부 무시하기
- _ 를 다른 패턴 내에 사용해서 값의 일부를 무시할 수도 있습니다
- 값의 일부를 테스트 하려는데 해당 코드에서 그 외의 나머지 부분은 필요하지 않을때 이 기능을 사용할 수 있습니다.
```rust
#![allow(unused)]
fn main() {
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
}
// 실행 결과 
// Can't overwrite an existing customized value 와 setting is Some(5) 를 출력
```

##### 언더스코어로 시작하는 이름을 이용해 쓰이지 않는 변수 무시하기
- 프로토타입을 만드는 중이거나 프로젝트를 막 시작했을 때와 같이 아직 사용하진 않아도 미리 변수를 만들어 두는 것이 유용할 때도 있습니다.
- _는 값을 바인드하지 않고 
- _변수명은 값을 바인드한다. 

```rust
fn main() {
    let _x = 5;
    let y = 10;
}

```
##### .. 를 이용해 값의 나머지 부분 무시하기
- 값의 일부만 사용하고 나머지는 무시하기 위해 '..' 구문을 사용
- .. 패턴은 우리가 패턴에서 명시하지 않은 값의 나머지 부분을 모두 무시
```rust
#![allow(unused)]
fn main() {
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
}

```
- ..을 사용할 땐 모호하지 않아야 한다. 
``` rust
fn main() {//모호한 예시
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```


#### 18.3.7 ref 와 ref mut 를 이용해 패턴 내에서 참조자 생성하기
- 가변 참조자 사용
    - 매치된 패턴 내에서 값을 변경하기 위해 가변 참조자를 생성하려면 &mut 대신 ref mut 을 사용해야 합니다
    - & 는 새 참조자를 생성하는 것이 아닌 이미 존재하는 가변 참조자를 매치시키는데 사용

#### 18.3.8 매치 가드를 이용한 추가 조건
- 매치 가드(match guard) 
    - match 갈래 뒤에 추가로 붙는 if 조건으로, 이것이 있을 경우 패턴 매칭과 해당 조건이 모두 만족되어야 해당 갈래가 선택됨.
```rust
#![allow(unused)]
fn main() {
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
}

```
- match 표현의 패턴 내에서 새로 생성한 변수가 기존에 있던 match 바깥의 변수를 가려버려서 기존의 변수를 테스트 할 수 없다. 이문제를 해결하기 위한 예시는 아래와 같다.
```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

```

```rust
#![allow(unused)]
fn main() {
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
}

```
- 첫 번째 갈래의 패턴에 매치
- 두 번째 갈래로 이동하고 매치되며, 프로그램은 no 를 출력

#### 18.3.9 @ 바인딩
- at 연산자인 @ 
    - 해당 값이 패턴과 매치되는지 확인하는 동시에 해당 값을 갖는 변수를 생성할 수 있도록 해줍니다.
```rust
#![allow(unused)]
fn main() {
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3...7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
}
```
- 3...7 범위 앞에 id_variable @ 를 지정하는 것으로 값이 범위 패턴과 매치되는지 테스트하면서 해당 값을 캡쳐할(capturing) 수 있습니다.

#### 정리

- 러스트의 패턴은 다른 종류의 데이터를 구별하는데 굉장히 유용
- match 표현 내에서 패턴을 사용하면 러스트가 여러분의 패턴이 모든 가능한 값을 커버할 수 있다는 것을 보장

## 차주 일정
- 스터디 장: 19. 고급 기능들
- 날짜: 2022-09-07
- 과제: 18장에서 배운 것들을 활용하여 자신이 지금까지 수행한 과제 중 하나 리팩토링하기

## 벌금현황
- 이지한: 0
- 김기덕: 0
- 유병조: 0
- 이명수: 0
- 이재현: 0
- 정연집: 10,000
- 허신: 10,000

## 회고
- 이지한: 
- 김기덕: [github 다양한 디자인패턴 저장소 목록](https://github.com/search?o=desc&q=rust+design+pattern&s=stars&type=Repositories)을 참고해서 과제했어요! 이번장에서 공부한 내용을 좀 능숙하게 쓸 수 있으면 좋겠습니다!
- 유병조: 패턴을 사용해서 간결하게 코드를 짤 수 있을거 같아서 좋아요~
- 이명수: 싱글톤 구현하다 낙오했어요 ㅜㅜ
- 이재현: 지금까지 warning때 자주 봤던 _나 for문 같은데서 썼던 ..를 패턴이라는 개념으로 정리가 되서 좋았던 것 같습니다 
- 정연집: 많이 쓸 것 같은 매칭에 대해 자세히 배운 것 같아서 좋았습니다.
- 허신: 이전에 if let 구문이 이해가 안됐었는데 오늘 17장 "패턴" 문법 통해서 잘 정리된 것 같아요.
