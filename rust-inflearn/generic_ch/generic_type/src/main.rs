/*
예를 들어 아래 두 함수처럼 비슷한 역할을 하는함수가 있다고 가정을 해보자
smallest_i32 는 정수 배열을 받아 가장 작은 값을 리턴하고
smallest_char 는 문자열 배열을 입력받아 가장 작은 값을 리턴한다.

두 함수의 차이는 받는 타입만 다를 뿐 기능은 동일하다.

*/

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

/*

입력 받는 타입에 자유를 주기 위해서 Generic 이라는 것을 사용한다.
위 두 함수를 Generic 으로 바꾸면 아래와 같이 표현할 수 있다.

입력이 들어오는 타입은 T다 라고 암시적으로 선언한 후
매개변수가 들어왔을 때 T의 위치에 변수의 타입이 들어가게 된다.

T:PartialOrd 는 순서가 들어있는 경우를 뜻하는 뜻이며
저게 무엇인지는 뒤에 traits 부분에서 다루도록 하겠다.
*/

fn smallest<T: PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

/*
Generic은 Struct 에서도 사용할 수 있다.
*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

/*
method 선언에 Generic
*/

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/*
Enum에 Generic
*/

enum Option<T> {
    Some(T),
    Err(),
}


fn main() {
    let number = vec![3, 4, 1, 6, 8, 10];

    let result = smallest(&number);
    println!("가장 작은 수는 = {}", result);

    let chars = vec!['홍', '길', '동'];

    let result = smallest(&chars);
    println!("가장 앞에 오는 글자는 = {}", result);

    let result = smallest(&["홍길동", "둘리", "장영삼", "데이비드"]);
    println!("가장 먼저 오는 이름은 = {}", result);


    let p1 = Point { x: 0, y: 2 };
    let p2 = Point { x: 2.3, y: 3.2 };
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}
