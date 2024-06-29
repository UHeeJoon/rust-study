fn main() {
    // if
    let x = 5;
    let condition = true;

    let _y = if condition { 3 } else { 5 }; // expression 으로 사용할 수 있음

    if x % 3 == 0 {
        println!("{x}를 3으로 나누어 떨어집니다.");
    } else if x % 3 == 1 {
        println!("{x}를 3으로 나눈 나머지는 {}입니다.", x % 3);
    } else if x % 3 == 2 {
        println!("{x}를 3으로 나눈 나머지는 {}입니다.", x % 3);
    }


    // ================================
    // loop 는 무한 반복
    // loop{}
    let mut counter = 0;
    // loop의 결과 값을 리턴해줄 수도 있다.
    let result = loop {
        println!("무한 반복");
        counter += 1;
        if counter == 3 {
            // 리턴 값 정의
            break counter;
        }
    };
    println!("loop의  결과는 {}", result);


    // ================================
    // while 문
    let mut counter = 0;
    while (counter < 3) {
        println!("반복!");
        counter += 1;
    }

    // 배열 원소 접근
    let xs = [1, 2, 3, 4];
    let mut idx = 0;
    while idx < xs.len() {
        println!("xs[{}] = {}", idx, xs[idx]);
        idx += 1;
    }


    // ================================
    // for 문
    let xs = [1, 2, 3, 4];
    for x in xs {
        println!("x = {}", x);
    }
    // 0부터 5 - 1까지
    for i in 0..5 {
        println!("i = {i}");
    }
    // 0 부터 5까지
    for i in 0..=5 {
        println!("i = {i}");
    }
    // 배열길이 만큼
    for idx in 0..xs.len() {
        println!("idx = {}, xs[{}] = {}", idx, idx, xs[idx]);
    }
    // 범위 거꾸로
    for i in (0..=5).rev() {
        println!("i = {i}");
    }
}
