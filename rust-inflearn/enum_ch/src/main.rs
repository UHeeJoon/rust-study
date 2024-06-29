// 디스플레이로 확인이 불가하여 debug가 필요함
// 비교할 때는 PartialEq
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

enum Message {
    StartGame,
    WindPoint {who: String}, // struct 처럼 사용 가능
    ChangePlayerName(String), // 튜플처럼 사용 가능
}

fn main() {
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    println!("red = {:?}, green = {:?}, blue = {:?}", red, green, blue);

    println!("red == blue => {}", red == blue);
    println!("red == blue => {}", red == red);

    // =====================================

    let m1 = Message::StartGame;
    let m2 = Message::WindPoint {
        who: String::from("홍길동"),
    };
    let m3 = Message::ChangePlayerName(String::from("둘리"));

}

// Enum 패턴 매칭
// match를 쓰면 선언되어있는 모든 경우에 대한 처리를 해주어야함
// Option<T>라는 기본으로 제공해주는 Enum타입도 있음 -> 다른 언어에서 사용하는 Null 대체/ Rust는 Null이 없음

struct RGB(u8, u8, u8);

// Color의 값에 맞게 RGB변경
fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}

// Message에 따른 결과 처리
fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("시작"),
        Message::WindPoint{who} => println!("{}의 득점 ", who),
        Message::ChangePlayerName(name) => println!("플레이어 이름 {}", name),
    }
}

// Option을 사용해 값 증가
fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

// 모든 경우를 처리할 수 없을 때엔 _(와일드카드)사용
fn handle_msg_wildcard(message: &Message) {
    match message {
        Message::StartGame => println!("게이 시작"),
        _ => println!("미구현 ㅠ"),
    }
}