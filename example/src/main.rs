fn main() {
    let s = String::from("응애"); // 변수 s가 문자열의 owner가 됨
    print_data(s);   // 변수 s가 파라미터로 전달되면서 소유권이 move됨.
    // 여기서부터는 변수 s 사용 못함.

    // println!("{}", s);

}
 
fn print_data(data: String) {  // 파라미터 data가 문자열의 새 owner가 됨
    println!("{}", data);      // 문자열 사용
}                              // 여기서 문자열 메모리 drop 함