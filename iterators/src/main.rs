fn main() {

    // In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }


    // 모든 iterators가 implement하는 trait Iterator의 정의
    pub trait Iterator {
        type Item;
    
        fn next(&mut self) -> Option<Self::Item>;
    
        // methods with default implementations elided
    }


    // // 나태한 러스트 iterator 때문에 warning 발생!!!!!!!!
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);
    for val in v1.iter().map(|x| x + 1) {
        println!("Got: {}", val);
    }

    // collect 메소드를 사용하면 해결된다
    // This method consumes the iterator and collects the resulting values into a collection data type
    // let v1: Vec<i32> = vec![1, 2, 3];
    // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // assert_eq!(v2, vec![2, 3, 4]);

}



// shoes_in_size 함수는 shoes 벡터의 ownership과 shoe size를 parameter로 가짐
// shoes_in_size에서 into_iter를 call해서 vector의 ownership을 가져가는 iterator를 만든다!!!

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // mut 삭제하면 에러 발생. v1_iter는 mutable이어야 함 -> code consumes, or uses up, the iterator
    // 헷갈리지만 .next call로부터 얻는 값들은 immutable references to the values in the vector

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // sum 콜 이후로 v1_iter 사용할 수 없음 -> sum이 iterator의 ownership을 강탈했기 때문에!!!!!!! 
    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}