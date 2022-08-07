# 8차 러스트 스터디

## 담당
- 발표: 김기덕
- 리뷰: 정연집
- 서기: 이재현

## 과제 리뷰
### [24](https://www.acmicpc.net/problem/1408)
정연집 님 : [코드리뷰](https://github.com/JannLee/rust-study/blob/main/Yeon-Jib/TwentyFour/src/main.rs)

## Chapter13_함수형 언어의 특성들: 반복자들과 클로저들
 - 함수형 프로그래밍에 많은 영향을 받음
 - 클로저
 - 반복자
 - 12장 I/O 프로젝트 향상 및 성능 체크 
 
### 클로저: 환경을 캡처할 수 있는 익명 함수
 - 익명 함수 (C++의 람다함수)

#### 클로저로 행위를 추상화 하기
 - 함수를 클로저로 변경 

``` rust
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

``` rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

#### 클로저 타입 추론과 어노테이션
 - 클로저는 fn 함수처럼 파라미터나 반환값의 타입을 명시할 것을 요구하지 않습니다.
 - 클로저는 보통 짧고 임의의 시나리오 보다 좁은 문맥 안에서만 관련이 있어서 컴파일러가 안정적으로 추론할 수 있다.
``` rust
// 모두 동일
fn  add_one_v1   (x: u32) -> u32 { x + 1 } 
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

#### 제너릭 파라미터와 Fn 트레잇을 사용하여 클로저 저장하기
- 메모리제이션(지연평가): 결과값을 캐시에 저장

``` rust
struct Cacher<T>
    where T: Fn(u32) -> u32 // 구조체에는 클로저 타입을 명시해주어야함.
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> { // 데이터를 생성
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 { 
    // 데이터를 얻기 (이미 있으면 그냥 반환, 없으면 calculation에 저장된 클로저로 생성)
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

#### Cacher 구현의 제약사항
 1. Cacher 인스턴스가 value 메소드의 arg 파라미터에 대해 항상 같은 값을 얻는다. (해시맵을 사용하자.)
 
 2. u32 타입 파라미터 한 개만 받고 하나의 u32 을 반환한다 (더 중립적인 파라미터를 사용하자.)

#### 클로저로 환경 캡처 하기
* 클로저로 변수 참조.
```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x; // 상위 스코프 변수x를 캡쳐하여 사용

    let y = 4;

    assert!(equal_to_x(y));
}
```
* 함수로 변수 참조(X)
```rust
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x } // x를 캡쳐할 수 없어서 에러 발생

    let y = 4;

    assert!(equal_to_x(y));
}
```
- `FnOnce` 캡처한 변수의 소유권을 가져갑니다, 그래서 한 번만 호출 될 수 있습니다.
- `Fn` 은 그 환경으로 부터 값들을 불변으로 빌려 옵니다.
- `FnMut` 값들을 가변으로 빌려오기 때문에 그 환경을 변경할 수 있습니다.


``` rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; // 'move'키워드를 통해 x의 소유권을 클로저에게 넘김 

    println!("can't use x here: {:?}", x); // x의 소유권을 클로저가 가져서 컴파일 불가

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

### 반복자로 일련의 항목들 처리하기
 * 일련의 항목들에 대해 순서대로 어떤 작업을 수행할 수 있도록 해줍니다.
 * 반복자는 게으른데, 항목들을 사용하기위해 반복자를 소비하는 메서드를 호출해야 합니다.
``` rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter(); // 반복자를 생성하지만 아무일도 일어나지 않는다. 
```
* for 루프에서 반복자 사용
```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

#### Iterator트레잇과 next 메서드
 * 모든 반복자는 표준 라이브러리에 정의된 Iterator 라는 이름의 트레잇을 구현 합니다.
``` rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```
 * 주의사항
    -'next' 호출로 얻어온 값들은 벡터 안에 있는 값들에 대한 불변 참조합니다.

#### 반복자를 소비하는 메서드들
 * next 메서드를 구현해야 반복자를 소비하는 메서드를 사용할 수 있다. 

```rust
iter()      // 불변 참조
into_iter() // 소유권 이전
iter_mut()  // 가변 참조
```

#### 다른 반복자를 생성하는 메서드들

``` rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1); //클로저를 인자로 전달.
```
 * 반복자를 생성해도 반복자는 게으르므로 소비를 해주어야한다. 
``` rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect()를 사용해 반복자를 소비

assert_eq!(v2, vec![2, 3, 4]);
```

#### 환경을 캡쳐하는 클로저 사용하기
- `filter()`: iterator로부터 각항목을 받아 boolean을 반환하는 클로저를 인자로 전달받는다.

``` rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

#### Iterator 트레잇으로 자신만의 반복자 만들기
 * next 메서드를 정의해 자신만의 반복자를 생성할 수 있다. 

 * Counter 반복자 생성하기 
``` rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

 * Counter 반복자 사용하기 
``` rust
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

#### 다른 Iterator 메서드들 사용하기
* Iterator 메서드 사용 가능: Interator의 next메소드를 구현했기때문문이다.
```rust
#[test]

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1)) // Counter 인스턴스에 의해 생성된 값과 쌍을 이루며
                                 .map(|(a, b)| a * b)         // 각 쌍을 함께 곱하고,
                                 .filter(|x| x % 3 == 0)      // 3으로 나눠지는 값들만 유지하며
                                 .sum();                      // 모든 결과 값을 함께 더한다
    assert_eq!(18, sum);
}
```

### I/O 프로젝트 개선하기

#### 반복자 어댑터로 더 간결한 코드 만들기
- 기존 코드
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
- 개선된 코드
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

### 성능 비교하기: 루프 vs. 반복자
```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```
* 속도
    -Iterator가 빠름...

## 다음 챕터
- [Cargo 와 Crates.io 더 알아보기](https://rinthel.github.io/rust-lang-book-ko/ch14-00-more-about-cargo.html)

## 다음 과제
- [얼마?](https://www.acmicpc.net/problem/9325)

## 벌금현황
- 이지한: 0
- 김기덕: 0
- 유병조: 0
- 이명수: 0
- 이재현: 0
- 정연집: 0
- 허신: 10,000

## 회고
- 이지한: 결석
- 김기덕: 뒤로 갈수록 어려워지는 러스트네요...!
- 유병조: 함수형 기능들이 지원되서 편리하네요~
- 이명수: 하루 하루 기억력이... ㅎㅎ
- 이재현: 오늘 챕터는 꽤 재밌었어요, 직접 만나서 스터디 하니까 다들 이야기도 좀 더 많이하고 좋았어요 
- 정연집: 처음으로 직접보고 스터디해서 좋았습니다~
- 허신: 파이썬과 비슷한 부분이 많아서 익숙하고 재밌었어요~
