pub fn reemplazar_pares<const N:usize>(arr : &mut [i32;N]){
    for i in 0..N{
        if arr[i] % 2 == 0{
            arr[i] = -1;
        }
    }
}

#[cfg(test)]
mod testing_ejercicio12{
    use crate::tp2::ej12;

    #[test]
    fn caso_especial(){
        let mut arr = [0;0];
        ej12::reemplazar_pares(&mut arr);
        assert_eq!(arr,[0;0]);
    }

    #[test]
    fn arreglo_impar(){
        let mut arr = [1,3,5,7,5];
        ej12::reemplazar_pares(&mut arr);
        assert_eq!(arr,[1,3,5,7,5]);
    }

    #[test]
    fn arreglo_par(){
        let mut arr = [2,4,0,0,8];
        ej12::reemplazar_pares(&mut arr);
        assert_eq!(arr,[-1;5]);
    }

}