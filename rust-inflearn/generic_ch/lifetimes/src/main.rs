/*
불가능!
borrowed value does not live long enough

*/
fn short_lifetime() {
    let x;
    {
        let y = 5;
        x = &y;
    }
    println!("x = {}", x);
}

/*
컴파일러 입장에서는 둘 중 lifetime이 뭐가 길지 알 수 없고 runtime에서만 알 수 있다.
또한 반환하는 참조자가 항상 유효한지도 알 수 없다. 그렇기 때문에 수명을 명시해주어야한다.

return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
*/

fn longest(s1: & str, s2: & str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


/*
lifetime 명시는 살짝 독특한 문법을 가지고 있다. `'`로 시작해야하고 보통 소문자를 사용한다.
관례적으로 `'a`를 많이 사용하고 `&`뒤에 온다.

함수의 파라미터의 lifetime을 특정해야 한다면 Generic과 마찬가지로 `<>`안에 정의 되어있어야한다.
 */
fn longest2<'a >(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// =================================================================
// 구조체에서 임대수명 표기
struct ImportantPart<'a> {
    part: &'a str,
}

fn lifetime_in_struct() {
    let sentences = String::from("러스트의 수명");
    let first_sentence = sentences
        .split(' ')
        .next()
        .expect("' '빈 칸을 찾을 수가 없습니다.");
    // ImportantPart를 first_sentence로 만듦으로써 두 개의 수명은 같게된다.
    let i = ImportantPart {
        part: first_sentence,
    };
}

fn main() {
    short_lifetime();

    let s1 = String::from("가나다");
    let s2 = "가나다라";

    let res = longest(&s1, &s2);
    println!("더 긴 문자열은 = {}", res);

    let s1 = String::from("가나다라");
    let res: &str;
    {
        // s2가 res보다 수명이 짧아서 오류가 나게 된다.
        let s2 = String::from("가나다라마바사");
        res = longest2(&s1.as_str(), &s2.as_str());
    }
    println!("더 긴 문자열은 {}", res);
}
