use std::fmt::{Debug, Display, Formatter};

trait Greet {
    fn greeting(&self) -> String;
}

// enum으로 트레이트 구현
#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
    Tiger,
}

impl Greet for Pet {
    fn greeting(&self) -> String {
        match self {
            Pet::Dog => String::from("멍멍"),
            Pet::Cat => String::from("야옹"),
            Pet::Tiger => String::from("어흥"),
        }
    }
}

// 구조체로 트레이트 구현
struct Person {
    name: String,
    active: bool,
}

impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("안녕")
    }
}

// Display 구현
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

// trait 파라미터로 전달 받기
fn meet(one: &impl Greet, another: &impl Greet) {
    println!("첫 번째가 인사합니다 = {}", one.greeting());
    println!("두 번째가 인사합니다 = {}", another.greeting());
}

// 만약 입력으로 들어온 Greet이 같은 타입이여햐 할 경우 generic을 사용해서 제한해줄 수도 있다.
fn meet2<T: Greet>(one: &T, another: &T) {
    println!("첫 번째가 인사합니다 = {}", one.greeting());
    println!("두 번째가 인사합니다 = {}", another.greeting());
}

// Greet와 Debug 모두 구현한 타입만 가능!
fn meet3<T: Greet + Debug>(one: &T, another: &T) {
    println!("{:?}가 인사합니다 = {}", one, one.greeting());
    println!("{:?}가 인사합니다 = {}", another, another.greeting());
}

/*
Generic에 구현해야하는 타입이 여러가지일 때
<T: Greet + Debug, U: Greet + Display> => 보기 힘들고 지저분하다!
이때 where라는 구문을 사용할 수 있다.
*/
fn meet4<T, U>(one: &T, another: &U)
where
    T: Greet + Debug,
    U: Greet + Display
{
    println!("{:?}가 인사합니다 = {}", one, one.greeting());
    println!("{}이/가 인사합니다 = {}", another, another.greeting());
}

fn main() {
    let cat = Pet::Cat;
    let gildong = Person {
        name: String::from("홍길동"),
        active: true,
    };
    meet(&cat, &gildong);
    meet2(&Pet::Cat, &Pet::Dog);
    meet3(&Pet::Cat, &Pet::Dog);
    meet4(&Pet::Tiger, &gildong);
}