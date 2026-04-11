fn main() {
    const NUMBER : u8 = 10;
    let mut arr : [u8;6] = [2,4,3,5,6,7];
    for i in 0..arr.len() {
        arr[i] = arr[i] * NUMBER;
        println!("{}",arr[i]);
    }
    
}
