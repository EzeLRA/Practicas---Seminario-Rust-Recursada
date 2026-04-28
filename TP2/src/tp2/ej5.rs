pub fn duplicar_valores<const N:usize>(arr : [f32;N])->[f32;N]{
    let mut res : [f32;N] = [0.0;N];
    for i in 0..N{
        res[i] = arr[i] * 2.0;
    }
    return res;
}

#[cfg(test)]
mod testing_ejercicio5{
    use crate::tp2::ej5;

    #[test]
    fn arreglos_especiales(){
        let arr = [0.0;0];
        assert_eq!(ej5::duplicar_valores(arr),[0.0;0]);
        let arr = [0.0;5];
        assert_eq!(ej5::duplicar_valores(arr),[0.0;5]);
    }

    #[test]
    fn arreglo_comun(){
        let arr1 = [2.0,3.2,4.7,5.2];
        let arr2 = [4.0,6.4,9.4,10.4];
        assert_eq!(ej5::duplicar_valores(arr1),arr2);
    }
}