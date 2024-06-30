mod utils;

// Rust와 js가 소통할 수 있게 도와주는 패키지
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// 외부에 있는 어떠한 함수를 호출하기 위해 미리 선언하는 구문
extern "C" {
    // 브라우저에 있는 alert 함수
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    // 브라우저에 있는 alert함수를 호출
    alert(&format!("입력한 값은, {} 입니다.", s));
}
