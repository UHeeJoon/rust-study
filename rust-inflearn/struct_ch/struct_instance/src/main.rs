// 구조체
struct User {
    name: String,
    email: String,
    active: bool,
}

// utility functions
fn build_user(name: String, email: String) -> User {
    // 중복되는 이름의 축약이 가능함
    User {
        name,
        email,
        active: true,
    }
}

fn main() {
    let user = User {
        name: String::from("홍길동"),
        email: String::from("hongildong@example.com"),
        active: true,
    };

    // 구조체도 그냥 선언 하면 immutable 타입이기에 변경하고자 한다면 mut 키워드를 붙여주어야함
    let mut user2 = build_user(String::from("hongildong"), String::from("email@example.com"));
    user2.email = String::from("hongildong2@example.com");
    println!("user name = {}", user2.name);
    println!("user email = {}", user2.email);
    println!("user active = {}", user2.active);

    // user에서 새로운 user를 만들고 싶다면 이런식으로 해도 되지만
    let user3 = User {
        name: user.name.clone(),
        email: user.email.clone(),
        active: user.active,
    };

    // 이런식으로 축약해서 복사 가능함
    let user4 = User {
        active: false,
        ..user
    };
}

