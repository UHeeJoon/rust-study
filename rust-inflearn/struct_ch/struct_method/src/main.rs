#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 함수의 메서드 선언할 땐 impl 영역 안에 선언
impl Rectangle {
    // &self는 자기 자신을 뜻함, 또한 &를 붙여서 임대해서 계속 사용가능
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// impl 을 여러번 사용해도 됨
impl Rectangle {
    // associated 함수
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("이 사각형의 너비는 = {}", rect.area());

    // associate 함수
    println!("정사각형의 너비는 = {}", Rectangle::square(20).area())
}
