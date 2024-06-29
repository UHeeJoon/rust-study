use std::io::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};

/**
Rust의 에러는 복구 가능한 에러와
ex) File not found

복구 불가능한 에러가 존재한다.
ex) array index out of range

복구 가능 -> Result<T, E>

복구 불가 -> panic!
 */

fn main() {
   // let v = vec![1, 2, 3];
   // v[5] // => panic!

    let file = File::open("hello.txt"); // => Result<T, E>
    let file = match file { // 에러처리
        Ok(f) => Ok(f),
        Err(e) => match e.kind() { // 에러 핸들링
            // 파일이 없을 때
            ErrorKind::NotFound => File::create("hello.txt"),
            _ => panic!("파일 접근 실패"),
        },
    };

    //     위와 같이 따로 작성 안해주고 알아서 panic! 처리를 원한다면 .unwrap()을 사용할 수도 있다.
    let file = File::open("hello.txt").unwrap();

    //     .expect()를 사용해서 panic! 의 메시지를 바꿀 수도 있다.
    let file = File::open("hello.txt").expect("파일을 열 수 없음");

}

// 에러의 전파

fn read_username() -> Result<String, Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e),
    }
}

// 위 함수 같은 내용을 아래와 같이 축약할 수 있다.
fn read_username_summ() -> Result<String, Error> {
    // ?를 붙이면서 Error가 없으면 그대로 진행 아니면 Error반환
    let mut file = File::open("hello.txt")?;

    let mut username = String::new();

    // ?를 붙이면서 Error가 없으면 그대로 진행 아니면 Error반환 | 에러 없으면 file내용 username에 작성
    file.read_to_string(&mut username)?;

    // File::open("hello.txt")?.read_to_string(&mut username)?; 와 같이 체이닝으로 축약 가능

    Ok(username)
}
