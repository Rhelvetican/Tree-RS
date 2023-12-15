fn main() {
    let i: i64 = 0;
    while i < 1000000000 {
        let i = i + 1;
    }
    println!("{}", i);
}
