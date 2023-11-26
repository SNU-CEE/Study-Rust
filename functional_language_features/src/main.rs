// 티셔츠 회사가 프로모션으로 옷을 증정하는 상황
// 만약 어떤 사람이 좋아하는 색상이 있다면 그걸 한 장 주고
// 없다면 지금 재고가 가장 많은 색상을 한 장 주고 싶다.

use std::{time::Duration, thread};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // user_preference가 Option<ShirtColor>이기 때문에
        // 값이 있다면 Some: Option<T> -> return value within the Some
        // 값이 없다면 None -> closure를 call

    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    // 함수와 다르게 capture the environment가 가능, closure 내에서 정의되지 않은 값도 전달 가능
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;


    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);


    // Closures can capture values from their environment in three ways
    // 1. borrowing immutably, 2. borrowing mutably, and 3. taking ownership


    // // closure that captures an immutable reference
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);


    // closure that captures a mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);


    // force the closure to take ownership of the values -> move 키워드를 사용하자
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Since our new thread runs in parallel, the stack frame containing list may well have disappeared by the time we try to use them
    // 그래서 move를 붙여야 함
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();


    // Closures will automatically implement one, two, or all three of these Fn traits
    // Fn traits에는 FnOnce, FnMut, Fn 3가지 trait이 있음.
    // FnOnce는 한 번만 call되는 closures에 적용됨, A closure that moves captured values out of its body: Ownership을 가져감
    // FnMut은 closures that don’t move captured values out of their body, 그렇지만 captured values mutate 가능, 여러번 called 가능
    // Fn은 Capture와 Mutate 둘 다 X

    // unwrap_or_else의 정의 예시: FnOnce
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    // standard library method, sort_by_key 예시: FnMut
    // it calls the closure multiple times
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // list.sort_by_key(|r| r.width);
    // println!("{:#?}", list);


    // 이 경우에는 에러 발생
    // Closure의 String을 sort_operations 벡터로 옮기려고 하는데 한 번만 가능하기 때문에 2번 이상부터 불가능
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);


    // // // Closure의 body를 수정해서 environment 밖으로 values를 옮기지 않게 하면 해결된다.
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // X move outside, but mutate captured values
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

}