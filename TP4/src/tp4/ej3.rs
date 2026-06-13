/*
	Estructuras secundarias 
*/

#[derive(PartialEq,Debug,Clone)]
enum TipoSuscripcion{
	Basic,
	Clasic,
	Super
}

#[derive(PartialEq, Debug, Clone)]
struct InfoMercadoPago {
    alias: String,
    cuil: String,
}

#[derive(PartialEq, Debug, Clone)]
struct InfoTransferencia {
    cbu: String,
    banco: String,
}

#[derive(PartialEq, Debug, Clone)]
struct InfoTarjeta {
    numero_tarjeta: String,
    franquicia: String, 
}

#[derive(PartialEq, Debug, Clone)]
struct InfoCripto {
    wallet_address: String,
    red: String,
}

#[derive(PartialEq,Debug,Clone)]
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
	pub fn new()->Plataforma{
		return Plataforma { 
			usuarios: Vec::new(), 
			registro_suscripciones: Vec::new() 
		}
	}
	pub fn registrar_usuario(&mut self,u:&Usuario)->bool{
		if !self.usuarios.iter().any(|user| user == u) {
			self.usuarios.push(u.clone());
		}else{
			return false
		}
		return true
	}
	pub fn registrar_contrato(&mut self,c:&ContratoSuscripcion)->bool{
		if !self.registro_suscripciones.iter().any(|s| s.dni_igual(c.dni_usuario) && s.activo){
			self.registro_suscripciones.push(c.clone());
		}else{
			return false
		}
		return true
	}
	fn usuario_en_sistema(&self,u:&Usuario)->bool{
		return self.usuarios.iter().any(|user| user == u)
	}
	pub fn upgrade(&mut self,u:&Usuario)->bool{
		let mut exito = false;
		if self.usuario_en_sistema(&u){
			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				let mut sus_nuevo = sus.clone();
				if sus_nuevo.upgrade_tipo(){
					sus.cancelar_suscripcion();
					self.registro_suscripciones.push(sus_nuevo);
					exito = true;
				}
			}
		}
		return exito
	}
	pub fn downgrade(&mut self,u:&Usuario)->bool{
		let mut exito = false;
		if self.usuario_en_sistema(&u){
			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				let mut sus_nuevo = sus.clone();
				sus.cancelar_suscripcion();
				if sus_nuevo.downgrade_tipo(){
					self.registro_suscripciones.push(sus_nuevo);
				}
				exito = true;
			}
		}
		return exito
	}
	pub fn cancelar_suscripcion(&mut self,u:&Usuario)->bool{
		let mut exito = false;
		if self.usuario_en_sistema(&u){
			if let Some(sus) = self.registro_suscripciones.iter_mut().rev().find(|s| s.dni_igual(u.get_dni()) && s.activo){
				sus.cancelar_suscripcion();
				exito = true;
			}
		}
		return exito
	}
}