fn main() {
    // 1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
    // 2. 한번에 딱 하나의 오너만 존재할 수 있다.
    // 3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

    let mut s = String::from("hello"); 
    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
    println!("{}", s);

    // mut string 타입은 변경가능하고 커질 수 있는 텍스트를 지원합니다.
    // 힙에서 컴파일 타임에는 알 수 없는 어느정도 크기의 메모리 공간을 미리 할당받아 내용을 저장해야 한다.

    // -> 런타임에 운영체제로부터 메모리가 요청되어야 한다.
    // -> String의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다.

    // 가비지 콜렉터(GC) 를 갖고 있는 언어들의 경우, GC가 더이상 사용하지 않는 메모리 조각을 계속해서 찾고 지워준다.

    // 러스트는 GC가 작동하는 것이 아니라 메모리는 변수가 소속되어있는 스코프 밖으로 나가면 자동으로 반납된다.
    // 변수가 스코프 밖으로 벗어나면, 러스트는 drop 함수를 호출하고 메모리를 반환한다.

    // let s1 = String::from("hello");
    // let s2 = s1;

    // 이렇게 작성하면 s1 -> s2로 변수가 '이동' 한것이다.
    //  “얕은 복사(shallow copy)”와 “깊은 복사(deep copy)” 의 개념과는 다르며 s1은 이제 사용할 수 없다.
    // 러스트는 자동적으로 깊은 복사를 만들지 않는다.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("{}", s1 == s2);

    takes_ownership(s);

    let x = 5;                      
    makes_copy(x);

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // 소유권을 넘겨준다.
    let s4 = gives_ownership(); 
    println!("{}", s4);

    // 튜플로 기존 값을 받아올 수 있다.
    let (s2, len) = calculate_length(s1);
    println!("s2 = {}, len = {}", s2, len);

    // 소유권을 넘기는 대신 참조자를 인자로 넘겨줄 수 있다.
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("s1 = {}, len = {}", s1, len);

    // 참조자의 원본을 변경하는 것을 허용하지 않는다.
    // let s = String::from("hello");
    // change(&s); 

    // mut 타입을 넘겨주어 변경할 수 있다.
    let mut s = String::from("hello");
    change2(&mut s);

    // 여러개의 가변 참조자를 생성할 수 없다.
    // let r1 = &mut s;
    // let r2 = &mut s; // -> cannot borrow `s` as mutable more than once at a time
    // println!("r1 = {}, r2 = {}", r1, r2);

    // -> 데이터 레이스는 아래에 정의된 세 가지 동작이 발생했을때 나타나는 특정한 레이스 조건이다.
    // 1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
    // 2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
    // 3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

    // 중괄호를 사용하여 동시에 사용하지 않게 하면 여러 개의 가변참조자를 만들 수 있다.
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // 여기서 r1은 스코프 밖으로 벗어났으므로, 아무 문제 없이 새로운 참조자를 만들 수 있습니다.
    
    let r2 = &mut s;
    println!("{}", r2);

    
    // 가변참조자와 불변 참조자를 혼용할 경우 생기는 문제
    // 불변 참조자를 가지고 있을 동안에도 역시 가변 참조자를 만들 수 없다.
    // let r1 = &s; // 문제 없음
    // let r2 = &s; // 문제 없음
    // let r3 = &mut s; // 큰 문제 -> cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("r1 = {}, r2 = {} r3 = {}", r1, r2, r3);


    // 댕글링 참조자
    // 포인터가 있는 언어에서는 자칫 잘못하면 댕글링 포인터(dangling pointer) 를 만들기 쉬운데,
    // 댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게
    // 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말합니다.

    // let reference_to_nothing = dangle(); // -> missing lifetime specifier
    // reference_to_nothing의 라이프 타임내에 원본 객체가 메모리에서 반환되었다.
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
    

    // 슬라이스
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word = {}", word);
    s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[3..11];
    let slice = &s[..2];
    println!("hello = {}, world = {} slice = {}", hello, world, slice);
    
    // 둘은 같다.
    // let len = s.len();
    // let slice = &s[3..len];
    // let slice = &s[3..];

    // 둘은 같다.
    // let slice = &s[0..len];
    // let slice = &s[..];

    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string[..]);
    println!("word = {}", word);

    let my_string_literal = "hello world";
    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word(&my_string_literal[..]);
    println!("word = {}", word);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word(my_string_literal);
    println!("word = {}", word);
}

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프안으로 들어왔습니다.
    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
 
fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킵니다.
    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.
    some_string                              // some_string이 반환되고, 호출한 쪽의함수로 이동됩니다.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len() // 가리키고 있는 값에 대한 소유권이 없기 떄문에 참조자가 스코프 밖으로 벗어났을 때도 메모리가 반납되지 않는다.
}

// fn change(some_string: &String) {
//     some_string.push_str(", world"); 
// }

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}