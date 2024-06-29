fn main() {
    // tuple type
    let t: (i32, bool, f64) = (32, true, 1.41);

    // 인덱스로 접근 가능
    let _x = t.0;
    let _y = t.1;
    let _z = t.2;

    // 구조 분해 구문 사용가능!
    let (_x, _y, _z) = t;
    // 튜플 내용 전체 출력할 때 아래처럼 가능
    println!("{:?}", t);


    // unit 튜플
    let _t:() = ();

    //     ==================================
    // 배열 array 타입/ 동일한 타입만 가능함
    let x:[i32; 5] = [1, 2, 3, 4, 5];
    let threes = [3; 100]; // 정수 3, 100개로 배열 초기화
    let hellos = ["hello"; 10]; // 문자열 "hello", 10개로 배열 초기화

    x[0]; x[1]; // 동일하게 인덱스로 사용 가능
    // 출력할 때 아래처럼 가능
    println!("{:?}", x)

}
