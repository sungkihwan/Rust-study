fn main() {
    another_function(10, 15);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 반환 값은 세미콜론으로 표시하지 않는다. 세미콜론으로 표시하면 이는 구문이 되고 반환값이 아니게 된다.
    };

    println!("x is {}", x);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}