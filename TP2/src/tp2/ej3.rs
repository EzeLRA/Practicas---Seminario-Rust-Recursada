pub fn suma_pares<const N:usize>(arr : [i32;N]) -> i32 {
    let mut res = 0;
    for num in arr{
        if num % 2 == 0{
            res = res + num;
        }
    }
    return res;
}

#[cfg(test)]
mod testing_ejercicio3{
    use crate::tp2::ej3;

    #[test]
    fn casos_especiales(){
        let arr : [i32;0] = [0;0];
        assert_eq!(ej3::suma_pares(arr),0);
        let arr : [i32;2] = [3;2];
        assert_eq!(ej3::suma_pares(arr),0);
    }

    #[test]
    fn array_mixto(){
        let arr = [0,1,2,3,4,5,6,7,8,9,10];
        assert_eq!(ej3::suma_pares(arr),30);
    }
}