fn main() {
    let arr : [u8;5] = [2,5,8,9,1];
    let mut total : u8 = 0;

    for num in arr{
        total += num;
    }

    println!("{}",total);
}
