# 제네릭, 트레이트, 수명
* Generic - 여러 타입에 대해 공통 코드를 작성
* Trait - 다양한 타입 중에 제약 조건을 만족하는 타입으로 한정
* Lifetime - 참조 Reference값의 수명 정보를 제공

# Generic
> 유연하고 중복이 없는 코드를 작성하기 위한 타입  
> 러스트 컴파일러는 제네릭을 사용하는 코드에 대해 `단형성화`를 수행해 성능에 아무런 지장이 없다.  
> _※ 단형성화란: 제네릭 코드를 실제로 채워질 구체적인 타입으로 된 특정 코드로 바꾸는 과정을 말한다._

smallest_i32 는 정수 배열을 받아 가장 작은 값을 리턴하고  
smallest_char 는 문자열 배열을 입력받아 가장 작은 값을 리턴하는 함수가 있다고 하자
```rust
fn smallest_i32(list: &[i32]) -> &i32 {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest_char(list: &[char]) -> &char {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
```
위 두 함수는 받는 타입만 다를 뿐 내부 로직은 동일한 것을 알 수 있다.  
타입이 다르다는 이유 만으로 같은 로직을 두 번 작성해야하는 번거로움이 있다.  
이럴 때 이러한 중복을 없애 주는 것이 `Generic`이다.  
그러면 이제 Generic으로 위 두 함수를 바꾸어보자.
```rust
fn smallest<T: PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
```
Generic은 `<>`를 사용해서 `어떠한 타입이 들어올거다.` 라고 알려준다.  
일단 PartialOrd는 무시하고 보자면 T라는 타입이 들어오고 해당 타입은 입력으로 들어온 배열, 반환 값의 타입 두 곳에서 쓰일 것이라는 것을 뜻한다.  
위에서 타입에 따라 두 번 사용했던 함수를 Generic으로 다시금 작성해서 코드의 중복을 줄이고 더 유연한 코드의 작성이 가능하게 된다.  

다시 돌아와 PartialOrd라는 것에 집중해보자.  
이것은 아래에서 다룰 trait로 간략히 설명하자면 하나의 구현체이다. 위 함수의 로직을 보면 `T`의 값을 비교하는 로직이 존재한다.  
일반적으로 list는 비교를 할 수 없다. 따라서 list를 비교하기 위해서는 어떻게, 어떤 순서로 정렬하고 비교할 것인지에 대한정의와 비교 연산이 가능하도록 해주는 역할을 하는 `partialOrd`가 필요하다.


# Trait
> trait는 여러 타입들이 공통적으로 갖는 동작에 대하여 추상화하도록 강제해준다.  
> 러스트 컴파일러에게 여러 타입이 공유할 수 있는 기능에 대한 설명을 해준다.
> 
trait를 아래처럼 선언하면 Greet trait를 구현하는 입장에서는 greeting 메서드를 반드시 구현해야한다.
```rust
trait Greet {
    fn greeting(&self) -> String;
}
```

trait 를 구현 할 때에는 `for` 키워드를 사용해서 구현한다.
```rust
// 구조체
struct Person {
    name: String,
    active: bool,
}
// 구조체로 trait 구현
impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("안녕")
    }
}
```
trait를 파라미터로 전달해서 받을 때에는 &impl Greet와 같이   
Greet trait를 구현한 타입이라는 것을알려줘야한다.
```rust
// Greet를 전달 받아서 Greet의 greeting 메서드 실행
fn meet(one: &impl Greet, another: &impl Greet) {
    println!("첫 번째가 인사합니다 = {}", one.greeting());
    println!("두 번째가 인사합니다 = {}", another.greeting());
}
```
# Generic + Trait

위 함수`meet`는 들어오는 Greet을 구현하기만 한다면, 타입이 다른 여러 구현체들이 매개변수로 들어올 수 있다.  
만약 매개 변수의 타입을 지정하고 싶다면 Generic을 사용하여 들어오는 타입을 정할 수도 있다.
```rust
fn meet<T:Greet>(one: &T, another: &T) {
    println!("첫 번째가 인사합니다 = {}", one.greeting());
    println!("두 번째가 인사합니다 = {}", another.greeting());
}
```
만약 Greet와 Debug를 모두 구현한 타입을 받고싶다면 `+`를 사용해서 추가할 수도있다.
```rust
fn meet<T:Greet + Debug>(one: &T, another: &T) {
    println!("{:?}가 인사합니다 = {}", one, one.greeting());
    println!("{:?}가 인사합니다 = {}", another, another.greeting());
}
```

---

지금까지는 몇 개 추가하지 않아서 지저분하지 않지만 trait를 더 많이 추가해야하고  
매개 변수 모두 구현하는 구현체가 달라야한다면 그때는 너무나 지저분 할 수가 있다.  
이때 `where`라는 키워드를 사용할 수 있다.
```rust
fn meet<T, U>(one: &T, another: &U)
where
    T: Greet + Debug,
    U: Greet + Display
{
    println!("{:?}가 인사합니다 = {}", one, one.greeting());
    println!("{}이/가 인사합니다 = {}", another, another.greeting());
}
```
기존에 작성했던것 과 달리 Generic에는 원시형태 그대로 작성하고 함수의 몸통을 작성하기 전 부분에
`where`를 사용하고 뒷 부분에 구현해야하는 `trait`들을 작성해주면 된다. 
Generic에 쓰지 않고 분리해서 코드가 한결 깔끔해지게된다.

# 임대값의 수명(lifetimes)
> * Rust의 고유한 부분
> * Generic - 어떤 타입의 특성을 제한
> * 수명 - 참조값이 필요한 만큼 유효하게 선언
> * 모든 참조값에는 유효 수명이 존재
> * 타입 추론으로 타입선언 생략할 수 있듯, 대부분 생략 가능
> 
## 임대 검사 (Borrow Checker)
> * 타입이 잘 맞춰졌는지 검사하는 "타입검사" 처럼
> * 임대한 참조의 수명이 유효한지 검사하는 "임대검사"
> * 참조의 수명보다, 원래 값의 수명이 같거나 길어야 함
> 
### 참조 값의 수명이 더 짧은 경우
> 참조 값의 수명이 더 길어야만 한다.
```rust
// 불가능!
// x가 y보다 수명이 길어서 값을 넣을 수가 없다.
fn short_lifetime() {
    let x;
    {
        let y = 5;
        x = &y;
    }
    println!("x = {}", x);
}
```

```rust
// 불가능!
// res가 s2보다 수명이 더 길어 res에 값을 대입할 때 에러가 발생하게 된다.
fn main() {
    let s1 = String::from("가나다라");
    let res: &str;
    {
        let s2 = String::from("가나다라마바사");
        res = longest(s1.as_str(), s2.as_str());
    }
    println!("더 긴 문자열은 {}", res);
}
```
`borrowed value does not live long enough` 에러가 발생한다.

  
### 참조 리턴값의 수명 명시
```rust
fn longest(s1: & str, s2: & str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```
컴파일러 입장에서는 둘 중 lifetime이 뭐가 길지 알 수 없고 runtime에서만 알 수 있다.  
또한 반환하는 참조자가 항상 유효한지도 알 수 없다. 그렇기 때문에 수명을 명시 해주어야 한다.  
> lifetime명시는 참조자가 얼마나 `사는지`를 결정짓는 것이 아니다.   
> 그저 여러개의 참조자에 대한 lifetime을 `연관` 짓도록 하는 것이다. 

lifetime 명시는 살짝 독특한 문법을 가지고 있다. `'`로 시작해야하고 보통 소문자를 사용한다.  
관례적으로 `'a`를 많이 사용하고 `&`뒤에 온다.

함수의 파라미터의 lifetime을 특정해야 한다면 Generic과 마찬가지로 `<>`안에 정의 되어있어야한다. 
```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

### static 수명
`'static`은 프로그램 실행 내내 유효한 수명을 뜻함
```rust
fn static_lifetime() {
    let s: &'static str = "프로그램 실행 내내 유효한 수명";
}
```

## 수명 표기 생략 규칙(Lifetime Elision Rules)
> 1. ~~각각의 파라미터에 차례로 수명 표기~~(원칙)
> 2. 파라미터가 딱 하나라면, 모든 반환값에 해당 수명 부여
> 3. 메소드이고, 여러 파라미터 중 하나가 &(mut)self 라면, 모든 반환 값 수명에 self 수명을 반영 

### 수명 표기 생략
파라미터가 하나라면, 모든 값에 해당 수명 부여
```rust
// 수명을 표기해주지 않아도 된다
fn rule(s: &str) -> &str {
    // ...
}
```
메소드이고, 여러 파라미터 중 하나가 &(mut)self 라면, 모든 반환 값 수명에 self 수명을 반영
```rust
impl ImportantPart {
    fn notice(&self, text:&str) -> &str {
        println!("{}", text);
    }
}
```
## Lifetime 요약
> * 참조의 유효 수명을 표기 (수명 자체를 변경하지는 못함)
> * 함수의 입력 파라미터에 있는 참조값과 반환 참조값의 수명관계 표현
> * 유효 수명이 확보되는지 "임대검사" 진행
> * 수명 표기 생략 규칙에 따라 생략 가능

