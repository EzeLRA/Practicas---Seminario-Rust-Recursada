
//Enum
#[derive(Debug , Clone)]
enum Colores{	
	//Primarios
	Rojo,
	Azul,
	Amarillo,
	//Secundarios
	Verde,
	Blanco,
	Negro
}
//Funcionalidad del enum
impl Colores{
	//Determina si es primario o secundario
	pub fn es_primario(&self)->bool{
		matches!(self, Colores::Rojo | Colores::Azul | Colores::Amarillo)
	}
	pub fn es_igual_a(&self, c: &Colores) -> bool {
        match (self, c) {
            (Colores::Rojo, Colores::Rojo) => true,
            (Colores::Azul, Colores::Azul) => true,
            (Colores::Verde, Colores::Verde) => true,
			(Colores::Amarillo, Colores::Amarillo) => true,
			(Colores::Blanco, Colores::Blanco) => true,
			(Colores::Negro, Colores::Negro) => true,
            _ => false
        }
    }
}

//Atributos
#[derive(Debug , Clone)]
struct Auto{
    marca : String,
    modelo : String,
    anio : u32,
    precio_bruto : f32,
    color : Colores
}

#[derive(Debug)]
struct ConcesionarioAuto{
	nombre : String,
	direccion : String,
	capacidad : u32,
	autos : Vec<Auto>
}

//Metodos
impl Auto{
	
	pub fn new(nom:&String,model:&String,anio_in:u32,precio:f32,color_in:&Colores)->Auto{
		return Auto{
			marca : nom.clone(),
			modelo : model.clone(),
			anio : anio_in,
			precio_bruto : precio,
			color : color_in.clone()
		}
	}

	pub fn calcular_precio(&self)->f32{
		let mut recargo : f32 = 0.0;
		let mut descuento : f32 = 0.0;

		if self.color.es_primario() {
			recargo += (self.precio_bruto * 25.0)/100.0; 
		}else{
			descuento += (self.precio_bruto * 10.0)/100.0;
		}

		if self.marca == "BMW" {
			recargo += (self.precio_bruto * 15.0)/100.0
		}

		if self.anio < 2000 {
			descuento += (self.precio_bruto * 5.0)/100.0;
		}

		return self.precio_bruto + recargo - descuento;
	}

	//Metodos secundarios
	pub fn get_marca(&self)->String{
		return self.marca.clone()
	}
	pub fn get_modelo(&self)->String{
		return self.modelo.clone()
	}
	pub fn get_anio(&self)->u32{
		return self.anio
	}
	pub fn get_precio_bruto(&self)->f32{
		return self.precio_bruto
	}
	pub fn get_color(&self)->Color{
		return self.color.clone()
	}
	pub fn es_igual_a(&self,a:&Auto)->bool{
		return (self.marca == a.get_marca())&&(self.modelo == a.get_modelo())&&(self.anio == a.get_anio())&&(self.precio_bruto == a.get_precio_bruto())&&(self.color.es_igual_a(&a.get_color()));
	}

}

impl ConcesionarioAuto{
	//Metodos secundarios
	pub fn get_nombre(&self)->String{
		return self.nombre.clone()
	}
	pub fn get_direccion(&self)->String{
		return self.direccion.clone()
	}
	pub fn get_capacidad(&self)->u32{
		return self.capacidad
	}
	pub fn es_igual_a(&self,c:&ConcesionarioAuto)->bool{
		return (self.nombre == c.get_nombre())&&(self.direccion == c.get_direccion())&&(self.capacidad == c.get_capacidad());
	}
	//Metodos primarios
	pub fn new(nom:&String,dir:&String,cant:u32)->ConcesionarioAuto{
		return ConcesionarioAuto{
			nombre : nom.clone(),
			direccion : dir.clone(),
			capacidad : cant,
			autos:Vec::new()
		}
	}
	//Preserva OwnerShip y agrega repetidos
	pub fn agregar_auto(&mut self,auto:&Auto)->bool{
		if (self.autos.len() as u32) < self.capacidad {
			self.autos.push(auto.clone());
			return true;
		}else{
			return false;
		}
	}
	//Elimina la primer ocurrencia para un auto con las caracteristicas exactas
	pub fn eliminar_auto(&mut self,a1:&Auto){
		if !self.autos.is_empty() {
			for i in 0..self.autos.len(){
				if let Some(auto) = self.autos.get(i){
					if auto.es_igual_a(&a1) {
						self.autos.remove(i);
						break;
					}
				}
			}
		}
	}
	
	//Busca un auto con las caracteristicas exactas
	pub fn buscar_auto(&self,a1:&Auto)->Option<Auto>{
		let mut res : Option<Auto> = None;
		if !self.autos.is_empty() {
			for auto in self.autos.clone(){
				if auto.es_igual_a(&a1) {
					res = Some(auto);
					break;
				}
			}
		}
		return res;
	}
}

#[cfg(test)]
mod testing_ejercicio7{
	use crate::tp3::ej7::{Colores,Auto,ConcesionarioAuto};

	/*
		Auto
	*/

	#[test]
	fn creacion_auto(){
		let a = Auto::new(&String::from("asdf"),&String::from("aytuiy"),2023,100432.0,&Colores::Rojo);
		assert_eq!(a.es_igual_a(&Auto::new(&String::from("asdf"),&String::from("aytuiy"),2023,100432.0,&Colores::Rojo)),true);
	}

	#[test]
	fn calculo_precio_auto(){
		//Identificar colores
		let a = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,100000.0,&Colores::Rojo);
		assert_eq!(a.calcular_precio(),125000.0);
		let a = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,100000.0,&Colores::Verde);
		assert_eq!(a.calcular_precio(),90000.0);
		//Identificar marca
		let a = Auto::new(&String::from("BMW"),&String::from("aytuiy"),2023,100000.0,&Colores::Verde);
		assert_eq!(a.calcular_precio(),105000.0);
		//Identificar antiguedad
		let a = Auto::new(&String::from("asd"),&String::from("aytuiy"),2000,100000.0,&Colores::Rojo);
		assert_eq!(a.calcular_precio(),125000.0);
	}

	/*
		Concensionaria
	*/

	#[test]
	fn creacion_consecionaria(){
		let conse1 = ConcesionarioAuto::new(&"asd".to_string(),&"tryertw".to_string(),10);
		assert_eq!(conse1.es_igual_a(&ConcesionarioAuto::new(&"asd".to_string(),&"tryertw".to_string(),10)),true);
	}

	#[test]
	fn operatoria_consecionaria(){
		let a1 = Auto::new(&String::from("asdf"),&String::from("aytuiy"),2023,100432.0,&Colores::Rojo);
		let a2 = Auto::new(&String::from("BMW"),&String::from("ajytjt"),2000,200500.0,&Colores::Verde);
		let mut conse1 = ConcesionarioAuto::new(&"asd".to_string(),&"tryertw".to_string(),3);
		//Limite de incersiones
		assert_eq!(conse1.agregar_auto(&a1),true);
		assert_eq!(conse1.agregar_auto(&a1),true);
		assert_eq!(conse1.agregar_auto(&a2),true);
		assert_eq!(conse1.agregar_auto(&a2),false);
		//Borra auto "a1"(primera recurrencia)
		conse1.eliminar_auto(&a1);

		//Busqueda de auto "a1"(solo encontrara al unico existente con tales caracteristicas)
		if let Some(a) = conse1.buscar_auto(&a1){
			assert_eq!(a.es_igual_a(&a1),true);
		}else{
			panic!("El auto no fue encontrado en el concesionario");
		}
		//Borra auto "a1"
		conse1.eliminar_auto(&a1);

		//Busqueda de auto "a1"(ya no lo dispone y no existe otro en la estructura)
		assert_eq!(conse1.buscar_auto(&a1).is_none(),true);
	}
}