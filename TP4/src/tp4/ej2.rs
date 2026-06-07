use std::collections::LinkedList;

#[derive(Debug, Clone)]
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

impl<'a> Persona<'a>{
	fn new(nom:&'a str,ape:&'a str,dir:&'a str,ci:&'a str,s:f64,e:u8)->Persona<'a>{
		return Persona{
			nombre : nom,
			apellido : ape,
			direccion : dir,
			ciudad : ci,
			salario : s,
			edad : e
		}
	}
	//Metodos secundarios
	fn obtener_nombre(&self)->String{
		return self.nombre.to_string().clone();
	}
	fn obtener_apellido(&self)->String{
		return self.apellido.to_string().clone();
	}
	fn obtener_direccion(&self)->String{
		return self.direccion.to_string().clone();
	}
	fn obtener_ciudad(&self)->String{
		return self.ciudad.to_string().clone();
	}
	fn obtener_salario(&self)->f64{
		return self.salario;
	}
	fn obtener_edad(&self)->u8{
		return self.edad;
	}
	fn es_igual_a(&self, p: &Persona<'a>)->bool{
		return (self.nombre == p.obtener_nombre())&&(self.apellido == p.obtener_apellido())&&(self.direccion == p.obtener_direccion())&&
		(self.ciudad == p.obtener_ciudad())&&(self.salario == p.obtener_salario())&&(self.edad == p.obtener_edad());
	}

	//Metodos primarios
	fn salario_mayor_a(&self,salario_in:f64)->bool{
		return self.salario > salario_in;
	}
}

//Modulo A
pub fn salarios_mayores_a<'a>(vector_p : &Vec<Persona<'a>>, monto: f64) -> LinkedList<Persona<'a>>{
    return vector_p.iter().filter(|p| p.obtener_salario() > monto).cloned().collect()
}

//Modulo B
pub fn ciudadanos_mayores_a<'a>(vector_p : &Vec<Persona<'a>>, edad:u8 , nom_ciu : &String)-> LinkedList<Persona<'a>>{
	return vector_p.iter().filter(|p| (p.obtener_edad() > edad)&&(p.obtener_ciudad() == *nom_ciu)).cloned().collect()
}

//Modulo C
pub fn ciudadanos_pertenecientes_a<'a>(vector_p : &Vec<Persona<'a>> , nom_ciu : &String)->bool{
    return vector_p.iter().all(|p| p.obtener_ciudad() == *nom_ciu)
}

//Modulo D
pub fn ciudadanos_existentes_en<'a>(vector_p : &Vec<Persona<'a>> , nom_ciu : &String)->bool{
    return vector_p.iter().any(|p| p.obtener_ciudad() == *nom_ciu)
}

//Modulo E
pub fn persona_existente<'a>(vector_p : &Vec<Persona<'a>> , per : &Persona<'a>)->bool{
    return vector_p.iter().any(|p| p.es_igual_a(&per))
}

//Modulo F
pub fn obtener_edades<'a>(vector_p : &Vec<Persona<'a>>)->Vec<u8>{
    return vector_p.iter().map(|p| p.obtener_edad()).collect()
}

//Modulo G
pub fn obtener_salarios_max_min<'a>(vector_p : &Vec<Persona<'a>>)->Option<(Persona<'a>,Persona<'a>)>{
    let mut res = None;
    
    if !vector_p.is_empty(){
        let mut min = &vector_p[0]; 
        let mut max = &vector_p[0]; 
        vector_p.iter().for_each(|persona|{
            match persona.salario {
                s if s>max.salario => max = persona,
                s if s<min.salario => min = persona,
                s if (s==max.salario) && (persona.edad>max.edad) => max = persona,
                s if (s==min.salario) && (persona.edad>min.edad) => min = persona,
                _ => (),
            }
        });
        res = Some((max.clone(),min.clone()));
    }

    return res
}

#[cfg(test)]
mod test_ejercicio2{
	use super::*;

	/*
		Metodos auxiliares
	*/

	fn retornar_test_vector1<'a>()->Vec<Persona<'a>>{
		let mut vector : Vec<Persona> = Vec::new();
		vector.push(Persona::new("Carlos","Maro","AvSanMartin","Buenos Aires",1500.0,10));
		vector.push(Persona::new("Maria","Mercedes","AvBelgrano","Buenos Aires",20000.0,25));
		vector.push(Persona::new("Julian","Wen","AvLibertad","Buenos Aires",28000.0,28));
		vector.push(Persona::new("Marcos","Deroga","AvMoron","Chaco",100000.0,50));
		return vector
	}

	fn retornar_test_vector2<'a>()->Vec<Persona<'a>>{
		let mut vector : Vec<Persona> = Vec::new();
		vector.push(Persona::new("Mateo","Parro","AvSanMartin","Buenos Aires",1500.0,12));
		vector.push(Persona::new("Carlos","Maro","AvSanMartin","Buenos Aires",1500.0,10));
		vector.push(Persona::new("Maria","Mercedes","AvBelgrano","Buenos Aires",20000.0,25));
		vector.push(Persona::new("Julian","Wen","AvLibertad","Buenos Aires",280000.0,28));
		vector.push(Persona::new("Juan","Cruz","AvLibertad","Buenos Aires",280000.0,38));
		vector.push(Persona::new("Matozo","Deroga","AvMoron","Buenos Aires",100000.0,50));
		return vector
	}

	/*
		Modulos para testing
	*/

	#[test]
	fn vector_vacio(){
		let mut vec_vacio = Vec::new();
		assert!(salarios_mayores_a(&vec_vacio, 1000.0).is_empty());
		assert!(ciudadanos_mayores_a(&vec_vacio,20,&"Buenos Aires".to_string()).is_empty());
		assert_eq!(ciudadanos_pertenecientes_a(&vec_vacio,&"Marmol".to_string()),true);	//Considerando que los metodos de iterator retornan true para vec vacios por defecto
		assert_eq!(ciudadanos_existentes_en(&vec_vacio,&"Lanus".to_string()),false); //Al contrario de arriba retorna false
		assert_eq!(persona_existente(&vec_vacio,&Persona::new(&"Matias",&"Ponzi",&"Alvear & 12", &"Argentino", 800000.0, 29)),false);
		assert!(obtener_edades(&vec_vacio).is_empty());
		assert!(obtener_salarios_max_min(&vec_vacio).is_none());
	}

	#[test]
	fn vector_con_personas(){
		let mut vector = retornar_test_vector1();
		assert_eq!(salarios_mayores_a(&vector, 10000.0).len(),3);
		assert_eq!(ciudadanos_mayores_a(&vector,20,&"Buenos Aires".to_string()).len(),2);
		assert_eq!(ciudadanos_mayores_a(&vector,20,&"Misiones".to_string()).len(),0);
		assert_eq!(ciudadanos_pertenecientes_a(&vector,&"Chaco".to_string()),false);
		assert_eq!(ciudadanos_pertenecientes_a(&retornar_test_vector2(),&"Buenos Aires".to_string()),true);
		assert_eq!(ciudadanos_existentes_en(&vector,&"Chaco".to_string()),true);
		assert_eq!(ciudadanos_existentes_en(&vector,&"Salta".to_string()),false);
		assert_eq!(persona_existente(&vector,&Persona::new("Maria","Mercedes","AvBelgrano","Buenos Aires",20000.0,25)),true);
		assert_eq!(persona_existente(&vector,&Persona::new("Maria","Mercedes","AvBelgrano","Buenos Aires",20000.0,17)),false);
		assert_eq!(obtener_edades(&vector).len(),4);
		assert!(obtener_salarios_max_min(&vector).is_some());
		if let Some(personas) = obtener_salarios_max_min(&vector){
			assert_eq!(personas.0.obtener_edad(),50);
			assert_eq!(personas.1.obtener_edad(),10);
		}
		if let Some(personas) = obtener_salarios_max_min(&retornar_test_vector2()){
			assert_eq!(personas.0.obtener_edad(),38);
			assert_eq!(personas.1.obtener_edad(),12);
		}
	}
}

