/*
	Estructuras secundarias : Suscripciones , Medios de pago y usuarios
*/

#[derive(PartialEq,Debug,Clone)]
pub enum Suscripciones{
	Basic,
	Clasic,
	Super
}
#[derive(PartialEq,Debug,Clone)]
//No se agrego el tipo de dato que contienen cada dato porque no se piden calculos sobre la misma
pub enum Medios_de_pago{
	Efectivo,
	Mercado_pago,
	Transferencia_bancaria,
	Tarjeta_de_credito,
	Criptomoneda
}

/*
	Estructura primaria : Usuario
*/

#[derive(PartialEq,Debug,Clone)]
pub struct Suscripcion_activa{
	tipo_suscripcion : Suscripciones,
	costo_mensual : f64,
	duracion_mes : u8,
	fecha_inicio : u64,
	tipo_pago : Medios_de_pago
}

#[derive(PartialEq,Debug,Clone)]
pub struct Usuario{
	nombre : String,
	dni : u64 ,
	suscripcion_actual : Option<Suscripcion_activa>,
	suscripcion_anterior : Option<Suscripcion_activa>
}

/*
	Funcionalidades secundarias : Suscripciones , Medios de pago y usuario
*/

pub trait DatosSuscripcion{
    fn get_medio(&self)->Medios_de_pago;
    fn get_tipo(&self)->Suscripciones;
    fn set_tipo(&mut self,t:Suscripciones);
}
pub trait DatosUsuario{
	fn get_nombre(&self)->String;
	fn get_suscripcion_anterior(&self)->Option<Suscripcion_activa>;
	fn get_suscripcion_actual(&self)->Option<Suscripcion_activa>;
	fn set_suscripcion_actual(&mut self,s:&Suscripcion_activa);
}
impl DatosUsuario for Usuario{
	fn get_nombre(&self)->String {
		return self.nombre.clone();
	}
	fn get_suscripcion_anterior(&self)->Option<Suscripcion_activa>{
		if self.suscripcion_anterior.is_some() {
			return self.suscripcion_anterior.clone();
		}
		return None;
	}
	fn get_suscripcion_actual(&self)->Option<Suscripcion_activa>{
		if self.suscripcion_actual.is_some() {
			return self.suscripcion_actual.clone();
		}
		return None;
	}
	fn set_suscripcion_actual(&mut self,s:&Suscripcion_activa){
		self.suscripcion_anterior = self.suscripcion_actual.clone();
		self.suscripcion_actual = Some(s.clone());
	}
}

impl DatosSuscripcion for Suscripcion_activa {
	fn get_medio(&self)->Medios_de_pago{
		return self.tipo_pago.clone();
	}
	 fn get_tipo(&self)->Suscripciones{
		return self.tipo_suscripcion.clone();
	}
	 fn set_tipo(&mut self,t:Suscripciones){
		self.tipo_suscripcion = t.clone();
	}
}

/*
	Funcionalidades primarias para usuario
*/

impl Suscripcion_activa{
	//Funciones primarias
	fn crear_suscripcion(tipo:Suscripciones,monto:f64,duracion:u8,fecha_ini : u64,metodo_pago:Medios_de_pago)->Suscripcion_activa{
		return Suscripcion_activa{
			tipo_suscripcion : tipo,
			costo_mensual : monto,
			duracion_mes : duracion,
			fecha_inicio : fecha_ini,
			tipo_pago : metodo_pago
		}
	}
	fn upgrade(&mut self)->bool
	{
		match self.get_tipo(){
			Suscripciones::Basic =>	self.set_tipo(Suscripciones::Clasic),
			Suscripciones::Clasic => self.set_tipo(Suscripciones::Super),
			Suscripciones::Super => return false,
		}
		return true;
	}
	fn downgrade(&mut self)->bool
	{
		match self.get_tipo(){
			Suscripciones::Basic =>	return false,
			Suscripciones::Clasic => self.set_tipo(Suscripciones::Basic),
			Suscripciones::Super => self.set_tipo(Suscripciones::Clasic),
		}
		return true;
	}
}

impl Usuario{
	fn new(nom:&String,dni_in:u64,s:Option<Suscripcion_activa>)->Usuario{
		return Usuario{
			nombre : nom.clone(),
			dni : dni_in,
			suscripcion_actual : s,
			suscripcion_anterior : None	
		}
	}
	fn upgrade_suscripcion(&mut self)->bool{
		if let Some(mut s) = self.get_suscripcion_actual(){
			if s.upgrade(){
				self.set_suscripcion_actual(&s);
				return true;
			}
		}
		return false;
	}
	fn downgrade_suscripcion(&mut self)->bool{
		if let Some(mut s) = self.get_suscripcion_actual(){
			self.suscripcion_anterior = self.suscripcion_actual.clone();
			if !s.downgrade(){
				self.suscripcion_actual = None;
			}else{
				self.suscripcion_actual = Some(s);
			}
			return true;
		}
		return false;
	}
	fn cancelar_suscripcion(&mut self)->bool{
		if self.suscripcion_actual.is_some(){
			self.suscripcion_anterior = self.suscripcion_actual.clone();
			self.suscripcion_actual = None;
			return true;
		}
		return false;
	}
	fn es_igual_a(&self,u:&Usuario)->bool{
		return (self.nombre == u.nombre)&&(self.dni == u.dni);
	}
}

//Funcion auxiliar para obtener un maximo de un vector (u8)
fn obtener_max<const N:usize>(arr: [u8;N])->Option<usize>{
	if arr != [0;N] {
		let mut max = 0;
		arr.iter().enumerate().for_each(|(i,cantidad)| {
			if *cantidad>arr[max] {
				max = i;
			}
		});
		return Some(max);
	}
	return None;
}

pub struct Plataforma{
	usuarios : Vec<Usuario>
}
impl Plataforma{
	fn new()->Plataforma{
		return Plataforma { usuarios: Vec::new() }
	}
	fn agregar(&mut self,u2:&Usuario)->bool{
		match self.usuarios.iter().find(|us| us.es_igual_a(&u2)) {
            Some(_u) => return false,
            None => {self.usuarios.push(u2.clone()); return true;},
        }
	}
	fn eliminar(&mut self,u:&Usuario)->bool{
		let mut pude = false;
		if let Some(pos) = self.usuarios.iter().position(|us| us.es_igual_a(&u) ){
        	self.usuarios.remove(pos);
        	pude = true;
        }
        return pude;
	}
	fn upgrade_usuario(&mut self, usuario: &Usuario) -> bool {
        match self.usuarios.iter_mut().find(|u| u.es_igual_a(&usuario)) {
            Some(u) => return u.upgrade_suscripcion(),
            None => return false,
        }
    }
	fn downgrade_usuario(&mut self, usuario: &Usuario) -> bool {
        match self.usuarios.iter_mut().find(|u| u.es_igual_a(&usuario)) {
            Some(u) => return u.downgrade_suscripcion(),
            None => return false,
        }
    }
	fn cancelar_suscripcion(&mut self, usuario: &Usuario) -> bool {
        match self.usuarios.iter_mut().find(|u| u.es_igual_a(&usuario)) {
            Some(u) => return u.cancelar_suscripcion(),
            None => return false,
        }
    }
	fn metodo_pago_mas_usado(&self)->Option<Medios_de_pago>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Medios_de_pago> = None;

			let mut metodos_cant = [0; 5];
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_actual(){
					match s.get_medio(){
						Medios_de_pago::Efectivo => metodos_cant[0] +=1,
						Medios_de_pago::Mercado_pago => metodos_cant[1] +=1,
						Medios_de_pago::Transferencia_bancaria => metodos_cant[2] +=1,
						Medios_de_pago::Tarjeta_de_credito => metodos_cant[3] +=1,
						Medios_de_pago::Criptomoneda => metodos_cant[4] +=1,
					}
				}
			});
			
			//Retornar segun posicion el tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(metodos_cant) {
				match pos {
					0 => res = Some(Medios_de_pago::Efectivo),
					1 => res = Some(Medios_de_pago::Mercado_pago),
					2 => res = Some(Medios_de_pago::Transferencia_bancaria),
					3 => res = Some(Medios_de_pago::Tarjeta_de_credito),
					4 => res = Some(Medios_de_pago::Criptomoneda),
					_ => res = None,
				}
			}

			return res;
		}
		return None
	}
	fn suscripcion_mas_contratada(&self)->Option<Suscripciones>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Suscripciones> = None;

			let mut tipos_cant = [0; 3]; 
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_actual(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			});
			
			//Retornar segun posicion del tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(tipos_cant) {
				match pos {
					0 => res = Some(Suscripciones::Basic),
					1 => res = Some(Suscripciones::Clasic),
					2 => res = Some(Suscripciones::Super),
					_ => res = None,
				}
			}

			return res;
		}
		return None
	}
	fn metodo_pago_anterior_mas_usado(&self)->Option<Medios_de_pago>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Medios_de_pago> = None;

			let mut metodos_cant = [0; 5]; 
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_anterior(){
					match s.get_medio(){
						Medios_de_pago::Efectivo => metodos_cant[0] +=1,
						Medios_de_pago::Mercado_pago => metodos_cant[1] +=1,
						Medios_de_pago::Transferencia_bancaria => metodos_cant[2] +=1,
						Medios_de_pago::Tarjeta_de_credito => metodos_cant[3] +=1,
						Medios_de_pago::Criptomoneda => metodos_cant[4] +=1,
					}
				}
			});
			
			//Retornar segun posicion el tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(metodos_cant) {
				match pos {
					0 => res = Some(Medios_de_pago::Efectivo),
					1 => res = Some(Medios_de_pago::Mercado_pago),
					2 => res = Some(Medios_de_pago::Transferencia_bancaria),
					3 => res = Some(Medios_de_pago::Tarjeta_de_credito),
					4 => res = Some(Medios_de_pago::Criptomoneda),
					_ => res = None,
				}
			}

			return res;
		}
		return None
	}
	fn suscripcion_anterior_mas_contratada(&self)->Option<Suscripciones>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Suscripciones> = None;

			let mut tipos_cant = [0; 3]; 
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_anterior(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			});
			//Obtener el maximo del array
	
			//Retornar segun posicion del tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(tipos_cant) {
				match pos {
			    0 => res = Some(Suscripciones::Basic),
			    1 => res = Some(Suscripciones::Clasic),
			    2 => res = Some(Suscripciones::Super),
				_ => res = None,
				}
			}		

			return res;
		}
		return None
	}
}

#[cfg(test)]
mod test_ejercicio3{
	use core::panic;
	use super::*;

	#[test]
	fn operar_suscripcion_usuario(){
		let mut usuario1 = Usuario::new(&"Daniel".to_string() , 
		64254 , 
		Some(Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 123.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria)));
			
		assert_eq!(usuario1,Usuario::new(&"Daniel".to_string() , 64254 , Some(Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,123.5,5, 120325, Medios_de_pago::Transferencia_bancaria))) );
		
		assert!(usuario1.upgrade_suscripcion());

		if let Some(s) = usuario1.get_suscripcion_actual(){
			assert_eq!(s.get_tipo(),Suscripciones::Clasic);
			if let Some(s2) = usuario1.get_suscripcion_anterior(){
				assert_eq!(s2.get_tipo(),Suscripciones::Basic);
			}else{
				panic!("No se registro/actualizo la suscripcion anterior");
			}
		}else{
			panic!("No se registro/actualizo la suscripcion actual");
		}

		assert!(usuario1.downgrade_suscripcion());

		if let Some(s) = usuario1.get_suscripcion_actual(){
			assert_eq!(s.get_tipo(),Suscripciones::Basic);
			if let Some(s2) = usuario1.get_suscripcion_anterior(){
				assert_eq!(s2.get_tipo(),Suscripciones::Clasic);
			}else{
				panic!("No se registro/actualizo la suscripcion anterior");
			}
		}else{
			panic!("No se registro/actualizo la suscripcion actual");
		}

		//Resulta en false porque se llego al limite de "downgrade" para efectuar sobre el usuario
		assert!(usuario1.downgrade_suscripcion());
		assert!(!usuario1.downgrade_suscripcion());

		//Prueba de baja de suscripcion
	    usuario1.set_suscripcion_actual(&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,123.5,5,120325,Medios_de_pago::Transferencia_bancaria));
		assert!(usuario1.cancelar_suscripcion());
		assert!(usuario1.get_suscripcion_actual().is_none());
		assert!(!usuario1.cancelar_suscripcion());
	}

	#[test]
	fn operar_suscripciones_usuarios(){
		let mut usuario1 = Usuario::new(&"Daniel".to_string() , 
		64254 , 
		Some(Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 123.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria)));

		let mut usuario2 = Usuario::new(&"Tobias".to_string() , 
		64254 , 
		Some(Suscripcion_activa::crear_suscripcion(Suscripciones::Super,
			 243.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria)));

		let mut usuario3 = Usuario::new(&"Marcos".to_string() , 
		542134 , 
		Some(Suscripcion_activa::crear_suscripcion(Suscripciones::Super,
			 103.5,
			  5, 
			  120325, 
			  Medios_de_pago::Efectivo)));

		let mut usuario4 = Usuario::new(&"Dario".to_string() , 
	32124 , 
		Some(Suscripcion_activa::crear_suscripcion(Suscripciones::Clasic,
			 103.5,
			  5, 
			  120325, 
			  Medios_de_pago::Criptomoneda)));
	

		//Plataforma vacia

		let mut pl1 = Plataforma::new();

		assert!(pl1.metodo_pago_mas_usado().is_none());
		assert!(pl1.metodo_pago_anterior_mas_usado().is_none());
		assert!(pl1.suscripcion_mas_contratada().is_none());
		assert!(pl1.suscripcion_anterior_mas_contratada().is_none());

		//Plataforma con usuarios

		pl1.agregar(&usuario1);
		pl1.agregar(&usuario2);
		pl1.agregar(&usuario3);
		pl1.agregar(&usuario4);
		
		//Prueba estadistica
		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Transferencia_bancaria);
		}else{
			panic!("No hubo un retorno esperado");
		}
		
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Super);
		}else{
			panic!("No hubo un retorno esperado");
		}
		assert!(pl1.metodo_pago_anterior_mas_usado().is_none());
		assert!(pl1.suscripcion_anterior_mas_contratada().is_none());

		//Se hace "upgrade" a un usuario
		pl1.upgrade_usuario(&usuario1);
		assert!(pl1.upgrade_usuario(&usuario1));
		assert_eq!(pl1.upgrade_usuario(&usuario2),false);

		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Transferencia_bancaria);
		}else{
			panic!("No hubo un retorno esperado");
		}

		if let Some(tipo) = pl1.metodo_pago_anterior_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Transferencia_bancaria);
		}else{
			panic!("No hubo un retorno esperado");
		}
		
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Super);
		}else{
			panic!("No hubo un retorno esperado");
		}

		if let Some(tipo) = pl1.suscripcion_anterior_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Clasic);
		}else{
			panic!("No hubo un retorno esperado");
		}

		//Se hace "downgrade" a un usuario
		assert!(pl1.downgrade_usuario(&usuario1));
		assert!(pl1.downgrade_usuario(&usuario1));
		assert!(pl1.downgrade_usuario(&usuario1));
		assert!(!pl1.downgrade_usuario(&usuario1));

		//Se cancela una suscripcion a un usuario
		assert!(pl1.cancelar_suscripcion(&usuario2));
		//Se borra un usuario y se hacen pruebas de existencia
		assert!(pl1.eliminar(&usuario2));
		assert!(!pl1.cancelar_suscripcion(&usuario2));
		assert!(!pl1.upgrade_usuario(&usuario2)); 
		assert!(!pl1.downgrade_usuario(&usuario2)); 
		
	}

	//Test para evaluar los diferentes resultados que proporcione la plataforma (maximos)
	#[test]
	fn evaluar_estadisticas(){
		//Usuarios
		let mut usuario1 = Usuario::new(&"Dario".to_string() , 32124 , None );
		let mut usuario2 = Usuario::new(&"Mario".to_string() , 367665 , None );
		
		let s = Suscripcion_activa::crear_suscripcion(Suscripciones::Super,100.0,5,120325,Medios_de_pago::Mercado_pago);
		usuario1.set_suscripcion_actual(&s);

		//Plataforma
		let mut pl1 = Plataforma::new();
		pl1.agregar(&usuario1);
		
		//Suscripcion max
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Super);
		}else{
			panic!("No hubo un retorno esperado");
		}
		assert!(pl1.suscripcion_anterior_mas_contratada().is_none());

		pl1.downgrade_usuario(&usuario1);
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Clasic);
		}else{
			panic!("No hubo un retorno esperado");
		}

		if let Some(tipo) = pl1.suscripcion_anterior_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Super);
		}else{
			panic!("No hubo un retorno esperado");
		}

		pl1.downgrade_usuario(&usuario1);
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Basic);
		}else{
			panic!("No hubo un retorno esperado");
		}

		if let Some(tipo) = pl1.suscripcion_anterior_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Clasic);
		}else{
			panic!("No hubo un retorno esperado");
		}

		//Metodo pago max
		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Mercado_pago);
		}else{
			panic!("No hubo un retorno esperado");
		}

		pl1.downgrade_usuario(&usuario1);
		if let Some(tipo) = pl1.suscripcion_anterior_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Basic);
		}else{
			panic!("No hubo un retorno esperado");
		}
		assert!(pl1.suscripcion_mas_contratada().is_none());
		
		//Metodo pago max (Casos con downgrade y con inseciones nuevas de usuario)
		if let Some(tipo) = pl1.metodo_pago_anterior_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Mercado_pago);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.eliminar(&usuario1);

		//Nuevo metodo de pago
		let s = Suscripcion_activa::crear_suscripcion(Suscripciones::Super,100.0,5,120325,Medios_de_pago::Tarjeta_de_credito);
		usuario2.set_suscripcion_actual(&s);
		pl1.agregar(&usuario2);

		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Tarjeta_de_credito);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.downgrade_usuario(&usuario2);

		if let Some(tipo) = pl1.metodo_pago_anterior_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Tarjeta_de_credito);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.eliminar(&usuario2);

		//Nuevo metodo de pago
		let s = Suscripcion_activa::crear_suscripcion(Suscripciones::Super,100.0,5,120325,Medios_de_pago::Efectivo);
		usuario2.set_suscripcion_actual(&s);
		pl1.agregar(&usuario2);

		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Efectivo);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.downgrade_usuario(&usuario2);

		if let Some(tipo) = pl1.metodo_pago_anterior_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Efectivo);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.eliminar(&usuario2);

		//Nuevo metodo de pago
		let s = Suscripcion_activa::crear_suscripcion(Suscripciones::Super,100.0,5,120325,Medios_de_pago::Criptomoneda);
		usuario2.set_suscripcion_actual(&s);
		pl1.agregar(&usuario2);

		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Criptomoneda);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.downgrade_usuario(&usuario2);

		if let Some(tipo) = pl1.metodo_pago_anterior_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Criptomoneda);
		}else{
			panic!("No hubo un retorno esperado");
		}
		pl1.eliminar(&usuario2);

	}

	
}