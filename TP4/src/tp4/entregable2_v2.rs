/*
	Extraccion Fecha - TP3 EJ3
*/

//Atributos
#[derive(Debug,Clone)]
pub struct Fecha{
    pub dia : u8,
    pub mes : u8,
    pub anio : u16
}

/*
    Metodos
*/

impl Fecha{

    //Metodos Secundarios
    pub fn get_dia(&self)->u8{
        return self.dia;
    }
    pub fn get_mes(&self)->u8{
        return self.mes;
    }
    pub fn get_anio(&self)->u16{
        return self.anio;
    }
    pub fn es_igual_a(&self,f:&Fecha)->bool{
        return if(self.get_dia() == f.get_dia())&&(self.get_mes() == f.get_mes())&&(self.get_anio() == f.get_anio()){true}else{false}
    }
    /*
        Metodos Primarios    
     */
    pub fn new(d:u8,m:u8,a:u16)->Fecha{
        return Fecha { dia: d , mes: m , anio: a }
    }
    pub fn es_fecha_valida(&self)->bool{
        
        if (self.mes > 0) && (self.mes <= 12) && (self.anio > 0) && (self.dia > 0) {
        
            match self.mes{
                2 => if self.es_bisiesto() { return self.dia <= 29 }else{ return self.dia <= 28},
                9|4|6|11 => return self.dia <= 30,
                _ => return self.dia <= 31
            }
            
        }

        return false;
    }

    pub fn es_bisiesto(&self)->bool{
        return (self.anio % 4)==0;
    }

    //Auxiliar para determinar el ultimo dia de un mes
    fn ultimo_dia(&self)->u8{
        match self.mes{
            2 => if self.es_bisiesto() {29}else{28},
            9|4|6|11 => 30,
            _ => 31
        }
    
    }

    //Auxiliar para avanzar de mes y anio
    fn avanzar_mes(&mut self) {
        if self.mes == 12 {
            self.mes = 1;
            self.anio += 1;
        } else {
            self.mes += 1;
        }
        self.dia = 1;
    }

    //Se considera que la fecha es valida
    pub fn sumar_dias(&mut self,mut dias_sumar:u32){
        //Bucle principal para el calculo
        while dias_sumar > 0 {
            //Obtiene el ultimo dia del mes (Cantidad total de dias que le corresponde)
            let dias_mes = self.ultimo_dia();
            //Calcula el resto de dias que debera actualizar en "dias_sumar" para avanzar en mes y anio hasta llegar al mes con la cantidad minima a sumar de dias correspondiente
            let dias_restantes = dias_mes - self.dia + 1;
            
            //Avanza en los meses y anios(si fuera necesario) hasta llegar al mes y sumar la cantidad minima de dias
            if dias_sumar >= dias_restantes as u32 {
                dias_sumar -= dias_restantes as u32;
                self.avanzar_mes();
            } else {
                //Suma la cantidad correspondiente al mes
                self.dia += dias_sumar as u8;
                //Fin de ejecucion
                dias_sumar = 0;
            }
        }

    }

    //Auxiliar para retroceder de mes y anio
    fn retroceder_mes(&mut self){
        if self.mes == 1{
            self.mes = 12;
            self.anio -= 1;
        } else {
            self.mes -= 1;
        }
        self.dia = self.ultimo_dia();
    }

    //Se considera que la fecha es valida
    //Y que no se llegara a una fecha negativa(anio negativo)
    pub fn restar_dias(&mut self, mut dias_restar:u32){
        //Bucle principal para el calculo
        while dias_restar > 0 {
            
            //Retrocede en los meses y anios(si fuera necesario) hasta llegar al mes y restar la cantidad minima de dias
            if dias_restar >= self.dia as u32 {
                dias_restar -= self.dia as u32;
                self.retroceder_mes();
            } else {
                //Resta la cantidad correspondiente al mes
                self.dia -= dias_restar as u8;
                //Fin de ejecucion
                dias_restar = 0;
            }
        }
    }

    pub fn es_mayor(&self , f:&Fecha)->bool{
        return if self.anio > f.anio {true}else 
        if (self.anio == f.anio) && (self.mes > f.mes) {true}else 
        if (self.mes == f.mes) && (self.dia > f.dia) {true}else{false};
    }

}


#[cfg(test)]
mod testing_ejercicio3{
    use super::Fecha;

    #[test]
    fn creacion_fecha(){
        let f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 1, 2025)),true);
    }

    #[test]
    fn validacion_de_fecha(){
        let mut f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_fecha_valida(),true);
        f = Fecha::new(31, 2, 2004);
        assert_eq!(f.es_fecha_valida(),false);
    }

    #[test]
    fn validar_bisiesto(){
        let mut f = Fecha::new(1, 1, 2028);
        assert_eq!(f.es_bisiesto(),true);
        f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_bisiesto(),false);
    }

    #[test]
    fn adicion_fecha(){
        let mut f = Fecha::new(1, 1, 2028);
        f.sumar_dias(30);
        assert_eq!(f.es_igual_a(&Fecha::new(31, 1, 2028)),true);
        f.sumar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 2, 2028)),true);
        f.sumar_dias(29);
        assert_eq!(f.es_igual_a(&Fecha::new(1,3,2028)),true);
    }

    #[test]
    fn sustraccion_fecha(){
        let mut f = Fecha::new(10, 4, 2028);
        f.restar_dias(9);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 4, 2028)),true);
        f.restar_dias(31);
        assert_eq!(f.es_igual_a(&Fecha::new(1,3,2028)),true);
        f.restar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(29, 2, 2028)),true);
    }

    #[test]
    fn comparacion_fechas(){
        let f1 = Fecha::new(25, 5, 2000);
        let f2 = Fecha::new(25, 2, 2004);
        assert_eq!(f1.es_mayor(&f2),false);
        assert_eq!(f2.es_mayor(&f1),true);
    }

}
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
//Como no se usa el trait PartialEq , lo comento para eliminar el conflicto que origina el struct Fecha y respeto la implementacion orignal del mismo , en base al TP3
#[derive(/*PartialEq,*/Debug,Clone)]
struct ContratoSuscripcion{
	//Referencia al usuario
	dni_usuario : u64,
	//Datos de contrato para la misma
	tipo_suscripcion : TipoSuscripcion,
	activo : bool,
	costo_mensual : f64,
	duracion_mes : u8,
	fecha_inicio : Fecha,
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

/* 

	Como hubo un error con lo que solicitiba la practica 4 en base al campo fecha
	se usara el struct Fecha de la practica 3 para el mismo.
	-Ya reclamado presencialmente en el dia de la fecha de la evaluacion  

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
*/
//Estaciones (Cada uno implementara los numeros del mes para usarlo en el filtrado)
enum TemporadaAnio{
	Otonio(u8),
	Invierno(u8),
	Primavera(u8),
	Verano(u8) 
}

//Nuevo struct (EstacionTop) - Corrigo Plataforma -> Estacion 
//Agrego los traits despues
struct EstacionTop{
	cant_suscripciones : u64,
	temporada : TemporadaAnio ,
	anio : u64
}

impl EstacionTop{
	pub fn new(te:TemporadaAnio,an:u64)->EstacionTop{
		return EstacionTop{
			cant_suscripciones : 0,
			temporada : te,
			anio: an
		}
	}
	//pub fn es_otonio(&self,f:Fecha)->bool;
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
	pub fn new(dni:u64,tipo:TipoSuscripcion,costo:f64,cant:u8,fecha:Fecha,medio:MediosDePago)->ContratoSuscripcion{
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
		let mut res = None;
		//Como mi codigo no usa alguna estructura para el campo fecha debere procesar la fecha (mala mia :c)
		let activas = self.listado_suscripciones(true);
		if !activas.is_empty(){
			//let fecha = Fecha::convertir_fecha(s.fecha);
			//let mut suscripciones_tempo : HashMap<EstacionTop,TemporadaAnio> = Vec::new();
			//Recorro solo sus activas
			activas.iter().for_each(|s|{
				//Recorro y filtro por temporada
				//Aca tendre que usar la fecha para concretar la temporada 
				//suscripciones_tempo.insert(..)
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
		//Correccion de la fecha
		let mut s1 = ContratoSuscripcion::new(1234,TipoSuscripcion::Basic, 100.0, 2, Fecha::new(12, 5, 2026), MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(1234,TipoSuscripcion::Super, 100.0, 2, Fecha::new(12, 5, 2026), MediosDePago::Efectivo);
		
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

		//Correccion Fecha
		let mut s1 = ContratoSuscripcion::new(1234, TipoSuscripcion::Basic, 1000.0, 5, Fecha::new(20, 1, 2025), MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(12345, TipoSuscripcion::Super, 5000.0, 5, Fecha::new(20, 1, 2025), MediosDePago::Efectivo);
		let mut s3 = ContratoSuscripcion::new(2345, TipoSuscripcion::Super, 5000.0, 5, Fecha::new(20, 1, 2025), MediosDePago::Efectivo);

		assert!(sistema.registrar_contrato(s1.clone()));
		assert!(sistema.registrar_contrato(s2));
		assert!(!sistema.registrar_contrato(s3));
		assert!(!sistema.registrar_contrato(s1));
		let mut s2 = ContratoSuscripcion::new(12345, TipoSuscripcion::Clasic, 5000.0, 5, Fecha::new(12, 8, 2026), MediosDePago::Efectivo);
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

		//Correcion Fecha
		let mut s1 = ContratoSuscripcion::new(1234, TipoSuscripcion::Basic, 1000.0, 5, Fecha::new(12, 5, 2026), MediosDePago::Efectivo);
		let mut s2 = ContratoSuscripcion::new(12345, TipoSuscripcion::Super, 5000.0, 5, Fecha::new(12, 5, 2026) , MediosDePago::Efectivo);
	
		sistema.registrar_contrato(s1);
		sistema.registrar_contrato(s2);

		assert!(sistema.downgrade(&user2));
		assert!(!sistema.upgrade(&user1));

		assert_eq!(sistema.registro_suscripciones.len(),2);

		assert!(!sistema.upgrade(&user2));
		assert!(sistema.downgrade(&user1));
		assert!(sistema.downgrade(&user1));

		s1 = ContratoSuscripcion::new(1234, TipoSuscripcion::Clasic, 2500.0, 5, Fecha::new(25, 7, 2026), MediosDePago::Efectivo);
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

		let s1 = ContratoSuscripcion::new(12345, TipoSuscripcion::Basic, 1000.0, 5, Fecha::new(20, 6, 2026), MediosDePago::Efectivo);
		let s2 = ContratoSuscripcion::new(1234, TipoSuscripcion::Basic, 1000.0, 5, Fecha::new(20, 6, 2026), MediosDePago::MercadoPago(InfoMercadoPago { alias: "zapato".to_string(), cuil: 123456 }));
		let s3 = ContratoSuscripcion::new(4554, TipoSuscripcion::Basic, 1000.0, 5, Fecha::new(20, 6, 2026), MediosDePago::Criptomoneda(InfoCripto { wallet_address : "asd2354tg42t".to_string(), red: "%#1234".to_string() }));
		let s4 = ContratoSuscripcion::new(3487, TipoSuscripcion::Basic, 1000.0, 5, Fecha::new(20, 6, 2026), MediosDePago::Efectivo);

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