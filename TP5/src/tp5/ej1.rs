use std::{fmt::Display, path};
use serde::{Serialize, Deserialize};
use serde_json;
use std::{fs::File, io::{Error, Read, Write}};
use std::io;

/*
	Nueva implementacion - Ejercicio1
*/

/*
	Tipos de errores
*/
#[derive(Debug,PartialEq)]
enum error_baja{
	Inexistente(String),
	EstructuraVacia(String)
}

impl Display for error_baja{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self{
			error_baja::Inexistente(val) => write!(f, "No se encontro el auto en la consecionaria {} ",val),
			error_baja::EstructuraVacia(val) => write!(f, "La consecionaria {} no dispone de autos ",val)
		}
	}
}

#[derive(Debug)]
struct error_capacidad(String);
impl Display for error_capacidad{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "La Capacidad de autos para la consecionaria {} fue superada" , self.0)
	}
}

#[derive(Debug)]
enum Errores{
	ErrorBaja(error_baja),
	ErrorExistencia,
	ErrorCapacidad(error_capacidad),
	ErrorIO(io::Error),
	ErrorSerde(serde_json::Error)
}
impl Display for Errores{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Errores::ErrorBaja(err) => write!(f,"{}",err),
			Errores::ErrorExistencia => write!(f, "Ya existe un auto con tales caracteristicas"),
			Errores::ErrorCapacidad(err) => write!(f,"{}",err),
		    Errores::ErrorIO(err) => write!(f, "Error de E/S al guardar: {}", err),
            Errores::ErrorSerde(err) => write!(f, "Error de serialización: {}", err)
		}
	}
}
//Implementacion para el uso del operador (?)
impl std::error::Error for Errores {}

//Implementacion automatica errores subyacentes
impl From<io::Error> for Errores {
    fn from(err: io::Error) -> Self {
        Errores::ErrorIO(err)
    }
}

impl From<serde_json::Error> for Errores {
    fn from(err: serde_json::Error) -> Self {
        Errores::ErrorSerde(err)
    }
}

/*
	Extraccion del ejercicio 7 - TP3 
	"Correcciones hechas a partir de feedbacks a partir del entregable de la primer fecha , solo aplicadas
	a la implementacion original"
	Consideracion: Se aplican las nuevas funciones sobre el struct base (Concesionaria)
*/

//Enum
#[derive( Serialize, Deserialize,Debug,Clone)]
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
#[derive(Debug,Clone,Serialize,Deserialize)]
struct Auto{
    marca : String,
    modelo : String,
    anio : u32,
    precio_bruto : f32,
    color : Colores
}

#[derive(Serialize, Deserialize, Debug)]
struct ConcesionarioAuto{
	nombre : String,
	direccion : String,
	capacidad : u32,
	autos : Vec<Auto>,
	path : String
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
	pub fn get_color(&self)->Colores{
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
	pub fn new(nom:&String,dir:&String,cant:u32,path_in:&str)->ConcesionarioAuto{
		let autos_lista : Vec<Auto> = match ConcesionarioAuto::recuperar_informacion(&path_in){
			Ok(dato) => {
                dato
            }
            Err(_) => {
                Vec::new()
            }
		};
		return ConcesionarioAuto{
			nombre : nom.clone(),
			direccion : dir.clone(),
			capacidad : cant,
			autos: autos_lista,
			path: path_in.to_string()
		}
	}
	/*
		Nuevos metodos del tp5
	*/
	fn recuperar_informacion(path:&str)-> Result<Vec<Auto>,Errores>{
		let file = File::open(path).map_err(Errores::ErrorIO)?;
		let autos: Vec<Auto> = serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
		Ok(autos)
	}
	fn guardar_informacion(&self) -> Result<(), Errores> {
	    let mut file = File::create(&self.path)?;
	    let serialized = serde_json::to_string(&self.autos)?;
        file.write_all(serialized.as_bytes())?;
		return Ok(())
    }
	//Agrega el auto recibido y no pueden existir repetidos dentro de la consecionaria
	pub fn agregar_auto(&mut self,auto:Auto)->Result<(),Errores>{
		if (self.autos.len() as u32) < self.capacidad {
			for a in &self.autos{
				if a.es_igual_a(&auto){
					return Err(Errores::ErrorExistencia)
				}
			}
			self.autos.push(auto);
			self.guardar_informacion()?;
			return Ok(())
		}
		return Err(Errores::ErrorCapacidad(error_capacidad(self.get_nombre())));
	}
	//Elimina un auto con las caracteristicas exactas
	pub fn eliminar_auto(&mut self,a1:&Auto)->Result<(), Errores>{
		if !self.autos.is_empty() {
			let mut pos = None;
			for i in 0..self.autos.len(){
				if self.autos[i].es_igual_a(&a1) {
					pos = Some(i);
					break;
				}
			}
			if let Some(pos_in) = pos{
				self.autos.remove(pos_in);
				self.guardar_informacion()?;
				return Ok(())
			}
			return Err(Errores::ErrorBaja(error_baja::Inexistente(self.get_nombre())))
		}
		return Err(Errores::ErrorBaja(error_baja::EstructuraVacia(self.get_nombre())))
	}
	
	//Busca un auto con las caracteristicas exactas
	pub fn buscar_auto(&self,a1:&Auto)->Option<&Auto>{
		let mut res = None;
		if !self.autos.is_empty() {
			for auto in &self.autos{
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
mod testing_ejercicio1{
	use crate::tp5::ej1::*;

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
		//Identificar colores primarios
		let a1 = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,100000.0,&Colores::Rojo);
		assert_eq!(a1.calcular_precio(), 125000.0);

		let a1 = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,200000.0,&Colores::Azul);
		assert_eq!(a1.calcular_precio(), 250000.0);

		let a1 = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,300000.0,&Colores::Amarillo);
		assert_eq!(a1.calcular_precio(), 375000.0);
		
		
		//Identificar colores secundarios
		let a2 = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,100000.0,&Colores::Verde);
		assert_eq!(a2.calcular_precio(), 90000.0);

		let a2 = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,50000.0,&Colores::Blanco);
		assert_eq!(a2.calcular_precio(), 45000.0);

		let a2 = Auto::new(&String::from("asd"),&String::from("aytuiy"),2023,15000.0,&Colores::Negro);
		assert_eq!(a2.calcular_precio(), 13500.0);
		
		
		//Identificar marca y conjunto de colores
		let a_bmw = Auto::new(&String::from("BMW"),&String::from("aytuiy"),2023,100000.0,&Colores::Verde);
		assert_eq!(a_bmw.calcular_precio(), 105000.0);

		let a_bmw = Auto::new(&String::from("BMW"),&String::from("aytuiy"),2023,200000.0,&Colores::Amarillo);
		assert_eq!(a_bmw.calcular_precio(), 280000.0);


		//Identificar antigüedad y conjunto de colores
		let a_antiguo = Auto::new(&String::from("asd"),&String::from("aytuiy"),2000,100000.0,&Colores::Rojo);
		assert_eq!(a_antiguo.calcular_precio(), 125000.0);

		let a_antiguo = Auto::new(&String::from("asd"),&String::from("aytuiy"),1999,100000.0,&Colores::Amarillo);
		assert_eq!(a_antiguo.calcular_precio(), 120000.0);

		//Reconocer condiciones mixtas
		let a_mixto = Auto::new(&String::from("BMW"),&String::from("aytuiy"),1995,100000.0,&Colores::Amarillo);
		assert_eq!(a_mixto.calcular_precio(), 135000.0);
	}

	/*
		Concensionaria
	*/

	fn crear_conjunto_autos()->Vec<Auto>{
		let mut res = Vec::new();
		let auto1 = Auto::new(&String::from("BMW"),&String::from("a234"),1995,100000.0,&Colores::Amarillo);
		let auto2 = Auto::new(&String::from("Piolita"),&String::from("b321"),1999,100000.0,&Colores::Verde);
		let auto3 = Auto::new(&String::from("Tesla"),&String::from("c521"),2020,1000000.0,&Colores::Negro);
		let auto4 = Auto::new(&String::from("Tojota"),&String::from("f931"),2019,500000.0,&Colores::Rojo);
		res.push(auto1);
		res.push(auto2);
		res.push(auto3);
		res.push(auto4);
		return res
	}

	#[test]
	fn creacion_consecionaria(){
		let conse1 = ConcesionarioAuto::new(&"Conse1".to_string(),&"Av1".to_string(),4,"./lista_autos.json");
		assert_eq!(conse1.es_igual_a(&ConcesionarioAuto::new(&"Conse1".to_string(),&"Av1".to_string(),4,"./lista_autos.json")),true);
	}

	#[test]
	fn operatoria_consecionaria(){
		let mut conse1 = ConcesionarioAuto::new(&"Conse1".to_string(),&"Av1".to_string(),4,"./lista_autos.json");
		//Agregar autos
		let autos = crear_conjunto_autos();
		for a in autos{
			assert!(conse1.agregar_auto(a).is_ok(),"Aqui no debio fallar");
		}

		//Intentar superar limite de espacio
		let auto_nuevo = Auto::new(&String::from("Bicho"),&String::from("j221"),2020,1000000.0,&Colores::Negro);
		assert!(conse1.agregar_auto(auto_nuevo.clone()).is_err_and(|e|{
			//Validar mensaje no nulo
			assert!(!e.to_string().is_empty());
			matches!(e,Errores::ErrorCapacidad(_))
		}),"Aquí debió fallar por superar la capacidad");
		
		//Intentar borrar un auto no agregado
		assert!(conse1.eliminar_auto(&auto_nuevo).is_err_and(|e|{
			//Validar mensaje no nulo
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorBaja(_))
		}),"Aquí debió fallar el eliminar un auto inexistente");

		//Continuar las operaciones con otra consecionaria 
		//(Demostracion de persistencia - traslado de datos a otra instancia en caso de "cambio")
		let mut conse2 = ConcesionarioAuto::new(&"Conse2".to_string(),&"Av2".to_string(),4,"./lista_autos.json");

		//Eliminar todos los autos
		let autos = crear_conjunto_autos();
		for a in autos{
			assert!(conse2.eliminar_auto(&a).is_ok(),"Aqui no debio fallar");
		}
		//Intentar borrar un auto en una lista vacia
		assert!(conse2.eliminar_auto(&auto_nuevo).is_err_and(|e| matches!(e, Errores::ErrorBaja(_))),"Aquí debió fallar el eliminar un auto en una estructura vacia");

		//Agregar y buscar un nuevo auto
		let auto_nuevo = Auto::new(&String::from("Bichito"),&String::from("j221"),2020,1000000.0,&Colores::Negro);
		assert!(conse2.agregar_auto(auto_nuevo.clone()).is_ok(),"Aqui no debio fallar");

		if let Some(a) = conse2.buscar_auto(&auto_nuevo){
			assert_eq!(a.es_igual_a(&auto_nuevo),true);
		}else{
			panic!("Aqui no tendria que haber fallado");
		}

		assert!(conse2.eliminar_auto(&auto_nuevo).is_ok(),"Aqui no debio fallar");
		assert_eq!(conse2.buscar_auto(&auto_nuevo).is_none(),true);
	}

	/*
		Casos especiales para la cobertura de coverage
	*/
	#[test]
	fn caso_especial_error_io() {
		// Se buscara forzar un ErrorIO usando una ruta cuyo directorio base NO EXISTE
		let path_imposible = "./carpeta_inexistente_123/autos.json";

		let mut conse = ConcesionarioAuto::new(&"ConseBackRoom".to_string(), &"Av 67".to_string(), 5, path_imposible);
		let auto = Auto::new(&"Ford".to_string(), &"Ka".to_string(), 2015, 60000.0, &Colores::Verde);

		// Al intentar agregar el auto, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO
		assert!(conse.agregar_auto(auto).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorIO(_))
		}),"Aquí debió fallar");
	}

	#[test]
	fn caso_especial_error_serde() {
		let path_err = "./corrupto.json";
		
		// Se fuerza la escritura en el contenido temporal que NO cumple con el formato estructurado de un .JSON válido
		assert!(std::fs::write(path_err, "{ &&5435#$#$&42365_XXXX1234 : [::: ").is_ok(),"No debio fallar aqui");

		// Se invoca directamente el método para leer el archivo del path que buscara

		assert!(ConcesionarioAuto::recuperar_informacion(path_err).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorSerde(_))
		}),"Aquí debió fallar");

		assert!(std::fs::remove_file(path_err).is_ok(),"Error fuera de lo previsto");
		
	}

}
