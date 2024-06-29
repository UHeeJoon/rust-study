# 소유권 임대
함수에서 소유권을 줬다가 다시 받아야할 때
```rust
fn main() {
    let s = String::from("hello");
    
    let (s, len) = calc_length(s);
    
    println!("'{}의 길이는  {}입니다.", s, len);
}

fn calc_length(s:String) -> (String, usize){
    let length = s.len();
    (s, length)
}
```
위과 같이 소유권을 함수로 넘겨주고 원하는 계산값과 소유권을 함께 돌려준다. <br>
하지만 위와 같이 함수를 작성하면 예상하지 않은 값인 s를 그대로 넘겨줘야해 비용이 증가하게 된다.<br>
따라서 '소유권 임대'라는 것이 필요하다.

### 소유권 임대 추가
기존과는 다르게 함수 인자 부분에 `&`를 붙여줌으로써 `소유권 임대`를 명시해준다.
```rust
fn main() {
    let s = String::from("hello");
    
    let len = calc_length(&s); // 소유권 임대
    
    println!("'{}의 길이는  {}입니다.", s, len);
}

fn calc_length(s: &String) -> usize {
    let length = s.len();
    length // 더 이상 소유권을 돌려줄 필요가 없음
}
```
이 전 함수와는 다르게 소유권을 돌려줄 필요가 없음으로 `calc_length`의 반환부에 소유권이 빠지게된다.

기존의 방식으로 값을 할당하게되면 포인터가 Heap 메모리의 같은 주소를 가리켜 소유권을 양도하게 되지만 
위처럼 `&`를 붙이면 `&s`가 `s`를 가리켜 참조하여 접근을 하게된다.

### 참조(Reference)
이처럼 참조(Reference)는
* 특정 데이터가 위치한, 접근할 수 있는 주소
* 해당 데이터는 누군가 다른 소유자가 소유

라는 특징을 가지고 있다.<br>

또한 참조는 기본적으로 immutable 하다는 특성을 가지고 있다.  
하지만 이 또한 mutable 하다고 명시를 해준다면 mutable 하게 사용할 수 있다.
```rust
fn main() {
    let mut s = String::new("hello");

    append_word_immut(&s); // immutable
    
    append_word_mut(&mut s); // mutable
    println!("s = {}", s);
}

fn append_word_immut(s:&String) {
    // 참조 변수는  immutable 하기 때문에 변경할 수 없다
    s.push_str(", world"); // Error
}

fn append_word_mut(s:&mut String) {
    s.push_str(", world"); // Success
}
```
mutable 참조에는 한 가지 특징이 있는데 mutable 참조를 사용하면 `'참조에 대한 접근'`은 `더 이상` 할 수 없다는 것이다.
```rust
fn main() {
    let mut s = String::new("hello");

    let r1 = &mut s; // mut으로 참조하지 않았다면 문제 없음
    let r2 = &mut s; // ""
    
    // mut으로 참조된걸 사용하면 이후의 해당 스코프에서 더 이상 참조에 대한 접근이 불가해진다.
    println!("r1 = {}, r2 = {}", &r1, &r2); // Error
}
```
위 예시는 처음 변수를 선언 했을 시에는 문제가 없지만 실제 사용을 하게되면 문제가 생기게 된다.   
이처럼 여러번 사용을 제한하게 되는 것을 `데이터 경쟁 조건(data trace)` 이라고 한다.
#### 데이터 경쟁 조건(data trace)
* 둘 이상의 포이터가 같은 데이터를 참조
* 두 개 이상의 포인터가 데이터를 쓰려고 접근
* 해당 데이터 접근을 동기화할 방법이 없음
* Rust는 아예 컴파일 타임에 데이터 경쟁 조건을 방지함

