fn main() {
    println!("Hello, world!");

    a_function();

    print_number(3);

    // 반환 값이 있으면 블럭도 문제 없이 변수에 대입 가능
    let x = {
        let y = 3; // statement
        y // expressions | ;이 없어야함
    };

    println!("x = {x}");

    let a = circle_area(2.0);
    println!("반지름이 2.0인 원의 면접은 {a} 입니다.");
}

// 러스트에서 이름을 지을 때에는 _를 사용함
fn a_function() {
    println!("a 함수");
}

// 파라미터를 받을 때에는 타입 명시를 해줘야함
fn print_number(x: i32) {
    println!("x = {x}");
}

// return values
const PI:f64 = 3.141592;
fn circle_area(radius: f64) -> f64 {
    PI * radius * radius    // 마지막 반환 값에는 ;이 있으면 안됨
    // return PI * radius * radius; // return 이 있으면 ;가 있던 말던 상관 없음
}