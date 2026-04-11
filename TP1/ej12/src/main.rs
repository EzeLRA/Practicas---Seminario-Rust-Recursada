fn main() {
    let tup : (String,[u8;10]) = (String::from("Hola!"),[1,2,3,4,5,6,7,8,9,10]);
    let mut total : u8 = 0;

    println!("{}",&tup.0);
    for num in &tup.1{
        total += num;
    }
    println!("{}",total);

}
