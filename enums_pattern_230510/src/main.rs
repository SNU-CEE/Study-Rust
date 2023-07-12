fn main() {

// 1111
    // // 열거형 정의하기
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }
    // // IpAddrKind 의 두 개의 variants 에 대한 인스턴스
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;   
    // // IpAddrKind 타입을 인자로 받는 함수를 정의할 수 있음
    // fn route(ip_type: IpAddrKind) { }
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

// 2222
    // // 열거형 variant 에 데이터를 직접 넣는 방식을 사용해서
    // // 더 간결하게 동일한 개념을 표현
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

// 3333
    // // 구조체 보다 열거형을 사용할 때 다른 장점
    // // 각 variant 는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있음!!
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

// // 4444
    // // 열거형 variant 에 어떤 종류의 데이터라도 넣을 수 있음
    // struct Ipv4Addr {
    //     // details elided
    // }
    // struct Ipv6Addr {
    //     // details elided
    // }
    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

// 5555
    // // 모든 variants 가 Message 타입으로 그룹화
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }
    // // 아래 구조체와 동일한 데이터를 포함하는 셈
    // struct QuitMessage; // 유닛 구조체
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // 튜플 구조체
    // struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

    // impl Message {
    //     fn call(&self) {
    //         // 메소드 내용은 여기 정의할 수 있습니다.
    //     }
    // }
    // let m = Message::Write(String::from("hello"));
    // m.call();


    // // 러스트에는 null 이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있음!!
    // // enum Option<T> {
    // //     Some(T),
    // //     None,
    // // }
    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
    // // 에러발생 why?? -> 컴파일러는 None 만 보고는 Some variant 가 어떤 타입인지 추론할 수 없음

    // // let x: i8 = 5;
    // let x: Option<i8> = Some(3);
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // // 컴파일러는 Option<T> 값을 명확하게 유효한 값처럼 사용하지 못하도록 함!!
    // // Option<i8> 와 i8은 다른 타입





    // match 흐름 제어 연산자

// 6666
    // // 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매치되었는지를 바탕으로 코드를 수행하도록 해줌
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }
    // // fn value_in_cents(coin: Coin) -> u32 {
    // //     match coin {
    // //         Coin::Penny => 1,
    // //         Coin::Nickel => 5,
    // //         Coin::Dime => 10,
    // //         Coin::Quarter => 25,
    // //     }
    // // }
    // // coin의 타입은 Coin 열거형
    // // Coin::Penny로 되어있는 패턴을 가지고 있고
    // // 그 후에 패턴과 실행되는 코드를 구분해주는 => 연산자
    // fn value_in_cents(coin: Coin) -> u32 {
    //     match coin {
    //         Coin::Penny => {
    //             println!("Lucky penny!");
    //             1
    //         },
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }
    // // Coin::Penny와 함께 메소드가 호출될 때마다 “Lucky penny!”를 출력


// 7777
    // // 또 다른 유용한 기능은 패턴과 매치된 값들의 부분을 바인딩 가능!!
    // #[derive(Debug)] // So we can inspect the state in a minute
    // enum UsState {
    //     Alabama,
    //     Alaska,
    //     // ... etc
    // }
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }
    // fn value_in_cents(coin: Coin) -> u32 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter(state) => {
    //             println!("State quarter from {:?}!", state);
    //             25
    //         },
    //     }
    // }
    // // Option<T>를 이용하는 매칭
    // // Option<i32>를 파라미터로 받아서, 내부에 값이 있으면, 그 값에 1을 더하는 함수
    // // 내부에 값이 없으면, 이 함수는 None 값을 반환하고 다른 어떤 연산도 수행하는 시도를 하지 않음
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }
    // let five = Some(5);
    // // Some(5) 값은 패턴 None과 매칭 X
    // // Some(5)가 Some(i)랑 매칭 O
    // let six = plus_one(five);
    // let none = plus_one(None);
    // // None은 잘 매칭됨

    // // fn plus_one(x: Option<i32>) -> Option<i32> {
    // //     match x {
    // //         Some(i) => Some(i + 1),
    // //     }
    // // } // 이런 경우 None 케이스를 다루지 않았고 컴파일러가 에러를 잡아줌


// 8888
    // // 우리가 모든 가능한 값을 나열하고 싶지 않을 경우에 사용할 수 있는 패턴
    // let some_u8_value = 0u8;
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
    // }
    // // _ 패턴은 어떠한 값과도 매칭

    // // 어떤 Option<u8> 값을 매칭 하지만 그 값이 3일 경우에만 코드를 실행시키는 프로그램
    // let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    // // if let을 이용하여 더 간결하게 표현 가능
    // if let Some(3) = some_u8_value {
    //     println!("three");
    // }


    // #[derive(Debug)] // So we can inspect the state in a minute
    // enum UsState {
    //     Alabama,
    //     Alaska,
    //     // ... etc
    // }
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }

    // // 쿼터가 아닌 모든 동전을 세고 싶은 동시에 쿼터 동전일 경우 또한 알려주고 싶다면
    // let mut count = 0;

    // let coin = Coin::Penny;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    // if let &Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }




}
