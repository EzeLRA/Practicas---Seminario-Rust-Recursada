pub fn es_primo(x:u32) -> bool {
	return match x {
		0 | 1 => false,
		2 | 3 => true,
		_ => {
			let mut num = x-1;
			loop{
				
                if num < 2 {
                    break true;
                }
                
                if x % num == 0 {
                    break false;
                }
                num = num - 1;
			}
		}
	}
}

#[cfg(test)]
mod testing_ejercicio2{
	use crate::tp2::ej2;

	#[test]
	fn numeros_primos_casos_especiales(){
		assert!(!ej2::es_primo(0) | !ej2::es_primo(1),"No deberian ser primos");
		assert!(ej2::es_primo(2) | ej2::es_primo(3) , "Deberian ser primos");
	}

	#[test]
	fn numeros_primos(){
		for num in [5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97] {
			assert!(ej2::es_primo(num),"{num} deberia ser primo");
		}
		
	}

	#[test]
	fn numeros_no_primos(){
		for num in [4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32, 33, 34, 35, 36, 38, 39, 40, 42, 44, 45, 46, 48, 49, 50, 51, 52, 54, 55, 56, 57, 58, 60, 62, 63, 64, 65, 66, 68, 69, 70, 72, 74, 75, 76, 77, 78, 80, 81, 82, 84, 85, 86, 87, 88, 90, 91, 92, 93, 94, 95, 96, 98, 99, 100] {
			assert!(!ej2::es_primo(num),"{num} no deberia ser primo");
		}
		
	}
}