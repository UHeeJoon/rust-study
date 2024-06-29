#[derive(Debug)] // 컴파일러에게 Debug를 위한 함수를 작성해달라는 표시 | {:?}와 같은 것으로 구조체 호출 가능
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 20;
    let height = 30;
    println!("사각형의 면적은 {}", area(width, height));

    let rect = (20, 30);
    println!("사각형의 면적은 {}", area_tuple(rect));

    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("사각형의 면적은 {}", area_struct(&rect));
    println!("사각형 = {:?}", &rect); //#[derive(Debug)] 붙여서 사용 가능
    // {}는 디스플레이, {:?}는 디버그 관련 메서드
    // 디버그 메서드/ 자세한 정보 출력
    dbg!(rect);
}

fn area(width: u32, height: u32) -> u32{
    width * height
}

fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


