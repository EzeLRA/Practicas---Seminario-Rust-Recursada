/*
    Implementacion EJ4-TP5
*/
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use serde_json;
use std::{fs::File, io::{Error, Read, Write}};
use std::io;

/*
    Tipos de errores
*/

#[derive(Debug)]
enum error_operatoria{
    Inexistente(String),
    EstructuraVacia(String),
	ContratoActivo(String),
	Rechazado(String)
}

impl Display for error_operatoria{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            error_operatoria::Inexistente(val) => write!(f, "No se encontro el elemento en la estructura {} ",val),
            error_operatoria::EstructuraVacia(val) => write!(f, "La estrucutra {} no dispone de elementos ",val),
			error_operatoria::ContratoActivo(val) => write!(f,"El usuario {} ya tiene un contrato activo y no puede emplear otro nuevo",val),
			error_operatoria::Rechazado(val) => write!(f,"La operacion a sido rechazada por {}",val)
		}
    }
}

#[derive(Debug)]
enum Errores{
    ErrorOperatoria(error_operatoria),
    ErrorIO(io::Error),
    ErrorSerde(serde_json::Error)
}

impl Display for Errores{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errores::ErrorOperatoria(err) => write!(f,"{}",err),
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
	Extraccion Ejercicio 3 - TP4
	Estructuras secundarias 
*/

use core::hash;
use std::collections::HashMap;

#[derive(PartialEq,Eq,Debug,Clone,Hash,Serialize,Deserialize)]
enum TipoSuscripcion{
	Basic,
	Clasic,
	Super
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Serialize,Deserialize)]
struct InfoMercadoPago {
    alias: String,
    cuil: u128,
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Serialize,Deserialize)]
struct InfoTransferencia {
    cbu: u128,
    banco: String,
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Serialize,Deserialize)]
struct InfoTarjeta {
    numero_tarjeta: u128,
    franquicia: String, 
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Serialize,Deserialize)]
struct InfoCripto {
    wallet_address: String,
    red: String,
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Serialize,Deserialize)]
enum MediosDePago{
	Efectivo,
	MercadoPago(InfoMercadoPago),
	TransferenciaBancaria(InfoTransferencia),
	TarjetaDeCredito(InfoTarjeta),
	Criptomoneda(InfoCripto)
}

/*
	Estructuras primarias : Usuario y suscripcion
*/

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct ContratoSuscripcion{
	//Referencia al usuario
	dni_usuario : u64,
	//Datos de contrato para la misma
	tipo_suscripcion : TipoSuscripcion,
	activo : bool,
	costo_mensual : f64,
	duracion_mes : u8,
	fecha_inicio : u64,
	tipo_pago : MediosDePago
}

#[derive(PartialEq,Debug,Clone)]
struct Usuario{
	nombre : String,
	dni : u64
}

/*
	Estructura plataforma
*/

struct Plataforma{
	usuarios : Vec<Usuario>,
	registro_suscripciones : Vec<ContratoSuscripcion>,
	path: String
}

impl Usuario{
	pub fn new(nom:&str,dni_in:u64)->Usuario{
		return Usuario { nombre: nom.to_string() , dni: dni_in }
	}
	pub fn get_dni(&self)->u64{
		return self.dni
	}
}

impl ContratoSuscripcion{
	pub fn new(dni:u64,tipo:&TipoSuscripcion,costo:f64,cant:u8,fecha:u64,medio:&MediosDePago)->ContratoSuscripcion{
		return ContratoSuscripcion { 
			dni_usuario: dni,
			tipo_suscripcion: tipo.clone(), 
			activo: true, 
			costo_mensual: costo, 
			duracion_mes: cant, 
			fecha_inicio: fecha, 
			tipo_pago: medio.clone() 
		}
	}
	pub fn cancelar_suscripcion(&mut self){
		self.activo = false;
	}
	pub fn upgrade_tipo(&mut self)->bool{
		let mut exito = true;
		match self.tipo_suscripcion{
			TipoSuscripcion::Basic => self.tipo_suscripcion = TipoSuscripcion::Clasic,
			TipoSuscripcion::Clasic => self.tipo_suscripcion = TipoSuscripcion::Super,
			TipoSuscripcion::Super => exito = false,
			_ => exito = false,
		}
		
		return exito
	}
	pub fn downgrade_tipo(&mut self)->bool{
		let mut exito = true;
		
		match self.tipo_suscripcion{
			TipoSuscripcion::Super => self.tipo_suscripcion = TipoSuscripcion::Clasic,
			TipoSuscripcion::Clasic => self.tipo_suscripcion = TipoSuscripcion::Basic,
			TipoSuscripcion::Basic => exito = false,
			_ => exito = false,
		}
		
		return exito
	}
	pub fn dni_igual(&self,dni:u64)->bool{
		return self.dni_usuario == dni
	}
}

impl Plataforma{
	pub fn new(path_in:&str)->Plataforma{
		let suscripciones : Vec<ContratoSuscripcion> = match Plataforma::recuperar_informacion(path_in){
			Ok(datos) => datos,
			Err(_) => Vec::new()
		};
		return Plataforma { 
			usuarios: Vec::new(), 
			registro_suscripciones: suscripciones ,
			path: path_in.to_string()
		}
	}
	/*
		Nueva implementacion - TP5
	*/
	fn recuperar_informacion(path:&str)-> Result<Vec<ContratoSuscripcion>,Errores>{
		let file = File::open(path).map_err(Errores::ErrorIO)?;
		let suscripciones: Vec<ContratoSuscripcion> = serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
		Ok(suscripciones)
	}
	fn guardar_informacion(&self) -> Result<(), Errores> {
	    let mut file = File::create(&self.path)?;
	    let serialized = serde_json::to_string(&self.registro_suscripciones)?;
        file.write_all(serialized.as_bytes())?;
		return Ok(())
    }
	/* 
		Metodos primarios
	*/
	//Los dni son unicos
	fn usuario_en_sistema(&self,user_dni:u64)->bool{
		return self.usuarios.iter().any(|user| user.get_dni() == user_dni)
	}
	pub fn registrar_usuario(&mut self,u:&Usuario)->bool{
		if !self.usuario_en_sistema(u.get_dni()){
			self.usuarios.push(u.clone());
		}else{
			return false
		}
		return true
	}
	pub fn registrar_contrato(&mut self,c:&ContratoSuscripcion)->Result<(),Errores>{
		/* 
		if (self.usuario_en_sistema(c.dni_usuario))&&(!self.registro_suscripciones.iter().any(|s| s.dni_igual(c.dni_usuario) && s.activo)){
			self.registro_suscripciones.push(c.clone());
		}else{
			return false
		}
		return true
		*/
		if self.usuario_en_sistema(c.dni_usuario){
			if !self.registro_suscripciones.iter().any(|s| s.dni_igual(c.dni_usuario) && s.activo){
				self.registro_suscripciones.push(c.clone());
				self.guardar_informacion()?;
				return Ok(())
			}
			return Err(Errores::ErrorOperatoria(error_operatoria::ContratoActivo(c.dni_usuario.to_string())))
		}
		return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Registro de usuarios"))))
	}
	pub fn upgrade(&mut self,u:&Usuario)->Result<(),Errores>{

		if self.usuario_en_sistema(u.get_dni()){

			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				let mut sus_nuevo = sus.clone();
				if sus_nuevo.upgrade_tipo(){
					sus.cancelar_suscripcion();
					self.registro_suscripciones.push(sus_nuevo);
					self.guardar_informacion()?;
					return Ok(())
				}
				return Err(Errores::ErrorOperatoria(error_operatoria::Rechazado(String::from("limite alcanzado para hacer upgrade"))))
			}
			return Err(Errores::ErrorOperatoria(error_operatoria::Rechazado(String::from("sin suscripciones activas"))))
		}
		return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Registro de usuarios"))))
	}
	pub fn downgrade(&mut self,u:&Usuario)->Result<(),Errores>{
		
		if self.usuario_en_sistema(u.get_dni()){

			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				let mut sus_nuevo = sus.clone();
				sus.cancelar_suscripcion();
				if sus_nuevo.downgrade_tipo(){
					self.registro_suscripciones.push(sus_nuevo);
				}
				
				self.guardar_informacion()?;
				return Ok(())
			}
			return Err(Errores::ErrorOperatoria(error_operatoria::Rechazado(String::from("sin suscripciones activas para hacer downgrade"))))
		}
		return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Registro de usuarios"))))
	}
	pub fn cancelar_suscripcion(&mut self,u:&Usuario)->Result<(),Errores>{
		if self.usuario_en_sistema(u.get_dni()){
			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				sus.cancelar_suscripcion();
				self.guardar_informacion()?;
				return Ok(())
			}
			return Err(Errores::ErrorOperatoria(error_operatoria::Rechazado(String::from("sin suscripciones activas para cancelar"))))
		}
		return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Registro de usuarios"))))
	}
	fn listado_suscripciones(&self,activos:bool)->Vec<ContratoSuscripcion>{
		return self.registro_suscripciones.iter().filter(|s| s.activo == activos).cloned().collect()
	}
	pub fn metodopago_max_suscripciones_activas(&self)->Option<MediosDePago>{
		let mut res = None;
		let mut listado = self.listado_suscripciones(true);

		if !listado.is_empty(){
			let mut contador_tipos : HashMap<MediosDePago,u32> = HashMap::new();
			listado.iter().for_each(|s|
			{
				*contador_tipos.entry(s.tipo_pago.clone()).or_insert(0) += 1;
			}
			);

			res = contador_tipos.into_iter().max_by_key(|&(_,cant)| cant).map(|(tipo,_)|tipo);
		}

		return res
	}
	pub fn metodopago_max_suscripciones_inactivas(&self)->Option<MediosDePago>{
		let mut res = None;
		let mut listado = self.listado_suscripciones(false);

		if !listado.is_empty(){
			let mut contador_tipos : HashMap<MediosDePago,u32> = HashMap::new();
			listado.iter().for_each(|s|
			{
				*contador_tipos.entry(s.tipo_pago.clone()).or_insert(0) += 1;
			}
			);

			res = contador_tipos.into_iter().max_by_key(|&(_,cant)| cant).map(|(tipo,_)|tipo);
		}

		return res
	}
	pub fn tipo_suscripcion_max_activas(&self)->Option<TipoSuscripcion>{
		let mut res = None;
		let mut listado = self.listado_suscripciones(true);

		if !listado.is_empty(){
			let mut contador_tipos : HashMap<TipoSuscripcion,u32> = HashMap::new();
			listado.iter().for_each(|s|
			{
				*contador_tipos.entry(s.tipo_suscripcion.clone()).or_insert(0) += 1;
			}
			);

			res = contador_tipos.into_iter().max_by_key(|&(_,cant)| cant).map(|(tipo,_)|tipo);
		}

		return res
	}
	pub fn tipo_suscripcion_max_inactivas(&self)->Option<TipoSuscripcion>{
		let mut res = None;
		let mut listado = self.listado_suscripciones(false);

		if !listado.is_empty(){
			let mut contador_tipos : HashMap<TipoSuscripcion,u32> = HashMap::new();
			listado.iter().for_each(|s|
			{
				*contador_tipos.entry(s.tipo_suscripcion.clone()).or_insert(0) += 1;
			}
			);

			res = contador_tipos.into_iter().max_by_key(|&(_,cant)| cant).map(|(tipo,_)|tipo);
		}

		return res
	}
}

#[cfg(test)]
mod test_ejercicio3{    
    use super::*;

	#[test]
	fn cambio_suscripcion(){
		let mut s1 = ContratoSuscripcion::new(1234,&TipoSuscripcion::Basic, 100.0, 2, 120526, &MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(1234,&TipoSuscripcion::Super, 100.0, 2, 120526, &MediosDePago::Efectivo);
		
		//Cambio nulo
		assert!(!s1.downgrade_tipo());
		assert_eq!(s1.tipo_suscripcion,TipoSuscripcion::Basic);

		assert!(!s2.upgrade_tipo());
		assert_eq!(s2.tipo_suscripcion,TipoSuscripcion::Super);

		//Cambio hecho
		assert!(s1.upgrade_tipo());
		assert_eq!(s1.tipo_suscripcion,TipoSuscripcion::Clasic);

		assert!(s2.downgrade_tipo());
		assert_eq!(s2.tipo_suscripcion,TipoSuscripcion::Clasic);

	}

	#[test]
	fn registro_inicial(){
		let mut sistema = Plataforma::new("./lista_suscripciones.json");
		let mut user1 = Usuario::new(&"Marco", 12345);
		let mut user2 = Usuario::new(&"Marco",1234);
		
		assert!(sistema.registrar_usuario(&user1));
		assert!(sistema.registrar_usuario(&user2));
		assert_eq!(sistema.usuarios.len(),2);
		assert!(!sistema.registrar_usuario(&user1));
		assert!(!sistema.registrar_usuario(&user2));
		assert_eq!(sistema.usuarios.len(),2);

		let mut s1 = ContratoSuscripcion::new(1234, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(12345, &TipoSuscripcion::Super, 5000.0, 5, 200125, &MediosDePago::Efectivo);
		let mut s3 = ContratoSuscripcion::new(2345, &TipoSuscripcion::Super, 5000.0, 5, 200125, &MediosDePago::Efectivo);

		assert!(sistema.registrar_contrato(&s1).is_ok());
		assert!(sistema.registrar_contrato(&s2).is_ok());
		assert!(sistema.registrar_contrato(&s3).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
		}));
		assert!(sistema.registrar_contrato(&s1).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e,Errores::ErrorOperatoria(error_operatoria::ContratoActivo(_)))
		}));
		let mut s2 = ContratoSuscripcion::new(12345, &TipoSuscripcion::Clasic, 5000.0, 5, 200125, &MediosDePago::Efectivo);
		assert!(sistema.registrar_contrato(&s2).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e,Errores::ErrorOperatoria(error_operatoria::ContratoActivo(_)))
		}));
		assert_eq!(sistema.listado_suscripciones(true).len(),2);

		//Limpieza para prevenir acumulacion de archivos
        assert!(std::fs::remove_file("./lista_suscripciones.json").is_ok(),"Error fuera de lo previsto");
	}

	
	#[test]
	fn registro_operatoria(){
		let mut sistema = Plataforma::new("./lista_suscripciones2.json");
		let mut user1 = Usuario::new(&"Patricio", 12345);
		let mut user2 = Usuario::new(&"Patricio",1234);
		
		sistema.registrar_usuario(&user1);
		sistema.registrar_usuario(&user2);

		//Suscripcion para user2
		let mut s1 = ContratoSuscripcion::new(1234, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::Efectivo);
		//Suscripcion para user1
		let mut s2 = ContratoSuscripcion::new(12345, &TipoSuscripcion::Super, 5000.0, 5, 200125, &MediosDePago::Efectivo);
	
		assert!(sistema.registrar_contrato(&s1).is_ok());
		assert!(sistema.registrar_contrato(&s2).is_ok());

		assert!(sistema.downgrade(&user2).is_ok());
		assert!(sistema.upgrade(&user1).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e,Errores::ErrorOperatoria(error_operatoria::Rechazado(_)))
		}));

		assert_eq!(sistema.registro_suscripciones.len(),2);

		assert!(sistema.upgrade(&user2).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e,Errores::ErrorOperatoria(error_operatoria::Rechazado(_)))
		}));
		assert!(sistema.downgrade(&user1).is_ok());
		assert!(sistema.downgrade(&user1).is_ok());

		s1 = ContratoSuscripcion::new(1234, &TipoSuscripcion::Clasic, 2500.0, 5, 200125, &MediosDePago::Efectivo);
		assert!(sistema.registrar_contrato(&s1).is_ok());
		assert!(sistema.cancelar_suscripcion(&user1).is_ok());

		assert_eq!(sistema.registro_suscripciones.len(),5);
		assert_eq!(sistema.listado_suscripciones(true).len(),1);
		assert_eq!(sistema.listado_suscripciones(false).len(),4);

		//Limpieza para prevenir acumulacion de archivos
        assert!(std::fs::remove_file("./lista_suscripciones2.json").is_ok(),"Error fuera de lo previsto");
	
	} 
	
	fn construir_sistema()->Result<Plataforma,Errores>{
		let mut sistema = Plataforma::new("./lista_suscripciones3.json");
		let user1 = Usuario::new(&"Patricio", 12345);
		let user2 = Usuario::new(&"Patricio",1234);
		let user3 = Usuario::new(&"Matias", 4554);
		let user4 = Usuario::new(&"David",3487);

		sistema.registrar_usuario(&user1);
		sistema.registrar_usuario(&user2);
		sistema.registrar_usuario(&user3);
		sistema.registrar_usuario(&user4);

		let s1 = ContratoSuscripcion::new(12345, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::Efectivo);
		let s2 = ContratoSuscripcion::new(1234, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::MercadoPago(InfoMercadoPago { alias: "zapato".to_string(), cuil: 123456 }));
		let s3 = ContratoSuscripcion::new(4554, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::Criptomoneda(InfoCripto { wallet_address : "asd2354tg42t".to_string(), red: "%#1234".to_string() }));
		let s4 = ContratoSuscripcion::new(3487, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::Efectivo);

		sistema.registrar_contrato(&s1)?;
		sistema.registrar_contrato(&s2)?;
		sistema.registrar_contrato(&s3)?;
		sistema.registrar_contrato(&s4)?;

		return Ok(sistema)
	}

	#[test]
	fn validacion_maximos(){
		let mut sis = Plataforma::new("./lista_suscripciones_vacio.json");

		//Plataforma vacia
		assert!(sis.metodopago_max_suscripciones_activas().is_none());
		assert!(sis.tipo_suscripcion_max_activas().is_none());
		assert!(sis.metodopago_max_suscripciones_inactivas().is_none());
		assert!(sis.tipo_suscripcion_max_inactivas().is_none());

		assert!(construir_sistema().is_ok_and(|s|{
			sis = s;
			true
		}),"Aqui no debio fallar");

		//Suscripciones activas
		assert!(sis.metodopago_max_suscripciones_activas().is_some_and(|res|{
			matches!(res,MediosDePago::Efectivo)
		}),"Debio retornar un maximo");

		assert!(sis.tipo_suscripcion_max_activas().is_some_and(|res|{
			matches!(res,TipoSuscripcion::Basic)
		}),"Debio retornar un maximo");

		assert!(sis.metodopago_max_suscripciones_inactivas().is_none());
		assert!(sis.tipo_suscripcion_max_inactivas().is_none());
		
		//Con registro de operaciones con suscripciones
		assert!(sis.downgrade(&Usuario::new(&"Patricio", 12345)).is_ok());
		assert!(sis.downgrade(&Usuario::new(&"Patricio", 1234)).is_ok());
		assert!(sis.upgrade(&Usuario::new(&"Matias", 4554)).is_ok());
		assert!(sis.upgrade(&Usuario::new(&"David",3487)).is_ok());
		assert!(sis.upgrade(&Usuario::new(&"David",3487)).is_ok());
		assert!(sis.cancelar_suscripcion(&Usuario::new(&"David",3487)).is_ok());

		//Activas
		assert!(sis.metodopago_max_suscripciones_activas().is_some_and(|res|{
			matches!(res,MediosDePago::Criptomoneda(_))
		}),"Debio retornar un maximo");

		assert!(sis.tipo_suscripcion_max_activas().is_some_and(|res|{
			matches!(res,TipoSuscripcion::Clasic)
		}),"Debio retornar un maximo");

		//Inactivas
		assert!(sis.metodopago_max_suscripciones_inactivas().is_some_and(|res|{
			matches!(res,MediosDePago::Efectivo)
		}),"Debio retornar un maximo");

		assert!(sis.tipo_suscripcion_max_inactivas().is_some_and(|res|{
			matches!(res,TipoSuscripcion::Basic)
		}),"Debio retornar un maximo");
		
		//Limpieza para prevenir exceso de archivos
		assert!(std::fs::remove_file("./lista_suscripciones3.json").is_ok(),"Error fuera de lo previsto");
	}

	/*
		Casos especiales para la cobertura de coverage
	*/
	#[test]
	fn caso_especial_error_io_() {
		// Se buscara forzar un ErrorIO usando una ruta cuyo directorio base no existe
		let path_err = "./carpeta_inexistente_123/x.json";

		let mut sis = Plataforma::new(path_err);
        let user1 = Usuario::new(&"Patricio", 12345);

		sis.registrar_usuario(&user1);
		
		let s1 = ContratoSuscripcion::new(12345, &TipoSuscripcion::Basic, 1000.0, 5, 200125, &MediosDePago::Efectivo);
	
        // Al intentar registrar una suscripcion, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO
        
		assert!(sis.registrar_contrato(&s1).is_err_and(|e|{
                assert!(!e.to_string().is_empty());
			    matches!(e, Errores::ErrorIO(_))
        }),"Ocurrio un error imprevisto");
	}


	#[test]
	fn caso_especial_error_serde() {
		let path_err = "./corrupto.json";
		
		// Se fuerza la escritura en el contenido temporal que NO cumple con el formato estructurado de un .JSON válido
		assert!(std::fs::write(path_err, "{ &&5435#$#$&42365_XXXX1234 : [::: ").is_ok(),"No debio fallar aqui");

		// Se invoca directamente el método para leer el archivo del path que buscara

		assert!(Plataforma::recuperar_informacion(path_err).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorSerde(_))
		}),"Aquí debió fallar");

		assert!(std::fs::remove_file(path_err).is_ok(),"Error fuera de lo previsto");
		
	}
	
}