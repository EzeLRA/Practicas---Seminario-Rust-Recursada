/*
    IMPLEMENTACION DE EJERCICIO 6 - TP5
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
    Denegado(String),
    SinVerificacion,
    FalloTransaccion
}

impl Display for error_operatoria{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            error_operatoria::Inexistente(val) => write!(f, "No se encontro el elemento en la estructura {} ",val),
            error_operatoria::EstructuraVacia(val) => write!(f, "La estrucutra {} no dispone de elementos ",val),
            error_operatoria::Denegado(val) => write!(f,"{}",val),
            error_operatoria::SinVerificacion => write!(f,"Wallet sin permiso"),
            error_operatoria::FalloTransaccion => write!(f,"Fallo en el proceso de transaccion")
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
    Extraccion Ejercicio 5 - TP4
*/

use std::collections::HashMap;

//Se debe importar (rand = "0.8") en cargo.toml
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

//Funcion auxiliar para generar un "hash simulado"
pub fn aleatorio(tam: usize) -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(tam)
        .map(char::from)
        .collect()
}

/*
    Estructuras base para el sistema
*/

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct Blockchain{
    nombre : String,
    prefijo : String
}

impl Blockchain{
    fn new(nom:&str,pre:&str)->Blockchain{
        return Blockchain { nombre: nom.to_string(), prefijo: pre.to_string() }
    }
    fn es_igual_a(&self,nom:&String)->bool{
        return &self.nombre == nom;
    }
    fn generar_hash(&self,tam:usize)->String{
        return format!("{}{}",self.get_nombre() , aleatorio(tam) )
    }
    fn get_nombre(&self)->&String{
        return &self.nombre;
    }
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct Criptomoneda{
    nombre : String,
    prefijo : String,
    blockchains : Vec<Blockchain>
}

impl Criptomoneda{
    fn new(nom:&str,pre:&str)->Criptomoneda{
        return Criptomoneda { nombre: nom.to_string(), prefijo: pre.to_string(), blockchains: Vec::new()}
    }
    fn agregar_blockchain(&mut self,b:Blockchain)->bool{
        let mut pude = false;
        
        if !self.blockchains.iter().any(|blockchain| blockchain.es_igual_a(b.get_nombre())) {
            self.blockchains.push(b);
            pude = true;
        }

        return pude;
    }
    fn eliminar_blockchain(&mut self,b:&Blockchain)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.blockchains.iter().position(|blockchain| blockchain.es_igual_a(b.get_nombre())){
            self.blockchains.remove(pos);
            pude = true;
        }

        return pude;
    }
    fn blockchain_encontrado(&self,nom:&String)->bool{
        return self.blockchains.iter().any(|blockchain| blockchain.es_igual_a(nom));
    }
    fn get_blockchain(&self,nom:&String)->Option<&Blockchain>{
        self.blockchains.iter().find(|blockchain| blockchain.es_igual_a(nom))
    }
    fn get_nombre(&self)->&String{
        return &self.nombre;
    }
    fn get_prefijo(&self)->&String{
        return &self.prefijo;
    }
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct CriptomonedaDispone(String,f64);

impl CriptomonedaDispone{
    //Sin limite de ingreso maximo
    fn contabilizar(&mut self,monto:f64){
        self.1 += monto;
    }
    //Con limite de extracion(Sin saldo negativo)
    fn descontabilizar(&mut self,monto:f64)->bool{
        let mut pude = false;
        if self.1 >= monto {
            self.1 -= monto;
            pude = true;
        }
        return pude;
    }
    fn es_igual_a(&self,nom:&String)->bool{
        return &self.0 == nom;
    }
    fn get_monto(&self)->f64{
        return self.1;
    }
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct BalancePropio{
    criptomonedas : Vec<CriptomonedaDispone>,
    dinero_fiat : f64
}

impl BalancePropio{
    fn new()->BalancePropio{
        return BalancePropio { criptomonedas: Vec::new() , dinero_fiat: 0.0 }
    }
    fn fijar_fiat(&mut self,monto:f64){
        self.dinero_fiat = monto;
    }
    //Sin limite de ingreso maximo
    fn contabilizar_fiat(&mut self,monto:f64){
        self.fijar_fiat(self.dinero_fiat + monto);
    }
    //Con limite de extracion(Sin saldo negativo)
    fn descontabilizar_fiat(&mut self,monto:f64)->bool{
        let mut pude = false;
        if self.dinero_fiat >= monto {
            self.fijar_fiat(self.dinero_fiat - monto);
            pude = true;
        }
        return pude;
    }
    fn agregar_criptomoneda(&mut self,nom:String,monto:f64){
        self.criptomonedas.push(CriptomonedaDispone(nom,monto));
    }
    fn contabilizar_criptomoneda(&mut self,nom:&String,monto:f64)->bool{
        let mut pude = false;
        
        if let Some(dato) = self.criptomonedas.iter_mut().find(|cripto| cripto.es_igual_a(nom)){
            dato.contabilizar(monto);
            pude = true;
        }

        return pude;
    }
    fn descontabilizar_criptomoneda(&mut self,nom:&String,monto:f64)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.criptomonedas.iter().position(|cripto| cripto.es_igual_a(nom)){
            pude = self.criptomonedas[pos].descontabilizar(monto);
            if (pude)&&(!(self.criptomonedas[pos].get_monto() > 0.0)) {
                self.criptomonedas.remove(pos);
            }
        }

        return pude;
    }
    fn get_cant_fiat(&self)->f64{
        return self.dinero_fiat;
    }
    fn tiene_criptomonedas(&self)->bool{
        return !self.criptomonedas.is_empty();
    }
}



/*
    Estructura abstracta datos personales y usuario no abstracta
*/

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct DatosPersona{
    nombre : String,
    apellido : String,
    email : String,
    dni : u64
}

pub trait InformacionPersonal{
    fn informacion_correcta(&self, info:&DatosPersona)->bool;
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
struct Usuario{
    datos : DatosPersona,
    validado : bool,
    balance : BalancePropio
}

impl InformacionPersonal for Usuario{
    fn informacion_correcta(&self, info:&DatosPersona)->bool{
        return &self.datos == info;
    }
}

impl Usuario{
    fn new(nom:&str,ape:&str,mail:&str,dni_in:u64)->Usuario{
        return Usuario { 
            datos: DatosPersona { 
                nombre: nom.to_string(), 
                apellido: ape.to_string(), 
                email: mail.to_string(), 
                dni: dni_in
            } , 
            validado: false, 
            balance: BalancePropio::new()}
    }
    fn is_verificado(&self)->bool{
        return self.validado;
    }
    fn cambiar_verificacion(&mut self){
        self.validado = !self.validado;
    }
    //Fiat moneda
    fn ingresar_monto_fiat(&mut self,monto:f64){
        self.balance.contabilizar_fiat(monto);
    }
    fn retirar_monto_fiat(&mut self,monto:f64)->bool{
        return self.balance.descontabilizar_fiat(monto);
    }
    fn dispone_criptomonedas(&self)->bool{
        return self.balance.tiene_criptomonedas();
    }
    fn get_balance_fiat(&self)->f64{
        return self.balance.get_cant_fiat();
    }
    //Criptomoneda(Los montos se basan en lo que le proporcione el sistema)
    fn comprar_criptomoneda(&mut self,nom:&String,monto_fiat_compra:f64,monto_a_comprar:f64)->bool{
        let mut pude = false;
        
        pude = self.retirar_monto_fiat(monto_fiat_compra);
        if pude {
            if !self.balance.contabilizar_criptomoneda(nom,monto_a_comprar){
                self.balance.agregar_criptomoneda(nom.clone(),monto_a_comprar);
            }
        }

        return pude;
    }
    fn vender_criptomoneda(&mut self,nom:&String,criptos_a_vender:f64,ganancia_venta:f64)->bool{
        let mut pude = false;
        
        pude = self.balance.descontabilizar_criptomoneda(nom,criptos_a_vender);            
        if pude {
            self.balance.contabilizar_fiat(ganancia_venta);
        }

        return pude;
    }
    //Transacciones a blockchains
    fn criptomoneda_a_blockchain(&mut self,nomCripto:&String,monto:f64,nomBlockchain:&String,cripto : &Criptomoneda)->bool{
        let mut pude = false;
        if cripto.blockchain_encontrado(nomBlockchain){
            pude = self.balance.descontabilizar_criptomoneda(nomCripto,monto);
        }
        return pude;
    }
    fn blockchain_a_criptomoneda(&mut self,nomCripto:&String,monto:f64,nomBlockchain:&String,cripto : &Criptomoneda)->bool{
        let mut pude = cripto.blockchain_encontrado(nomBlockchain);
        if pude {
            pude = self.balance.contabilizar_criptomoneda(nomCripto,monto);
        }
        return pude;
    }
}

/* 
    Extraccion TP3 - EJ3 - Fecha
*/
//Atributos
#[derive(Debug,Clone,Serialize,Deserialize)]
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
        return (self.anio % 4 == 0 && self.anio % 100 != 0) || (self.anio % 400 == 0)
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
		f = Fecha::new(32, 2, 2005);
        assert_eq!(f.es_fecha_valida(),false);
    }

    #[test]
    fn validar_bisiesto(){
        let mut f = Fecha::new(1, 1, 2028);
        assert_eq!(f.es_bisiesto(),true);
        f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_bisiesto(),false);
		f = Fecha::new(1, 1, 100);
        assert_eq!(f.es_bisiesto(),false);
		f = Fecha::new(1, 1, 400);
		assert_eq!(f.es_bisiesto(),true);
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
    Estructuras pertenciente al sistema
*/

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Datos_Ingreso{
    datos_usuario : DatosPersona,
    fecha: Fecha,
    monto : f64
}

impl Datos_Ingreso{
    fn new(datos : DatosPersona, f : Fecha , m : f64)->Datos_Ingreso{
        return Datos_Ingreso { datos_usuario: datos, fecha: f, monto: m };
    }
    fn get_fecha(&self)->&Fecha{
        return &self.fecha;
    }
    fn get_monto(&self)->f64{
        return self.monto;
    }
}

impl InformacionPersonal for Datos_Ingreso{
    fn informacion_correcta(&self, info:&DatosPersona)->bool{
        return &self.datos_usuario == info;
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Datos_Retiro{
    datos_genericos : Datos_Ingreso,
    medio_pago : MediosPago
}

impl Datos_Retiro{
    fn new(d:DatosPersona,fe:Fecha,monto:f64,medio:MediosPago)->Datos_Retiro{
        return Datos_Retiro { datos_genericos: Datos_Ingreso::new(d, fe , monto), medio_pago: medio }
    }

    fn get_medio_pago(&self)->&MediosPago{
        return &self.medio_pago;
    }
}

//Tipos de transacciones implementados
#[derive(Debug,Clone,Serialize,Deserialize)]
struct Datos_Operacion_Criptomoneda{
    datos_genericos : Datos_Ingreso,
    criptomoneda : Criptomoneda,
    cotizacion : f64
}

impl Datos_Operacion_Criptomoneda{
    fn new(user:DatosPersona,f:Fecha,m:f64,c:Criptomoneda,cotiz:f64)->Datos_Operacion_Criptomoneda{
        return Datos_Operacion_Criptomoneda { 
            datos_genericos: Datos_Ingreso::new(user, f, m ),
            criptomoneda: c,
            cotizacion: cotiz };
    }
    fn get_monto_operacion(&self)->f64{
        return self.datos_genericos.get_monto();
    }
    fn get_cripto_nom(&self)->&String{
        return self.criptomoneda.get_nombre();
    }
    fn get_criptomoneda(&self)->&Criptomoneda{
        return &self.criptomoneda;
    }
    fn get_cotizacion(&self)->f64{
        return self.cotizacion;
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Datos_Retiro_Blockchain{
    datos_criptomoneda : Datos_Operacion_Criptomoneda,
    blockchain : Blockchain,
    hash : String
}

impl Datos_Retiro_Blockchain{
    fn new(user:DatosPersona,f:Fecha,m:f64,c:Criptomoneda,cotiz:f64,b:Blockchain)->Datos_Retiro_Blockchain{
        return Datos_Retiro_Blockchain { 
            datos_criptomoneda: Datos_Operacion_Criptomoneda::new(user, f, m, c, cotiz),
            hash: b.generar_hash(10),
            blockchain: b
        }
    }
    fn get_blockchain(&self)->&Blockchain{
        return &self.blockchain;
    }
    fn get_hash(&self)->&String{
        return &self.hash;
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Datos_Extraccion_Blockchain{
    datos_criptomoneda : Datos_Operacion_Criptomoneda,
    blockchain : Blockchain
}

impl Datos_Extraccion_Blockchain{
    fn new(user:DatosPersona,f:Fecha,m:f64,c:Criptomoneda,cotiz:f64,b:Blockchain)->Datos_Extraccion_Blockchain{
        return Datos_Extraccion_Blockchain { 
            datos_criptomoneda: Datos_Operacion_Criptomoneda::new(user, f, m, c, cotiz),
            blockchain: b
        }
    }
    fn get_blockchain(&self)->Blockchain{
        return self.blockchain.clone();
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)] 
enum TiposTransacciones{
    IngresoFiat(Datos_Ingreso),
    CompraCriptomoneda(Datos_Operacion_Criptomoneda),
    VentaCriptomoneda(Datos_Operacion_Criptomoneda),
    RetiroCriptomoneda(Datos_Retiro_Blockchain),
    RecepcionCriptomoneda(Datos_Extraccion_Blockchain),
    RetiroFiat(Datos_Retiro),
}

impl TiposTransacciones{
    pub fn es_tipo_compra(&self)->bool{
        return matches!(self, TiposTransacciones::CompraCriptomoneda(_));
    }
    pub fn es_tipo_venta(&self)->bool{
        return matches!(self, TiposTransacciones::VentaCriptomoneda(_));
    }
    pub fn obtener_nombre_criptomoneda(&self)->Option<&String>{
        let mut res : Option<&String> = None;

        //Solamente se procesan los tipos compra y venta de criptomonedas , los demas se los excluye para la resolucion principal
        res = match self{
            TiposTransacciones::CompraCriptomoneda(datos) => Some(datos.get_cripto_nom()),
            TiposTransacciones::VentaCriptomoneda(datos) => Some(datos.get_cripto_nom()),
            _ => None, 
        };

        return res;
    }
    pub fn obtener_volumen_criptomoneda(&self)->Option<f64>{
        let mut res : Option<f64> = None;

        //Solamente se procesan los tipos compra y venta de criptomonedas , los demas se los excluye para la resolucion principal
        res = match self{
            TiposTransacciones::CompraCriptomoneda(datos) => Some(datos.get_monto_operacion() * datos.get_cotizacion()),
            TiposTransacciones::VentaCriptomoneda(datos) => Some(datos.get_monto_operacion() * datos.get_cotizacion()),
            _ => None,
        };

        return res;
    }
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
enum MediosPago{
    MercadoPago,
    TransferenciaBancaria
}

#[derive(PartialEq,Debug,Clone)]
//Valor(Unidad) = Se ingresa el monto equivalente en fiat
struct Criptomoneda_disponible(Criptomoneda,f64);
impl Criptomoneda_disponible{
    fn get_cotiza(&self)->f64{
        return self.1;
    }
    
}



/*
    Estructura principal : Sistema
*/


#[derive(Debug,Clone)]
struct Plataforma{
    usuarios : Vec<Usuario>,
    criptomonedas_dispone : Vec<Criptomoneda_disponible>, //Datos de la criptomoneda y cotiza
    registro_transacciones : Vec<TiposTransacciones>,
    path_balances : String,
    path_transacciones : String

}

impl Plataforma{
    /*
		Nueva implementacion - TP5
	*/
	fn recuperar_balances_info(path:&str)-> Result<Vec<Usuario>,Errores>{
		let file = File::open(path).map_err(Errores::ErrorIO)?;
		let balances: Vec<Usuario> = serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
		Ok(balances)
	}
	fn guardar_balances_info(&self) -> Result<(), Errores> {
	    let mut file = File::create(&self.path_balances)?;
	    let serialized = serde_json::to_string(&self.usuarios)?;
        file.write_all(serialized.as_bytes())?;
		return Ok(())
    }
    fn recuperar_transacciones_info(path:&str)-> Result<Vec<TiposTransacciones>,Errores>{
		let file = File::open(path).map_err(Errores::ErrorIO)?;
		let transacciones : Vec<TiposTransacciones> = serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
		Ok(transacciones)
	}
	fn guardar_transacciones_info(&self) -> Result<(), Errores> {
	    let mut file = File::create(&self.path_transacciones)?;
	    let serialized = serde_json::to_string(&self.registro_transacciones)?;
        file.write_all(serialized.as_bytes())?;
		return Ok(())
    }
    //Funciones secundarias
    fn new(path_b:&str,path_t:&str)->Plataforma{
        let wallets = match Plataforma::recuperar_balances_info(path_b){
            Ok(dato) => dato,
            Err(_) => Vec::new()
        };
        let comprobantes = match Plataforma::recuperar_transacciones_info(path_t){
            Ok(dato) => dato,
            Err(_) => Vec::new()
        };
        return Plataforma{
            usuarios:wallets,
            criptomonedas_dispone:Vec::new(),
            registro_transacciones:comprobantes,
            path_balances : path_b.to_string(),
            path_transacciones : path_t.to_string()
        }
    }
    fn eliminar_criptomoneda(&mut self,cripto:&Criptomoneda)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.criptomonedas_dispone.iter().position(|c| c.0.get_nombre() == cripto.get_nombre()){
            pude = true;
            self.criptomonedas_dispone.remove(pos);
        }       

        return pude;
    }
    fn registrar_criptomoneda(&mut self,cripto:Criptomoneda,cotiza:f64)->bool{
        let mut pude = false;
        
        if self.criptomonedas_dispone.iter().find(|&c| c.0.get_nombre() == cripto.get_nombre() ).is_none(){
            self.criptomonedas_dispone.push(Criptomoneda_disponible(cripto,cotiza));
            pude = true;
        }        

        return pude;
    }
    fn obtener_cotizacion_criptomoneda(&self,nom:&String)->f64{
        let mut total = 0.0;
        if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == &nom){
            total = cr.get_cotiza();
        }
        return total;
    }
    fn registrar_transaccion(&mut self,t : TiposTransacciones)->Result<(),Errores>{
        self.registro_transacciones.push(t);
        self.guardar_transacciones_info()?;
        return Ok(())
    }
    fn registrar_usuario(&mut self,u1:Usuario)->Result<(),Errores>{
        
        if !self.usuarios.iter().any(|user| user.informacion_correcta(&u1.datos)){
            self.usuarios.push(u1);
            self.guardar_balances_info()?;
            return Ok(())
        }        

        return Err(Errores::ErrorOperatoria(error_operatoria::Denegado("Wallet ya creada".to_string())));
    }
    fn retornar_fiat_usuario(&mut self,u1:&Usuario)->f64{
        let mut res = 0.0;
        
        if let Some(u) = self.usuarios.iter().find(|user| user.informacion_correcta(&u1.datos)){
            res = u.get_balance_fiat();
        }        

        return res;
    }
    fn validar_usuario(&mut self,u1:&Usuario)->Result<(),Errores>{
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if !u.is_verificado(){
                u.cambiar_verificacion();
                self.guardar_balances_info()?;
                return Ok(())
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Denegado("Wallet ya validada".to_string())));
        }       
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())))
    }
    //Funciones primarias
    fn ingresar_monto_usuario(&mut self,u1:&Usuario,f:Fecha,m:f64)->Result<(),Errores>{
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            u.ingresar_monto_fiat(m);
            let datos = Datos_Ingreso::new(u.datos.clone(),f, m);
            self.registrar_transaccion(TiposTransacciones::IngresoFiat(datos))?;
            self.guardar_balances_info()?;
            self.guardar_transacciones_info()?;
            return Ok(())
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())) );
    }
    //Se ingresa la cantidad que se quiera invertir en cripto
    fn comprar_criptomoneda_usuario(&mut self,u1:&Usuario,f:Fecha,monto_fiat:f64,nom:&String)->Result<(),Errores>{
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == &nom){
                //Compra
                if u.is_verificado(){
                    let monto_compra = monto_fiat/cr.get_cotiza();

                    if u.comprar_criptomoneda(nom,monto_fiat,monto_compra) {
                        //Generacion de comprobante
                        let datos = Datos_Operacion_Criptomoneda::new(u.datos.clone(),f,monto_compra,cr.0.clone(),cr.get_cotiza());
                        self.registrar_transaccion(TiposTransacciones::CompraCriptomoneda(datos))?;
                        self.guardar_balances_info()?;
                        return Ok(())
                    }
                    
                    return Err(Errores::ErrorOperatoria(error_operatoria::Denegado("Saldo insuficiente".to_string())))
                }
                return Err(Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Criptomonedas".to_string())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())) );
    }
    //Se ingresa la cantidad de cripto que se desea
    fn vender_criptomoneda_usuario(&mut self,u1:&Usuario,f:Fecha,criptos_vender:f64,nom:&String)->Result<(),Errores>{
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == &nom){
                //Venta
                if u.is_verificado(){
                    let ganancia = criptos_vender * cr.get_cotiza();

                    if u.vender_criptomoneda(nom,criptos_vender,ganancia) {
                        //Generacion de comprobante
                        let datos = Datos_Operacion_Criptomoneda::new(u.datos.clone(),f,criptos_vender,cr.0.clone(),cr.get_cotiza());
                        self.registrar_transaccion(TiposTransacciones::VentaCriptomoneda(datos))?;
                        self.guardar_balances_info()?;
                        return Ok(())
                    }
                    return Err(Errores::ErrorOperatoria(error_operatoria::Denegado("Saldo insuficiente de criptomonedas".to_string())))
                }
                return Err(Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Criptomonedas".to_string())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())));
    }
    fn criptomoneda_a_blockchain_usuario(&mut self,u1:&Usuario,f:Fecha,montoTransaccion:f64,nomCripto:&String,nomBlockchain:&String)->Result<(),Errores>{
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == &nomCripto){
                //Buscar blockchain en la criptomoneda
                if u.is_verificado(){
                    //Generar comprobante
                    if u.criptomoneda_a_blockchain(nomCripto,montoTransaccion,nomBlockchain,&cr.0) {
                        if let Some(b) = cr.0.get_blockchain(nomBlockchain){
                            let datos = Datos_Retiro_Blockchain::new(u.datos.clone(),f,montoTransaccion,cr.0.clone(),cr.get_cotiza(),b.clone());
                            self.registrar_transaccion(TiposTransacciones::RetiroCriptomoneda(datos))?;
                            self.guardar_balances_info()?;
                            return Ok(())
                        }
                    }
                    return Err(Errores::ErrorOperatoria(error_operatoria::FalloTransaccion))
                }
                return Err(Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Criptomonedas".to_string())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())));
    }
    fn blockchain_a_criptomoneda_usuario(&mut self,u1:&Usuario,f:Fecha,montoTransaccion:f64,nomCripto:&String,nomBlockchain:&String)->Result<(),Errores>{
        let mut pude = false;
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == &nomCripto){
                //Hacer transaccion y generar comprobante
                if u.blockchain_a_criptomoneda(nomCripto,montoTransaccion,nomBlockchain,&cr.0) {
                    if let Some(b) = cr.0.get_blockchain(nomBlockchain){
                        let datos = Datos_Extraccion_Blockchain::new(u.datos.clone(),f,montoTransaccion,cr.0.clone(),cr.get_cotiza(),b.clone());
                        self.registrar_transaccion(TiposTransacciones::RecepcionCriptomoneda(datos))?;
                        self.guardar_balances_info()?;
                        return Ok(())
                    }
                }
                return Err(Errores::ErrorOperatoria(error_operatoria::FalloTransaccion))
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Criptomonedas".to_string())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())))
    }
    fn retirar_monto_usuario(&mut self,u1:&Usuario,f:Fecha,m:f64,med:MediosPago)->Result<(),Errores>{
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if u.is_verificado(){
                if u.retirar_monto_fiat(m){
                    let datos = Datos_Retiro::new(u.datos.clone(), f, m,med);
                    self.registrar_transaccion(TiposTransacciones::RetiroFiat(datos))?;
                    self.guardar_balances_info()?;
                    return Ok(())
                }
                return Err(Errores::ErrorOperatoria(error_operatoria::Denegado("Saldo insuficiente".to_string())))
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Wallets".to_string())));
    }

    /*
        Funciones estadistica para comprobantes generados por el sistema
    */
    fn criptomoneda_max_cant_compras(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : HashMap<String,u32> = HashMap::new();

            self.registro_transacciones.iter().filter(|c| c.es_tipo_compra())
            .for_each(|comprobante|{
                if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                    *contador.entry(nombre.clone()).or_insert(0) += 1;
            
                }
            });
            
            res = contador.into_iter().max_by_key(|&(_,cant)| cant).map(|(nom,_)|nom);
        }

        return res;
    }

    fn criptomoneda_max_cant_ventas(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : HashMap<String,u32> = HashMap::new();

            self.registro_transacciones.iter().filter(|c| c.es_tipo_venta())
            .for_each(|comprobante|{
                if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                    *contador.entry(nombre.clone()).or_insert(0) += 1;
            
                }
            });
            
            res = contador.into_iter().max_by_key(|&(_,cant)| cant).map(|(nom,_)|nom);

        }

        return res;
    }

    fn criptomoneda_max_monto_compras(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : HashMap<String,f64> = HashMap::new();

            self.registro_transacciones.iter().filter(|c|c.es_tipo_compra())
            .for_each(|comprobante|{
                if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                    if let Some(monto) = comprobante.obtener_volumen_criptomoneda(){
                        *contador.entry(nombre.clone()).or_insert(0.0) += monto;
                    }        
                }
            });
            
            res = contador.into_iter().fold(None, |max_actual: Option<(String, f64)>, (nom, monto)| {
                match max_actual {
                    Some((max_nom, max_monto)) => {
                        if monto > max_monto {
                            Some((nom, monto))
                        } else {
                            Some((max_nom, max_monto))
                        }
                    }
                    None => Some((nom, monto)),
                }
            }).map(|(nom, _)| nom);

        }

        return res;
    }
    fn criptomoneda_max_monto_ventas(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : HashMap<String,f64> = HashMap::new();

            self.registro_transacciones.iter().filter(|c|c.es_tipo_venta())
            .for_each(|comprobante|{
                if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                    if let Some(monto) = comprobante.obtener_volumen_criptomoneda(){
                        *contador.entry(nombre.clone()).or_insert(0.0) += monto;
                    }        
                }
            });
            
            res = contador.into_iter().fold(None, |max_actual: Option<(String, f64)>, (nom, monto)| {
                match max_actual {
                    Some((max_nom, max_monto)) => {
                        if monto > max_monto {
                            Some((nom, monto))
                        } else {
                            Some((max_nom, max_monto))
                        }
                    }
                    None => Some((nom, monto)),
                }
            }).map(|(nom, _)| nom);        
        }

        return res;
    }
}






/*
 *
        Seccion de testing
 *
*/

#[cfg(test)]
mod test_ejercicio5{    
    use super::*;

    #[test]
    fn prueba_blockchain(){
        let nom = "Block81".to_string();
        let b = Blockchain::new(&nom,&"BLO".to_string());
        assert_eq!(b,Blockchain::new(&nom,&"BLO".to_string()));
        assert!(b.es_igual_a(&nom));
        assert!(!b.generar_hash(5).is_empty());
    }

    #[test]
    fn prueba_criptomoneda(){
        let c = Criptomoneda::new(&"Cripton1".to_string(),&"CRP".to_string());
        assert_eq!(c,Criptomoneda::new(&"Cripton1".to_string(),&"CRP".to_string()));
        assert!(!c.get_prefijo().is_empty());
        assert!(c.blockchains.is_empty());
    }

    #[test]
    fn conexiones_cripto_blockchain(){
        //Criptomoneda
        let mut c = Criptomoneda::new(&"Cripton2".to_string(),&"CRP2".to_string());
        //Blockchains
        let b1 = Blockchain::new(&"Block81".to_string(),&"BLO81".to_string());
        let b2 = Blockchain::new(&"Block87".to_string(),&"BLO87".to_string());
        let b3 = Blockchain::new(&"Block19".to_string(),&"BLO19".to_string());

        //Se le agregan las conexiones con blockchains
        assert!(c.agregar_blockchain(b1.clone()));
        assert!(c.agregar_blockchain(b2.clone()));
        assert!(c.agregar_blockchain(b3.clone()));
        assert!(!c.agregar_blockchain(b1.clone()));    //Ya existe un vinculo por lo que no existe una revinculacion
        assert!(!c.blockchains.is_empty());

        //Busqueda de una blockchain
        let nom = "Block81".to_string();
        assert!(c.blockchain_encontrado(&nom));
        assert!(c.get_blockchain(&nom).is_some());
        let nom = "Block31".to_string();
        assert!(c.get_blockchain(&nom).is_none());
        
        //Desvinculacion y busqueda de blockchain
        let nom = "Block81".to_string();
        assert!(c.eliminar_blockchain(&b1));
        assert!(!c.eliminar_blockchain(&b1));   //No se hace una desvinculacion para un blockchain inexistente
        assert!(!c.blockchain_encontrado(&nom));
        assert!(c.get_blockchain(&nom).is_none());

        //Desvinculacion total 
        assert!(c.eliminar_blockchain(&b2));
        assert!(c.eliminar_blockchain(&b3));
        assert!(c.blockchains.is_empty());
    }

    #[test]
    fn prueba_datos_personales_usuario(){
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Informacion correcta
        assert!(us1.informacion_correcta(&datos));

        assert!(!us1.is_verificado());
        us1.cambiar_verificacion();
        assert!(us1.is_verificado());
    }

    //Prueba de operatoria sin uso del sistema
    #[test]
    fn operatoria_balance_usuario(){
        //Usuario
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Monto vacio de dinero fiat
        assert_eq!(us1.get_balance_fiat(),0.0);

        //Ingreso dinero fiat
        us1.ingresar_monto_fiat(10000.0);
        assert_eq!(us1.get_balance_fiat(),10000.0);

        //Retiro de monto
        assert!(!us1.retirar_monto_fiat(20000.0));
        assert!(us1.retirar_monto_fiat(10000.0));
        assert_eq!(us1.get_balance_fiat(),0.0);  



        //Simulacion de compra de criptomonedas
        assert!(!us1.is_verificado());
        us1.cambiar_verificacion();
        assert!(us1.is_verificado());
        us1.ingresar_monto_fiat(20000.0);

        //Criptomonedas
        let c1 = Criptomoneda::new(&"Cripton".to_string(),&"CRP".to_string());
        let c2 = Criptomoneda::new(&"MineCoin".to_string(),&"MIN".to_string());
        let c3 = Criptomoneda::new(&"Bitcoin".to_string(),&"BTC".to_string());

        assert!(us1.comprar_criptomoneda(c1.get_nombre(),5000.0,10.0));
        assert!(us1.comprar_criptomoneda(c2.get_nombre(),5000.0,40.0));
        assert!(!us1.comprar_criptomoneda(c3.get_nombre(),25000.0,10.0));
        assert!(us1.comprar_criptomoneda(c3.get_nombre(),10000.0,10.0));
        assert_eq!(us1.get_balance_fiat(),0.0); 

        //Simulacion de venta de criptomonedas
        assert!(us1.vender_criptomoneda(c3.get_nombre(),5.0,20000.0));
        assert!(!us1.vender_criptomoneda(&"Robux".to_string(),10.0,10000.0));
        assert!(us1.vender_criptomoneda(c1.get_nombre(),10.0,8000.0));
        assert!(!us1.vender_criptomoneda(c1.get_nombre(),10.0,8000.0));
        assert!(us1.get_balance_fiat()>0.0);
    }

    //Prueba de transacciones sin el uso del sistema
    #[test]
    fn transacciones_blockchain_usuario(){
        //Usuario
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Criptomoneda
        let mut c = Criptomoneda::new(&"Cripton2".to_string(),&"CRP2".to_string());
        //Blockchains
        let b1 = Blockchain::new(&"Block81".to_string(),&"BLO81".to_string());
        let b2 = Blockchain::new(&"Block87".to_string(),&"BLO87".to_string());
        let b3 = Blockchain::new(&"Block19".to_string(),&"BLO19".to_string());

        c.agregar_blockchain(b1.clone());
        c.agregar_blockchain(b2.clone());
        c.agregar_blockchain(b3.clone());

        //Ingreso de monto fiat
        us1.ingresar_monto_fiat(10000.0);
        //Compra de criptomoneda
        us1.cambiar_verificacion();
        assert!(!us1.dispone_criptomonedas());
        assert!(us1.comprar_criptomoneda(c.get_nombre(),10000.0,100.0));

        //Transaccion
        assert!(us1.criptomoneda_a_blockchain(c.get_nombre(),50.0,b1.get_nombre(),&c));
        assert!(!us1.criptomoneda_a_blockchain(&"BitCoin".to_string(),50.0,b1.get_nombre(),&c));
        assert!(!us1.criptomoneda_a_blockchain(c.get_nombre(),50.0,&"Block1".to_string(),&c));

        assert!(us1.blockchain_a_criptomoneda(c.get_nombre(),50.0,b1.get_nombre(),&c));
        assert!(!us1.blockchain_a_criptomoneda(&"BitCoin".to_string(),50.0,b1.get_nombre(),&c));
        assert!(!us1.blockchain_a_criptomoneda(c.get_nombre(),50.0,&"Block1".to_string(),&c));
    }

    //Operatoria basica del sistema
    #[test]
    fn operatoria_principal_sistema(){
        //Usuario
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Criptomonedas
        let c1 = Criptomoneda::new(&"Cripton".to_string(),&"CRP".to_string());
        let c2 = Criptomoneda::new(&"MineCoin".to_string(),&"MIN".to_string());
        let c3 = Criptomoneda::new(&"Bitcoin".to_string(),&"BTC".to_string());
        let c4 = Criptomoneda::new(&"Fakex".to_string(),&"FKX".to_string());

        //Creacion de sistema
        let mut sis1 = Plataforma::new("./lista_balances.json","./lista_transacciones.json");
        assert!(sis1.registrar_criptomoneda(c1.clone(),1500.0) );
        assert!(sis1.registrar_criptomoneda(c2.clone(),1000.0) );
        assert!(sis1.registrar_criptomoneda(c3.clone(),5000.0) );
        assert!(!sis1.registrar_criptomoneda(c1.clone(),700.0) );
        assert!(sis1.registrar_criptomoneda(c4.clone(),10.0) );

        //Baja de una criptomoneda
        assert!(sis1.eliminar_criptomoneda(&c4));
        assert!(!sis1.eliminar_criptomoneda(&c4));

        //Obtener cotizacion de una criptomoneda
        assert_eq!(sis1.obtener_cotizacion_criptomoneda(&"Cripton".to_string()),1500.0);

        //Intentar validar un usuario inexistente
        assert!(sis1.validar_usuario(&Usuario::new(&"Pichu", &"Pape", &"pipi@mail.com", 33221)).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }));

        //Operar con un usuario
        assert!(sis1.registrar_usuario(us1.clone()).is_ok());
        assert!(sis1.registrar_usuario(us1.clone()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Denegado(_)))
        }));

        //Operacion de dinero fiat en el unico usuario
        assert!(sis1.validar_usuario(&us1).is_ok());
        assert!(sis1.validar_usuario(&us1).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Denegado(_)))
        }));
        assert!(sis1.ingresar_monto_usuario(&us1,Fecha::new(13,03,2025),10000.0).is_ok());
        assert!(sis1.ingresar_monto_usuario(&Usuario::new(&"Daniel".to_string(),&datos.apellido,&datos.email,datos.dni),Fecha::new(13,03,2025),100000.0).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );

        assert!(sis1.retornar_fiat_usuario(&us1) == 10000.0);
        
        assert!(sis1.retirar_monto_usuario(&us1,Fecha::new(13,03,2025),10000.0,MediosPago::TransferenciaBancaria).is_ok() );
        assert!(sis1.retirar_monto_usuario(&Usuario::new(&"Daniel".to_string(),&datos.apellido,&datos.email,datos.dni),Fecha::new(13,03,2025),100000.0,MediosPago::TransferenciaBancaria).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }));
        
        assert!(sis1.retornar_fiat_usuario(&us1) == 0.0);

        //Resultado de comprobantes(El sistema dispone de comprobantes)
        assert!(!sis1.registro_transacciones.is_empty());
        assert!(sis1.registro_transacciones.len() == 2);
        
        //Operatoria de criptomonedas
        assert!(sis1.ingresar_monto_usuario(&us1,Fecha::new(20,05,2025),10000.0).is_ok());
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),7500.0,c1.get_nombre()).is_ok() );

        assert!(sis1.retornar_fiat_usuario(&us1) == 2500.0);

        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),1500.0,c1.get_nombre()).is_ok() );

        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),500.0,&"Cripton2".to_string()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );
        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),500.0,&"Cripton2".to_string()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),2500.0,c3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Denegado(_)))
        }));

        assert!(sis1.ingresar_monto_usuario(&us1,Fecha::new(20,05,2025),10000.0).is_ok());

        assert_eq!(sis1.retornar_fiat_usuario(&us1),11000.0);
        assert_eq!(sis1.registro_transacciones.len(),6);

        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(10,08,2025),2.0,c1.get_nombre()).is_ok() );
        assert_eq!(sis1.retornar_fiat_usuario(&us1),14000.0);
        assert!(sis1.registro_transacciones.len() == 7);

        assert!(sis1.retirar_monto_usuario(&us1,Fecha::new(10,08,2026), 14000.1, MediosPago::TransferenciaBancaria).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Denegado(_)))
        }));

        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),250000.0,c3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Denegado(_)))
        }));

        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),2500.0,c1.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Denegado(_)))
        }));

        //Rechazar la operacion con criptos si no tiene permiso un usuario o si no existe el mismo
        let mut us3 = Usuario::new(&"Fulano",&"Queseyo",&"cualquiera@mail.com",1234);
        assert!(sis1.comprar_criptomoneda_usuario(&us3,Fecha::new(23,05,2025),250000.0,c3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }));

        assert!(sis1.vender_criptomoneda_usuario(&us3,Fecha::new(23,05,2025),2500.0,c1.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }));
        
        assert!(sis1.registrar_usuario(us3.clone()).is_ok());

        assert!(sis1.retirar_monto_usuario(&us3,Fecha::new(10,08,2026), 14000.1, MediosPago::TransferenciaBancaria).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::SinVerificacion ))
        }));

        assert!(sis1.comprar_criptomoneda_usuario(&us3,Fecha::new(23,05,2025),500.0,c1.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
        }) );
        assert!(sis1.vender_criptomoneda_usuario(&us3,Fecha::new(23,05,2025),500.0,c1.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
        }) );

        //Limpieza de archivos para prevencion de exceso de los mismos
        assert!(std::fs::remove_file("./lista_balances.json").is_ok(),"Error fuera de lo previsto");
        assert!(std::fs::remove_file("./lista_transacciones.json").is_ok(),"Error fuera de lo previsto");  

    }
     
    #[test]
    fn operatoria_blockchains_sistema(){
        //Usuario
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Criptomonedas
        let mut c1 = Criptomoneda::new(&"Bytecoin".to_string(),&"BYT".to_string());
        let mut c2 = Criptomoneda::new(&"Etherum".to_string(),&"ETH".to_string());
        let mut c3 = Criptomoneda::new(&"Bitcoin".to_string(),&"BTC".to_string());

        //Blockchains
        let b1 = Blockchain::new(&"Block1".to_string(),&"BLK1".to_string());
        let b2 = Blockchain::new(&"Block3".to_string(),&"BLK3".to_string());
        let b3 = Blockchain::new(&"Block10".to_string(),&"BLK10".to_string());
        let b4 = Blockchain::new(&"Block27".to_string(),&"BLK27".to_string());

        //Conexiones BL y CR
        assert!(c1.agregar_blockchain(b1.clone()) );
        assert!(c1.agregar_blockchain(b4.clone()) );

        assert!(c2.agregar_blockchain(b2) );
        assert!(c2.agregar_blockchain(b1.clone()) );

        assert!(c3.agregar_blockchain(b1.clone()) );
        assert!(c3.agregar_blockchain(b3.clone()) );

        //Creacion de sistema
        let mut sis1 = Plataforma::new("./lista_balances2.json","./lista_transacciones2.json");

        assert!(sis1.registrar_criptomoneda(c1.clone(),2500.0) );
        assert!(sis1.registrar_criptomoneda(c2.clone(),1000.0) );
        assert!(sis1.registrar_criptomoneda(c3.clone(),5000.0) );

        assert!(sis1.registrar_usuario(us1.clone()).is_ok());

        assert!(sis1.ingresar_monto_usuario(&us1,Fecha::new(20,05,2025),100000.0).is_ok());

        assert!(sis1.validar_usuario(&us1).is_ok());

        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),12500.0,c1.get_nombre()).is_ok() );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),5000.0,c2.get_nombre()).is_ok() );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),25000.0,c3.get_nombre()).is_ok() );

        //Transacciones con blockchains

        //Con usuario inexistente
        let mut us3 = Usuario::new(&"Memo",&"Teso",&"teso@mail.com",4321);
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us3,Fecha::new(23,06,2025),2.0,c1.get_nombre(),&"Bloque1".to_string()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );
        assert!(sis1.blockchain_a_criptomoneda_usuario(&us3,Fecha::new(27,08,2025),4.0,c1.get_nombre(),b3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );
        //Criptomoneda inexistente
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,Fecha::new(23,06,2025),2.0,&"CositoCoin".to_string() ,b3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );
        assert!(sis1.blockchain_a_criptomoneda_usuario(&us1,Fecha::new(27,08,2025),4.0,&"CositoCoin".to_string(),b3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
        }) );
        //Con usuario existente y no verificado
        assert!(sis1.registrar_usuario(us3.clone()).is_ok());
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us3,Fecha::new(23,06,2025),2.0,c1.get_nombre(),b3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::SinVerificacion))
        }) );

        //Con usuario existente y verificado
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,Fecha::new(23,06,2025),2.0,c1.get_nombre(),b1.get_nombre()).is_ok() );
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,Fecha::new(23,06,2025),2.0,c1.get_nombre(),&"Bloque1".to_string()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::FalloTransaccion))
        }) );
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,Fecha::new(13,07,2025),2.0,c1.get_nombre(),b4.get_nombre()).is_ok());
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,Fecha::new(21,07,2025),2.0,c1.get_nombre(),b3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::FalloTransaccion))
        }) );
        assert_eq!(sis1.registro_transacciones.len(),6);

        assert!(sis1.blockchain_a_criptomoneda_usuario(&us1,Fecha::new(27,08,2025),4.0,c1.get_nombre(),b1.get_nombre()).is_ok() );
        assert!(sis1.blockchain_a_criptomoneda_usuario(&us1,Fecha::new(27,08,2025),4.0,c1.get_nombre(),b3.get_nombre()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::FalloTransaccion))
        }) );
        assert!(sis1.blockchain_a_criptomoneda_usuario(&us1,Fecha::new(27,08,2025),4.0,c1.get_nombre(),&"Bloque1".to_string()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::FalloTransaccion))
        }) );
        assert_eq!(sis1.registro_transacciones.len(),7);

        //Limpieza de archivos para prevencion de exceso de los mismos
        assert!(std::fs::remove_file("./lista_balances2.json").is_ok(),"Error fuera de lo previsto");
        assert!(std::fs::remove_file("./lista_transacciones2.json").is_ok(),"Error fuera de lo previsto");  

    }

    
    #[test]
    fn operatoria_estadistica_sistema(){
        //Usuario
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Criptomonedas
        let mut c1 = Criptomoneda::new(&"Hibicoin".to_string(),&"HBC".to_string());
        let mut c2 = Criptomoneda::new(&"Etherum".to_string(),&"ETH".to_string());
        let mut c3 = Criptomoneda::new(&"Bitcoin".to_string(),&"BTC".to_string());

        //Creacion de sistema
        let mut sis1 = Plataforma::new("./lista_balances3.json","./lista_transacciones3.json");

        assert!(sis1.registrar_usuario(us1.clone()).is_ok());

        sis1.registrar_criptomoneda(c1.clone(),1500.0);
        sis1.registrar_criptomoneda(c2.clone(),600.0);
        sis1.registrar_criptomoneda(c3.clone(),2000.0);

        assert!(sis1.ingresar_monto_usuario(&us1,Fecha::new(20,05,2025),900000.0).is_ok());
        assert!(sis1.validar_usuario(&us1).is_ok());

        assert!(sis1.criptomoneda_max_cant_compras().is_none());
        assert!(sis1.criptomoneda_max_cant_ventas().is_none());
        assert!(sis1.criptomoneda_max_monto_compras().is_none());
        assert!(sis1.criptomoneda_max_monto_ventas().is_none());

        //Operaciones de compra y venta        
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),15000.0,c1.get_nombre()).is_ok() );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),15000.0,c1.get_nombre()).is_ok());
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),15000.0,c1.get_nombre()).is_ok());
        
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),1200.0,c2.get_nombre()).is_ok());
        
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),4000.0,c3.get_nombre()).is_ok());
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),4000.0,c3.get_nombre()).is_ok());
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),4000.0,c3.get_nombre()).is_ok());
        assert!(sis1.comprar_criptomoneda_usuario(&us1,Fecha::new(23,05,2025),4000.0,c3.get_nombre()).is_ok());

        assert_eq!(sis1.registro_transacciones.len(),9);
        
        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(25,05,2025),10.0,c1.get_nombre()).is_ok());
        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(25,05,2025),10.0,c1.get_nombre()).is_ok());
        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(25,05,2025),10.0,c1.get_nombre()).is_ok());

        assert!(sis1.vender_criptomoneda_usuario(&us1,Fecha::new(25,05,2025),2.0,c3.get_nombre()).is_ok());

        assert_eq!(sis1.registro_transacciones.len(),13);

        assert!(sis1.criptomoneda_max_cant_compras().is_some_and(|res| res == *c3.get_nombre()),"Aqui no tuvo que fallar");

        assert!(sis1.criptomoneda_max_monto_compras().is_some_and(|res| res == *c1.get_nombre()),"Aqui no tuvo que fallar");

        assert!(sis1.criptomoneda_max_cant_ventas().is_some_and(|res| res == *c1.get_nombre()),"Aqui no tuvo que fallar");
        
        assert!(sis1.criptomoneda_max_monto_ventas().is_some_and(|res| res == *c1.get_nombre()),"Aqui no tuvo que fallar");
        
        //Limpieza de archivos para prevencion de exceso de los mismos
        assert!(std::fs::remove_file("./lista_balances3.json").is_ok(),"Error fuera de lo previsto");
        assert!(std::fs::remove_file("./lista_transacciones3.json").is_ok(),"Error fuera de lo previsto");  

    }
    
    #[test]
    fn prueba_comprobantes(){
        let d = DatosPersona { nombre: "asd".to_string(), apellido: "fd".to_string(), email: "asdf".to_string(), dni: 123 };
        let c1 = Datos_Ingreso::new(d.clone(), Fecha::new(12, 4, 26), 100.0);
        assert!(c1.get_fecha().es_igual_a(&Fecha::new(12, 4, 26)));
        assert!(c1.get_monto()>0.0);
        let c2 = Datos_Retiro::new(d.clone(), Fecha::new(12,4,26), 100.0, MediosPago::MercadoPago);
        assert!(matches!(c2.get_medio_pago(),MediosPago::MercadoPago));
        let c3 = Datos_Operacion_Criptomoneda::new(d.clone(), Fecha::new(1,1,26), 100.0, Criptomoneda::new("asd", "a"), 100.0);
        assert!(c3.get_cotizacion()>0.0);
        assert_eq!(*c3.get_cripto_nom(),"asd".to_string());
        assert_eq!(*c3.get_criptomoneda().get_nombre(),"asd".to_string());
        let c4 = Datos_Extraccion_Blockchain::new(d.clone(), Fecha::new(1,1,26), 10.0, Criptomoneda::new("asd", "a"), 1.0, Blockchain { nombre: "a".to_string(), prefijo: "f".to_string() });
        assert_eq!(*c4.get_blockchain().get_nombre(),"a".to_string());
        let c5 = Datos_Retiro_Blockchain::new(d, Fecha::new(1,1,26), 1.0, Criptomoneda::new("asd", "a"), 2.0, Blockchain { nombre: "a".to_string(), prefijo: "f".to_string() });
        assert_eq!(*c5.get_blockchain().get_nombre(),"a".to_string());
        assert!(!c5.get_hash().is_empty());
    }   

    /*
		Casos especiales para la cobertura de coverage
	*/
	#[test]
	fn caso_especial_error_io_balances() {
		// Se buscara forzar un ErrorIO usando una ruta cuyo directorio base no existe
		let path_err = "./carpeta_inexistente_123/x.json";

        let mut sis1 = Plataforma::new(path_err,"./lista_transacciones4.json");
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        // Al intentar crear una wallet, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO
		assert!(sis1.registrar_usuario(us1).is_err_and(|e|{
                assert!(!e.to_string().is_empty());
			    matches!(e, Errores::ErrorIO(_))
        }),"Ocurrio un error imprevisto");
	}

    #[test]
	fn caso_especial_error_io_transacciones() {
		// Se buscara forzar un ErrorIO usando una ruta cuyo directorio base no existe
		let path_err = "./carpeta_inexistente_123/x.json";

		let mut sis1 = Plataforma::new("./equisde.json",path_err);
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        assert!(sis1.registrar_usuario(us1.clone()).is_ok() );
		assert!(sis1.ingresar_monto_usuario(&us1,Fecha::new(31, 2, 2026) , 1000.0).is_err_and(|e|{
                assert!(!e.to_string().is_empty());
			    matches!(e, Errores::ErrorIO(_))
        }),"Ocurrio un error imprevisto");
        assert!(std::fs::remove_file("./equisde.json").is_ok(),"Error fuera de lo previsto");

	}

	#[test]
	fn caso_especial_error_serde() {
		let path_err = "./corrupto.json";
		
		// Se fuerza la escritura en el contenido temporal que NO cumple con el formato estructurado de un .JSON válido
		assert!(std::fs::write(path_err, "{ &&5435#$#$&42365_XXXX1234 : [::: ").is_ok(),"No debio fallar aqui");

		// Se invoca directamente el método para leer el archivo del path que buscara

		assert!(Plataforma::recuperar_balances_info(path_err).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorSerde(_))
		}),"Aquí debió fallar");

        assert!(Plataforma::recuperar_transacciones_info(path_err).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorSerde(_))
		}),"Aquí debió fallar");

		assert!(std::fs::remove_file(path_err).is_ok(),"Error fuera de lo previsto");
		
	}
}