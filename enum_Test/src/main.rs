#![allow(unused)]
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    
    let loopback2 = IpAddr2::V6(String::from("::1"));

    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr3::V4(127, 0, 0, 1);
    
    let loopback = IpAddr3::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 메소드 내용은 여기 정의할 수 있습니다.
            println!("address")
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();


    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");
}

