struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // tuple을 struct로 선언했을 때 일반 tuple과 별 다른점은 없지만
    // 기존에 tuple을 여러개 선언하면 뭐가 뭔지 식별하기 힘들지만
    // struct로 선언하게 되면 구분하기 쉬워진다.
    let color = Color(1, 2, 3);
    let _point = Point(1, 2, 3);
    println!("color.0 = {}, color.1 = {}, color.2 = {}", color.0, color.1, color.2);
}
