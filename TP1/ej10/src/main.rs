fn main() {
    let arr1 : [u8;5] = [2,4,6,8,10];
    let arr2 : [u8;5] = [1,3,5,7,9];
    let mut arr3 : [u8;5] = [0;5];

    for i in 0..arr1.len(){
        arr3[i] = arr1[i] + arr2[i];
    }

    //Resultado
    //println!("{:?}",arr3);
}
