fn main() {
    another_function(10, 15);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 반환 값은 세미콜론으로 표시하지 않는다. 세미콜론으로 표시하면 이는 구문이 되고 반환값이 아니게 된다.
    };

    println!("x is {}", x);
    println!("The value of y is: {}", y);

    let mut z = five();

    println!("z is {}", z);

    let k = "a";

    println!("{}", k);

    z = plus_one(z);

    println!("{}", z);

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

