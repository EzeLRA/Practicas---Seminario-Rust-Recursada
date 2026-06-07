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
pub fn ciudadano_pertenecientes_a<'a>(vector_p : &Vec<Persona<'a>> , nom_ciu : &String)->bool{
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
