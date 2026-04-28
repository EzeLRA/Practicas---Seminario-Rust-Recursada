
pub fn es_par(x:u32)->bool{
    return x % 2 == 0;
}

#[cfg(test)]
mod testing_ejercicio1{
    use crate::tp2::ej1;

    #[test]
    fn numero_par(){
        for x in [2,4,8,10,20,100,1000] {
            assert!(ej1::es_par(x));
        }
    }

    #[test]
    fn numero_impar(){
        for x in [3,1,5,7,11,101,1001] {
            assert!(!ej1::es_par(x));
        }
    }
}