fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 3;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100000;
    println!("The value of MAX_POINTS is: {} {}", MAX_POINTS, x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("value {}", spaces);

    
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess {}", guess);
}
