pub fn serie_geométrica<const TAMAÑO:usize>() -> [u32;TAMAÑO]{
    let mut arr = [0;TAMAÑO];
    
    if TAMAÑO > 0 {
        arr[0] = 1;
        for i in 1..TAMAÑO {
            arr[i] = arr[i-1] * 2;
        }
    }

    return arr;
}

#[cfg(test)]
mod testing_ejercicio15{
    use crate::tp2::ej15;
    
    #[test]
    fn caso_especial() {
        assert_eq!(ej15::serie_geométrica::<0>(),[0;0]);
        assert_eq!(ej15::serie_geométrica(),[0;0]);
    }

    #[test]
    fn arrays_geometricos(){
        let arr1 = ej15::serie_geométrica::<1>();
        assert_eq!(arr1.len(),1);
        assert_eq!(arr1[0],1);

        let arr1 = ej15::serie_geométrica::<5>();
        assert_eq!(arr1.len(),5);
        assert_eq!(arr1[arr1.len()-1],16);

    }
}