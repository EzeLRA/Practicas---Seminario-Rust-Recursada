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
