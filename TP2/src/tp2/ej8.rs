pub fn sumar_arreglos<const N:usize>(arr1 : [f32;N], arr2 : [f32;N]) -> [f32;N]{
    let mut res = [0.0;N];
    
    for i in 0..N{
        res[i] = arr1[i] + arr2[i];
    }

    return res;
}

#[cfg(test)]
mod testing_ejercicio8{
    use crate::tp2::ej8;

    #[test]
    fn caso_especial(){
        let arr1 = [0.0;0];
        let arr2 = [0.0;0];
        assert_eq!(ej8::sumar_arreglos(arr1, arr2),[0.0;0]);
        assert_eq!(ej8::sumar_arreglos(arr1, arr1),[0.0;0]);
    }

    #[test]
    fn arreglos_comunes(){
        let arr1 = [0.0,1.2,-1.0,-1.2];
        let arr2 = [0.0,0.5,1.0,-2.5];
        assert_eq!(ej8::sumar_arreglos(arr1, arr2),[0.0,1.7,0.0,-3.7]);
    }

}