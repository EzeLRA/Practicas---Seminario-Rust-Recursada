/*
	Estructuras secundarias 
*/

use core::hash;
use std::collections::HashMap;

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
enum TipoSuscripcion{
	Basic,
	Clasic,
	Super
}

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
struct InfoMercadoPago {
    alias: String,
    cuil: u128,
}

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
struct InfoTransferencia {
    cbu: u128,
    banco: String,
}

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
struct InfoTarjeta {
    numero_tarjeta: u128,
    franquicia: String, 
}

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
struct InfoCripto {
    wallet_address: String,
    red: String,
}

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
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

#[derive(PartialEq,Debug,Clone)]
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
	Nueva implementacion entregable 2 
*/
//Fecha tupla
struct Fecha(u8,u8,u64);

impl Fecha{
	pub fn convertir_fecha(f:u64)->Fecha{
		//Tendre que hacer el proceso
		//..
		let mut a = ..
		let mut m = ..
		let mut d = ..
		return Fecha(d,m,a)
	}
}

//Estaciones (Cada uno implementara los numeros del mes para usarlo en el filtrado)
enum TemporadaAnio{
	Otonio(..),
	Invierno(..),
	Primavera(..),
	Verano(..) 
}

//Nuevo struct
//Agrego los traits despues
struct PlataformaTop{
	cant_suscripciones : u64,
	temporada : TemporadaAnio ,
	anio : u64
}

impl PlataformaTop{
	pub fn new(te:TemporadaAnio,an:u64)->PlataformaTop{
		return PlataformaTop{
			cant_suscripciones : 0,
			temporada : te,
			anio: u64
		}
	}
	pub fn es_otonio(&self,f:Fecha)->bool;
	//.. hacer para las demas temporadas para simplificar el proceso
}

/*
	Estructura plataforma
*/

struct Plataforma{
	usuarios : Vec<Usuario>,
	registro_suscripciones : Vec<ContratoSuscripcion>
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
	pub fn new(dni:u64,tipo:TipoSuscripcion,costo:f64,cant:u8,fecha:u64,medio:MediosDePago)->ContratoSuscripcion{
		return ContratoSuscripcion { 
			dni_usuario: dni,
			tipo_suscripcion: tipo, 
			activo: true, 
			costo_mensual: costo, 
			duracion_mes: cant, 
			fecha_inicio: fecha, 
			tipo_pago: medio
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
	pub fn new()->Plataforma{
		return Plataforma { 
			usuarios: Vec::new(), 
			registro_suscripciones: Vec::new() 
		}
	}
	//Los dni son unicos
	fn usuario_en_sistema(&self,user_dni:u64)->bool{
		return self.usuarios.iter().any(|user| user.get_dni() == user_dni)
	}
	pub fn registrar_usuario(&mut self,u:Usuario)->bool{
		if !self.usuario_en_sistema(u.get_dni()){
			self.usuarios.push(u);
		}else{
			return false
		}
		return true
	}
	pub fn registrar_contrato(&mut self,c:ContratoSuscripcion)->bool{
		if self.usuario_en_sistema(c.dni_usuario)&&(!self.registro_suscripciones.iter().any(|s| s.dni_igual(c.dni_usuario) && s.activo)){
			self.registro_suscripciones.push(c);
		}else{
			return false
		}
		return true
	}
	pub fn upgrade(&mut self,u:&Usuario)->bool{
		let mut exito = false;
		if self.usuario_en_sistema(u.get_dni()){
			let mut dato = None;

			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				let mut sus_nuevo = sus.clone();
				if sus_nuevo.upgrade_tipo(){
					sus.cancelar_suscripcion();
					dato = Some(sus_nuevo);
					exito = true;
				}
			}

			if let Some(sus_nuevo) = dato{
				self.registro_suscripciones.push(sus_nuevo);
			}
		}
		return exito
	}
	pub fn downgrade(&mut self,u:&Usuario)->bool{
		let mut exito = false;
		if self.usuario_en_sistema(u.get_dni()){
			let mut dato = None;

			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				let mut sus_nuevo = sus.clone();
				sus.cancelar_suscripcion();
				if sus_nuevo.downgrade_tipo(){
					dato = Some(sus_nuevo);
				}
				exito = true;
			}
			if let Some(sus_nuevo) = dato {
				self.registro_suscripciones.push(sus_nuevo);
			}
		}
		return exito
	}
	pub fn cancelar_suscripcion(&mut self,u:&Usuario)->bool{
		let mut exito = false;
		if self.usuario_en_sistema(u.get_dni()){
			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				sus.cancelar_suscripcion();
				exito = true;
			}
		}
		return exito
	}
	fn listado_suscripciones(&self,activos:bool)->Vec<&ContratoSuscripcion>{
		return self.registro_suscripciones.iter().filter(|s| s.activo == activos).collect()
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

	/*
		Nueva implementacion para la platadorma
	*/
	pub fn estacion_con_mas_suscripciones(&self)->Option<EstacionTop>{
		let mut res = None
		//Como mi codigo no usa alguna estructura para el campo fecha debere procesar la fecha (mala mia :c)
		let activas = self.listado_suscripciones(true);
		if !activas.is_empty(){
			let fecha = Fecha::convertir_fecha(s.fecha);
			let mut suscripciones_tempo : HashMap<PlataformaTop,TemporadaAnio> = Vec::new();
			//Recorro solo sus activas
			activas.iter().for_each(|s|{
				//Recorro y filtro por temporada
				//Aca tendre que usar la fecha para concretar la temporada 
				suscripciones_tempo.insert(..)
				//Continuar
			});	

		}

		return res
	}
}



#[cfg(test)]
mod test_entregable2{    
    use super::*;
	
	//listado sin activas
	#[test]
	fn sin_reporte1(){

	}

	//listado con activas
	#[test]
	fn con_reporte(){

	}

	//listado vacio
	#[test]
	fn sin_reporte2(){

	}

}

#[cfg(test)]
mod test_ejercicio3{    
    use super::*;

	#[test]
	fn cambio_suscripcion(){
		let mut s1 = ContratoSuscripcion::new(1234,TipoSuscripcion::Basic, 100.0, 2, 120526, MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(1234,TipoSuscripcion::Super, 100.0, 2, 120526, MediosDePago::Efectivo);
		
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
		let mut sistema = Plataforma::new();
		let mut user1 = Usuario::new(&"Marco", 12345);
		let mut user2 = Usuario::new(&"Marco",1234);
		
		assert!(sistema.registrar_usuario(user1.clone()));
		assert!(sistema.registrar_usuario(user2.clone()));
		assert_eq!(sistema.usuarios.len(),2);
		assert!(!sistema.registrar_usuario(user1));
		assert!(!sistema.registrar_usuario(user2));
		assert_eq!(sistema.usuarios.len(),2);

		let mut s1 = ContratoSuscripcion::new(1234, TipoSuscripcion::Basic, 1000.0, 5, 200125, MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(12345, TipoSuscripcion::Super, 5000.0, 5, 200125, MediosDePago::Efectivo);
		let mut s3 = ContratoSuscripcion::new(2345, TipoSuscripcion::Super, 5000.0, 5, 200125, MediosDePago::Efectivo);

		assert!(sistema.registrar_contrato(s1.clone()));
		assert!(sistema.registrar_contrato(s2));
		assert!(!sistema.registrar_contrato(s3));
		assert!(!sistema.registrar_contrato(s1));
		let mut s2 = ContratoSuscripcion::new(12345, TipoSuscripcion::Clasic, 5000.0, 5, 200125, MediosDePago::Efectivo);
		assert!(!sistema.registrar_contrato(s2));
		assert_eq!(sistema.listado_suscripciones(true).len(),2);
	}

	#[test]
	fn registro_operatoria(){
		let mut sistema = Plataforma::new();
		let mut user1 = Usuario::new(&"Patricio", 12345);
		let mut user2 = Usuario::new(&"Patricio",1234);
		
		sistema.registrar_usuario(user1.clone());
		sistema.registrar_usuario(user2.clone());

		let mut s1 = ContratoSuscripcion::new(1234, TipoSuscripcion::Basic, 1000.0, 5, 200125, MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(12345, TipoSuscripcion::Super, 5000.0, 5, 200125, MediosDePago::Efectivo);
	
		sistema.registrar_contrato(s1);
		sistema.registrar_contrato(s2);

		assert!(sistema.downgrade(&user2));
		assert!(!sistema.upgrade(&user1));

		assert_eq!(sistema.registro_suscripciones.len(),2);

		assert!(!sistema.upgrade(&user2));
		assert!(sistema.downgrade(&user1));
		assert!(sistema.downgrade(&user1));

		s1 = ContratoSuscripcion::new(1234, TipoSuscripcion::Clasic, 2500.0, 5, 200125, MediosDePago::Efectivo);
		sistema.registrar_contrato(s1);
		assert!(sistema.cancelar_suscripcion(&user1));

		assert_eq!(sistema.registro_suscripciones.len(),5);
		assert_eq!(sistema.listado_suscripciones(true).len(),1);
		assert_eq!(sistema.listado_suscripciones(false).len(),4);
	}

	fn construir_sistema()->Plataforma{
		let mut sistema = Plataforma::new();
		let user1 = Usuario::new(&"Patricio", 12345);
		let user2 = Usuario::new(&"Patricio",1234);
		let user3 = Usuario::new(&"Matias", 4554);
		let user4 = Usuario::new(&"David",3487);

		sistema.registrar_usuario(user1);
		sistema.registrar_usuario(user2);
		sistema.registrar_usuario(user3);
		sistema.registrar_usuario(user4);

		let s1 = ContratoSuscripcion::new(12345, TipoSuscripcion::Basic, 1000.0, 5, 200125, MediosDePago::Efectivo);
		let s2 = ContratoSuscripcion::new(1234, TipoSuscripcion::Basic, 1000.0, 5, 200125, MediosDePago::MercadoPago(InfoMercadoPago { alias: "zapato".to_string(), cuil: 123456 }));
		let s3 = ContratoSuscripcion::new(4554, TipoSuscripcion::Basic, 1000.0, 5, 200125, MediosDePago::Criptomoneda(InfoCripto { wallet_address : "asd2354tg42t".to_string(), red: "%#1234".to_string() }));
		let s4 = ContratoSuscripcion::new(3487, TipoSuscripcion::Basic, 1000.0, 5, 200125, MediosDePago::Efectivo);

		sistema.registrar_contrato(s1);
		sistema.registrar_contrato(s2);
		sistema.registrar_contrato(s3);
		sistema.registrar_contrato(s4);

		return sistema
	}

	#[test]
	fn validacion_maximos(){
		let mut sis = Plataforma::new();

		//Plataforma vacia
		assert!(sis.metodopago_max_suscripciones_activas().is_none());
		assert!(sis.tipo_suscripcion_max_activas().is_none());
		assert!(sis.metodopago_max_suscripciones_inactivas().is_none());
		assert!(sis.tipo_suscripcion_max_inactivas().is_none());

		sis = construir_sistema();

		//Suscripciones activas
		if let Some(max) = sis.metodopago_max_suscripciones_activas(){
			assert_eq!(max,MediosDePago::Efectivo);
		}else{
			panic!("Debio de retornar un maximo");
		}
		if let Some(max) = sis.tipo_suscripcion_max_activas(){
			assert_eq!(max,TipoSuscripcion::Basic);
		}else{
			panic!("Debio de retornar un maximo");
		}
		assert!(sis.metodopago_max_suscripciones_inactivas().is_none());
		assert!(sis.tipo_suscripcion_max_inactivas().is_none());

		//Con registro de operaciones con suscripciones
		sis.downgrade(&Usuario::new(&"Patricio", 12345)); 
		sis.downgrade(&Usuario::new(&"Patricio", 1234));
		sis.upgrade(&Usuario::new(&"Matias", 4554));
		sis.upgrade(&Usuario::new(&"David",3487));
		sis.upgrade(&Usuario::new(&"David",3487));
		sis.cancelar_suscripcion(&Usuario::new(&"David",3487));

		//Activas
		if let Some(max) = sis.metodopago_max_suscripciones_activas(){
			assert!(matches!(max, MediosDePago::Criptomoneda(..)));
		}else{
			panic!("Debio de retornar un maximo");
		}
		if let Some(max) = sis.tipo_suscripcion_max_activas(){
			assert_eq!(max,TipoSuscripcion::Clasic);
		}else{
			panic!("Debio de retornar un maximo");
		}
		//Inactivas
		if let Some(max) = sis.metodopago_max_suscripciones_inactivas(){
			assert!(matches!(max, MediosDePago::Efectivo));
		}else{
			panic!("Debio de retornar un maximo");
		}
		if let Some(max) = sis.tipo_suscripcion_max_inactivas(){
			assert_eq!(max,TipoSuscripcion::Basic);
		}else{
			panic!("Debio de retornar un maximo");
		}
	}

}