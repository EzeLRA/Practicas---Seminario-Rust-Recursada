
//Trait para numeros enteros

pub trait ProcesadorEnteros{
	fn es_primo(&self)->bool;
}

impl ProcesadorEnteros for u32{
	fn es_primo(&self)->bool{
		let mut cumple = true;

		let mut i = 2;
		while(i < *self)&&(cumple){
			if(self % i) == 0{
				cumple = false;
			}
			i += 1;
		}

		return cumple;
	}
}

pub fn cantidad_numeros_primos(v1:&Vec<u32>)->u32{
	
	let it1 = v1.iter();
	//Recibe el iterador y lo filtra para retornar en un vector con solo los numeros primos (y solo obtener el tamaño del vector)
	let res: Vec<_> = it1.filter( |x| x.es_primo() ).collect();

	return res.len() as u32;
}

#[cfg(test)]
mod test_ejercicio1{
	use super::*;

	#[test]
	fn vector_numeros_primos(){
		let v1 : Vec<u32> = vec![1,2,3,4,5,8,7];
		assert_eq!(cantidad_numeros_primos(&v1), 5);
	}

}