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

#[derive(PartialEq,Debug,Clone)]
pub struct Blockchain{
    nombre : String,
    prefijo : String
}

impl Blockchain{
    fn new(nom:&String,pre:&String)->Blockchain{
        return Blockchain { nombre: nom.clone(), prefijo: pre.clone() }
    }
    fn es_igual_a(&self,nom:&String)->bool{
        return &self.nombre == nom;
    }
    fn generar_hash(&self,tam:usize)->String{
        aleatorio(tam)
    }
    fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Criptomoneda{
    nombre : String,
    prefijo : String,
    blockchains : Vec<Blockchain>
}

impl Criptomoneda{
    fn new(nom:&String,pre:&String)->Criptomoneda{
        return Criptomoneda { nombre: nom.clone(), prefijo: pre.clone(), blockchains: Vec::new()}
    }
    fn agregar_blockchain(&mut self,b:&Blockchain)->bool{
        let mut pude = false;
        
        if self.blockchains.iter().find(|&blockchain| blockchain == b ).is_none() {
            self.blockchains.push(b.clone());
            pude = true;
        }

        return pude;
    }
    fn eliminar_blockchain(&mut self,b:&Blockchain)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.blockchains.iter().position(|blockchain| blockchain == b){
            self.blockchains.remove(pos);
            pude = true;
        }

        return pude;
    }
    fn dispone_blockchains(&self)->bool{
        return !self.blockchains.is_empty();
    }
    fn blockchain_encontrado(&self,nom:&String)->bool{
        let mut res = self.dispone_blockchains();
        if res{
            res = self.blockchains.iter().find(|&blockchain| blockchain.es_igual_a(nom)).is_some();
        }
        return res;
    }
    fn get_blockchain(&self,nom:&String)->Option<Blockchain>{
        let mut res : Option<Blockchain> = None;
        if let Some(b) = self.blockchains.iter().find(|&blockchain| blockchain.es_igual_a(nom)){
            res = Some(b.clone());
        }
        return res;
    }
    fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    fn get_prefijo(&self)->String{
        return self.prefijo.clone();
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct CriptomonedaDispone(String,f64);

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
        return self.0 == nom.clone();
    }
    fn get_monto(&self)->f64{
        return self.1;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct BalancePropio{
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
    fn agregar_criptomoneda(&mut self,nom:&String,monto:f64){
        self.criptomonedas.push(CriptomonedaDispone(nom.clone(),monto));
    }
    fn contabilizar_criptomoneda(&mut self,nom:&String,monto:f64)->bool{
        let mut pude = false;
        
        if let Some(dato) = self.criptomonedas.iter_mut().find(|cripto| cripto.es_igual_a(&nom)){
            dato.contabilizar(monto);
            pude = true;
        }

        return pude;
    }
    fn descontabilizar_criptomoneda(&mut self,nom:&String,monto:f64)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.criptomonedas.iter().position(|cripto| cripto.es_igual_a(&nom)){
            pude = self.criptomonedas[pos].descontabilizar(monto);
            if self.criptomonedas[pos].get_monto() <= 0.0 {
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

#[derive(PartialEq,Debug,Clone)]
pub struct DatosPersona{
    nombre : String,
    apellido : String,
    email : String,
    dni : u64
}

pub trait InformacionPersonal{
    fn get_nombre(&self, datos : &DatosPersona)->String{
        return datos.nombre.clone();
    }
    fn get_apellido(&self, datos: &DatosPersona)->String{
        return datos.apellido.clone();
    }
    fn get_email(&self, datos : &DatosPersona)->String{
        return datos.email.clone();
    }
    fn get_dni(&self, datos : &DatosPersona)->u64{
        return datos.dni;
    }
    fn informacion_correcta(&self, info:&DatosPersona)->bool;
}

#[derive(PartialEq,Debug,Clone)]
pub struct Usuario{
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
    fn new(nom:&String,ape:&String,mail:&String,dni_in:u64)->Usuario{
        return Usuario { datos: DatosPersona { nombre: nom.clone(), apellido: ape.clone(), email: mail.clone(), dni: dni_in} , validado: false, balance: BalancePropio::new()}
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
    fn comprar_criptomoneda(&mut self,nom:&String,montoCompra:f64,montoComprado:f64)->bool{
        let mut pude = false;
        if self.validado {
            pude = self.retirar_monto_fiat(montoCompra);
            if pude {
                if !self.balance.contabilizar_criptomoneda(nom,montoComprado){
                    self.balance.agregar_criptomoneda(nom,montoComprado);
                }
            }
        }
        return pude;
    }
    fn vender_criptomoneda(&mut self,nom:&String,montoVenta:f64,gananciaObtenida:f64)->bool{
        let mut pude = false;
        if self.validado {
            pude = self.balance.descontabilizar_criptomoneda(nom,montoVenta);            
            if pude {
                self.balance.contabilizar_fiat(gananciaObtenida);
            }
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
    Estructuras pertenciente al sistema
*/


#[derive(PartialEq,Debug,Clone)]
pub struct Fecha(u8,u8,u64);

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Ingreso{
    datos_usuario : DatosPersona,
    fecha: Fecha,
    monto : f64
}

impl Datos_Ingreso{
    fn new(datos : &DatosPersona, f : &Fecha , m : f64)->Datos_Ingreso{
        return Datos_Ingreso { datos_usuario: datos.clone(), fecha: f.clone(), monto: m };
    }
    fn get_fecha(&self)->Fecha{
        return self.fecha.clone();
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

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Retiro{
    datos_genericos : Datos_Ingreso,
    medio_pago : MediosPago
}

impl Datos_Retiro{
    fn new(d:&DatosPersona,fe:&Fecha,monto:f64,medio:&MediosPago)->Datos_Retiro{
        return Datos_Retiro { datos_genericos: Datos_Ingreso::new(d, fe , monto), medio_pago: medio.clone() }
    }

    fn get_medio_pago(&self)->MediosPago{
        return self.medio_pago.clone();
    }
}

//Tipos de transacciones implementados
#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Operacion_Criptomoneda{
    datos_genericos : Datos_Ingreso,
    criptomoneda : Criptomoneda,
    cotizacion : f64
}

impl Datos_Operacion_Criptomoneda{
    fn new(user:&DatosPersona,f:&Fecha,m:f64,c:&Criptomoneda,cotiz:f64)->Datos_Operacion_Criptomoneda{
        return Datos_Operacion_Criptomoneda { 
            datos_genericos: Datos_Ingreso::new(user, f, m ),
            criptomoneda: c.clone(),
            cotizacion: cotiz };
    }
    fn get_monto_operacion(&self)->f64{
        return self.datos_genericos.get_monto();
    }
    fn get_cripto_nom(&self)->String{
        return self.criptomoneda.get_nombre();
    }
    fn get_criptomoneda(&self)->Criptomoneda{
        return self.criptomoneda.clone();
    }
    fn get_cotizacion(&self)->f64{
        return self.cotizacion;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Retiro_Blockchain{
    datos_criptomoneda : Datos_Operacion_Criptomoneda,
    blockchain : Blockchain,
    hash : String
}

impl Datos_Retiro_Blockchain{
    fn new(user:&DatosPersona,f:&Fecha,m:f64,c:&Criptomoneda,cotiz:f64,b:&Blockchain)->Datos_Retiro_Blockchain{
        return Datos_Retiro_Blockchain { datos_criptomoneda: Datos_Operacion_Criptomoneda::new(user, f, m, c, cotiz),
             blockchain: b.clone(),
            hash: b.generar_hash(5) }
    }
    fn get_blockchain(&self)->Blockchain{
        return self.blockchain.clone();
    }
    fn get_hash(&self)->String{
        return self.hash.clone();
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Extraccion_Blockchain{
    datos_criptomoneda : Datos_Operacion_Criptomoneda,
    blockchain : Blockchain
}

impl Datos_Extraccion_Blockchain{
    fn new(user:&DatosPersona,f:&Fecha,m:f64,c:&Criptomoneda,cotiz:f64,b:&Blockchain)->Datos_Extraccion_Blockchain{
        return Datos_Extraccion_Blockchain { datos_criptomoneda: Datos_Operacion_Criptomoneda::new(user, f, m, c, cotiz),
             blockchain: b.clone()
        }
    }
    fn get_blockchain(&self)->Blockchain{
        return self.blockchain.clone();
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum TiposTransacciones{
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
    pub fn obtener_nombre_criptomoneda(&self)->Option<String>{
        let mut res : Option<String> = None;

        //Solamente se procesan los tipos compra y venta de criptomonedas , los demas se los excluye para la resolucion principal
        match self{
            TiposTransacciones::CompraCriptomoneda(datos) => res = Some(datos.get_cripto_nom()),
            TiposTransacciones::VentaCriptomoneda(datos) => res = Some(datos.get_cripto_nom()),
            _ => todo!(), //Descarta cualquiera de los tipos del enum(TiposTransacciones) pero se lo deja como una posible implementacion a futuro
        }

        return res;
    }
    pub fn obtener_monto_criptomoneda(&self)->Option<f64>{
        let mut res : Option<f64> = None;

        //Solamente se procesan los tipos compra y venta de criptomonedas , los demas se los excluye para la resolucion principal
        match self{
            TiposTransacciones::CompraCriptomoneda(datos) => res = Some(datos.get_monto_operacion()),
            TiposTransacciones::VentaCriptomoneda(datos) => res = Some(datos.get_monto_operacion()),
            _ => todo!(), //Descarta cualquiera de los tipos del enum(TiposTransacciones) pero se lo deja como una posible implementacion a futuro
        }

        return res;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum MediosPago{
    MercadoPago,
    TransferenciaBancaria
}

#[derive(PartialEq,Debug,Clone)]
//Valor(Unidad) = Se ingresa el monto equivalente en fiat
pub struct Criptomoneda_disponible(Criptomoneda,f64);
impl Criptomoneda_disponible{
    fn get_cotiza(&self)->f64{
        return self.1;
    }
    
}



/*
    Estructura principal : Sistema
*/


#[derive(PartialEq,Debug,Clone)]
pub struct Plataforma{
    usuarios : Vec<Usuario>,
    criptomonedas_dispone : Vec<Criptomoneda_disponible>, //Datos de la criptomoneda y cotiza
    registro_transacciones : Vec<TiposTransacciones>
}

impl Plataforma{
    //Funciones secundarias
    fn new()->Plataforma{
        return Plataforma{usuarios:Vec::new(),criptomonedas_dispone:Vec::new(),registro_transacciones:Vec::new()}
    }
    fn eliminar_criptomoneda(&mut self,cripto:&Criptomoneda)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.criptomonedas_dispone.iter().position(|c| c.0.get_nombre() == cripto.get_nombre()){
            pude = true;
            self.criptomonedas_dispone.remove(pos);
        }       

        return pude;
    }
    fn registrar_criptomoneda(&mut self,cripto:&Criptomoneda,cotiza:f64)->bool{
        let mut pude = false;
        
        if self.criptomonedas_dispone.iter().find(|&c| c.0.get_nombre() == cripto.get_nombre() ).is_none(){
            self.criptomonedas_dispone.push(Criptomoneda_disponible(cripto.clone(),cotiza));
            pude = true;
        }        

        return pude;
    }
    fn obtener_cotizacion_criptomoneda(&self,nom:&String)->f64{
        let mut total = 0.0;
        if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nom){
            total = cr.get_cotiza();
        }
        return total;
    }
    fn registrar_transaccion(&mut self,t : &TiposTransacciones){
        self.registro_transacciones.push(t.clone());
    }
    fn registrar_usuario(&mut self,u1:&Usuario)->bool{
        let mut pude = false;
        
        if self.usuarios.iter().find(|user| user.informacion_correcta(&u1.datos)).is_none(){
            self.usuarios.push(u1.clone());
            pude = true;
        }        

        return pude;
    }
    fn retornar_fiat_usuario(&mut self,u1:&Usuario)->f64{
        let mut res = 0.0;
        
        if let Some(u) = self.usuarios.iter().find(|user| user.informacion_correcta(&u1.datos)){
            res = u.get_balance_fiat();
        }        

        return res;
    }
    fn validar_usuario(&mut self,u1:&Usuario)->bool{
        let mut pude = false;
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if !u.is_verificado(){
                u.cambiar_verificacion();
                pude = true;
            }
        }       

        return pude;
    }
    //Funciones primarias
    fn ingresar_monto_usuario(&mut self,u1:&Usuario,f:&Fecha,m:f64)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            u.ingresar_monto_fiat(m);
            let datos = Datos_Ingreso::new(&u.datos, f, m);
            self.registrar_transaccion(&TiposTransacciones::IngresoFiat(datos));
            completo = true;
        }
        return completo;
    }
    //Se ingresa la cantidad de cripto que se desea
    fn comprar_criptomoneda_usuario(&mut self,u1:&Usuario,f:&Fecha,montoCompra:f64,nom:&String)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nom){
                //Compra
                let montoResultante = cr.get_cotiza()*montoCompra;
                if u.get_balance_fiat() >= montoResultante{
                    
                    completo = u.comprar_criptomoneda(nom,montoResultante,montoCompra);

                    if completo {
                        //Generacion de comprobante
                        let datos = Datos_Operacion_Criptomoneda::new(&u.datos.clone(),&f.clone(),montoCompra,&cr.0.clone(),cr.get_cotiza());
                        self.registrar_transaccion(&TiposTransacciones::CompraCriptomoneda(datos));
                    }
                }
            }
        }
        return completo;
    }
    //Se ingresa la cantidad de cripto que se desea
    fn vender_criptomoneda_usuario(&mut self,u1:&Usuario,f:&Fecha,montoVenta:f64,nom:&String)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nom){
                //Venta
                let montoObtenido = cr.1 * montoVenta;
                completo = u.vender_criptomoneda(nom,montoVenta,montoObtenido);

                if completo {
                    //Generacion de comprobante
                    let datos = Datos_Operacion_Criptomoneda::new(&u.datos.clone(),&f.clone(),montoVenta,&cr.0.clone(),cr.get_cotiza());
                    self.registrar_transaccion(&TiposTransacciones::VentaCriptomoneda(datos));
                }
                
            }
        }
        return completo;
    }
    fn criptomoneda_a_blockchain_usuario(&mut self,u1:&Usuario,f:&Fecha,montoTransaccion:f64,nomCripto:&String,nomBlockchain:&String)->bool{
        let mut pude = false;
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nomCripto){
                //Buscar blockchain en la criptomoneda
                pude = u.criptomoneda_a_blockchain(nomCripto,montoTransaccion,nomBlockchain,&cr.0);

                //Generar comprobante
                if pude {
                    if let Some(b) = cr.0.get_blockchain(nomBlockchain){
                        let datos = Datos_Retiro_Blockchain::new(&u.datos.clone(),&f.clone(),montoTransaccion,&cr.0.clone(),cr.get_cotiza(),&b);
                        self.registrar_transaccion(&TiposTransacciones::RetiroCriptomoneda(datos));
                    }
                }
            }
        }

        return pude;
    }
    fn blockchain_a_criptomoneda_usuario(&mut self,u1:&Usuario,f:&Fecha,montoTransaccion:f64,nomCripto:&String,nomBlockchain:&String)->bool{
        let mut pude = false;
        
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nomCripto){
                //Buscar blockchain en la criptomoneda
                pude = u.blockchain_a_criptomoneda(nomCripto,montoTransaccion,nomBlockchain,&cr.0);

                //Generar comprobante
                if pude {
                    if let Some(b) = cr.0.get_blockchain(nomBlockchain){
                        let datos = Datos_Extraccion_Blockchain::new(&u.datos.clone(),&f.clone(),montoTransaccion,&cr.0.clone(),cr.get_cotiza(),&b);
                        self.registrar_transaccion(&TiposTransacciones::RecepcionCriptomoneda(datos));
                    }
                }
            }
        }

        return pude;
    }
    fn retirar_monto_usuario(&mut self,u1:&Usuario,f:&Fecha,m:f64,med:&MediosPago)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            u.retirar_monto_fiat(m);
            let datos = Datos_Retiro::new(&u.datos, f, m,med);
            self.registrar_transaccion(&TiposTransacciones::RetiroFiat(datos));
            completo = true;
        }
        return completo;
    }

    /*
        Funciones estadistica para comprobantes generados por el sistema
    */
    fn criptomoneda_max_cant_compras(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : Vec<(String,u32)> = Vec::new();

            for comprobante in &self.registro_transacciones {
                if comprobante.es_tipo_compra() {
                    if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                
                        match contador.iter_mut().find(|(n, _)| *n == nombre) {
                            Some((_, cant)) => *cant += 1,  
                            None => contador.push((nombre, 1)),  
                        }
                        
                    }
                }
            }
            
            if !contador.is_empty(){
                let mut cant: u32 = 0;
                let mut max : String = "".to_string();

                for nom in contador{
                    if nom.1 > cant {
                        cant = nom.1;
                        max = nom.0;
                    }
                }
             
                res = Some(max);
            }
        }

        return res;
    }

    fn criptomoneda_max_cant_ventas(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : Vec<(String,u32)> = Vec::new();

            for comprobante in &self.registro_transacciones {
                if comprobante.es_tipo_venta() {
                    if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                
                        match contador.iter_mut().find(|(n, _)| *n == nombre) {
                            Some((_, cant)) => *cant += 1,  
                            None => contador.push((nombre, 1)),  
                        }
                        
                    }
                }
            }
            
            if !contador.is_empty(){
                let mut cant: u32 = 0;
                let mut max : String = "".to_string();

                for nom in contador{
                    if nom.1 > cant {
                        cant = nom.1;
                        max = nom.0;
                    }
                }
             
                res = Some(max);
            }
        }

        return res;
    }

    fn criptomoneda_max_monto_compras(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : Vec<(String,f64)> = Vec::new();

            for comprobante in &self.registro_transacciones {
                if comprobante.es_tipo_compra() {
                    if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                        if let Some(monto) = comprobante.obtener_monto_criptomoneda(){
                            match contador.iter_mut().find(|(n, _)| *n == nombre) {
                                Some((_, cant)) => *cant += monto,  
                                None => contador.push((nombre, monto)),  
                            }
                        }
                        
                    }
                }
            }
            
            if !contador.is_empty(){
                let mut cant: f64 = 0.0;
                let mut max : String = "".to_string();

                for nom in contador{
                    if nom.1 > cant {
                        cant = nom.1;
                        max = nom.0;
                    }
                }
             
                res = Some(max);
            }
        }

        return res;
    }
    fn criptomoneda_max_monto_ventas(&self)->Option<String>{
        let mut res : Option<String> = None;

        if !self.registro_transacciones.is_empty(){

            let mut contador : Vec<(String,f64)> = Vec::new();

            for comprobante in &self.registro_transacciones {
                if comprobante.es_tipo_venta() {
                    if let Some(nombre) = comprobante.obtener_nombre_criptomoneda() {
                        if let Some(monto) = comprobante.obtener_monto_criptomoneda(){
                            match contador.iter_mut().find(|(n, _)| *n == nombre) {
                                Some((_, cant)) => *cant += monto,  
                                None => contador.push((nombre, monto)),  
                            }
                        }
                        
                    }
                }
            }
            
            if !contador.is_empty(){
                let mut cant: f64 = 0.0;
                let mut max : String = "".to_string();

                for nom in contador{
                    if nom.1 > cant {
                        cant = nom.1;
                        max = nom.0;
                    }
                }
             
                res = Some(max);
            }
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
        assert!(!c.dispone_blockchains());
        assert!(!c.get_nombre().is_empty());
        assert!(!c.get_prefijo().is_empty());
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
        assert!(c.agregar_blockchain(&b1));
        assert!(c.agregar_blockchain(&b2));
        assert!(c.agregar_blockchain(&b3));
        assert!(!c.agregar_blockchain(&b1));    //Ya existe un vinculo por lo que no existe una revinculacion
        assert!(c.dispone_blockchains());

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
        assert!(!c.dispone_blockchains());
    }

    #[test]
    fn prueba_datos_personales_usuario(){
        let datos = DatosPersona{nombre : "Marcos".to_string(),apellido : "Deltodo".to_string(),email : "exmpl@example.com".to_string(),dni : 1234876};
        let mut us1 = Usuario::new(&datos.nombre,&datos.apellido,&datos.email,datos.dni);

        //Informacion correcta
        assert!(us1.informacion_correcta(&datos));
        assert!(!us1.get_nombre(&us1.datos).is_empty());
        assert!(!us1.get_apellido(&us1.datos).is_empty());
        assert!(!us1.get_email(&us1.datos).is_empty());
        assert_eq!(us1.get_dni(&us1.datos),1234876);

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

        assert!(us1.comprar_criptomoneda(&c1.get_nombre(),5000.0,10.0));
        assert!(us1.comprar_criptomoneda(&c2.get_nombre(),5000.0,40.0));
        assert!(!us1.comprar_criptomoneda(&c3.get_nombre(),25000.0,10.0));
        assert!(us1.comprar_criptomoneda(&c3.get_nombre(),10000.0,10.0));
        assert_eq!(us1.get_balance_fiat(),0.0); 

        //Simulacion de venta de criptomonedas
        assert!(us1.vender_criptomoneda(&c3.get_nombre(),5.0,20000.0));
        assert!(!us1.vender_criptomoneda(&"Robux".to_string(),10.0,10000.0));
        assert!(us1.vender_criptomoneda(&c1.get_nombre(),10.0,8000.0));
        assert!(!us1.vender_criptomoneda(&c1.get_nombre(),10.0,8000.0));
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

        c.agregar_blockchain(&b1);
        c.agregar_blockchain(&b2);
        c.agregar_blockchain(&b3);

        //Ingreso de monto fiat
        us1.ingresar_monto_fiat(10000.0);
        //Compra de criptomoneda
        us1.cambiar_verificacion();
        assert!(!us1.dispone_criptomonedas());
        assert!(us1.comprar_criptomoneda(&c.get_nombre(),10000.0,100.0));

        //Transaccion
        assert!(us1.criptomoneda_a_blockchain(&c.get_nombre(),50.0,&b1.get_nombre(),&c));
        assert!(!us1.criptomoneda_a_blockchain(&"BitCoin".to_string(),50.0,&b1.get_nombre(),&c));
        assert!(!us1.criptomoneda_a_blockchain(&c.get_nombre(),50.0,&"Block1".to_string(),&c));

        assert!(us1.blockchain_a_criptomoneda(&c.get_nombre(),50.0,&b1.get_nombre(),&c));
        assert!(!us1.blockchain_a_criptomoneda(&"BitCoin".to_string(),50.0,&b1.get_nombre(),&c));
        assert!(!us1.blockchain_a_criptomoneda(&c.get_nombre(),50.0,&"Block1".to_string(),&c));
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
        let mut sis1 = Plataforma::new();
        assert!(sis1.registrar_criptomoneda(&c1.clone(),1500.0) );
        assert!(sis1.registrar_criptomoneda(&c2.clone(),1000.0) );
        assert!(sis1.registrar_criptomoneda(&c3.clone(),5000.0) );
        assert!(!sis1.registrar_criptomoneda(&c1.clone(),700.0) );
        assert!(sis1.registrar_criptomoneda(&c4.clone(),10.0) );

        //Baja de una criptomoneda
        assert!(sis1.eliminar_criptomoneda(&c4));
        assert!(!sis1.eliminar_criptomoneda(&c4));

        //Obtener cotizacion de una criptomoneda
        assert_eq!(sis1.obtener_cotizacion_criptomoneda(&"Cripton".to_string()),1500.0);

        assert!(sis1.registrar_usuario(&us1));
        assert!(!sis1.registrar_usuario(&us1));

        //Operacion de dinero fiat en el unico usuario
        assert!(sis1.ingresar_monto_usuario(&us1,&Fecha(13,03,2025),10000.0) );
        assert!(!sis1.ingresar_monto_usuario(&Usuario::new(&"Daniel".to_string(),&datos.apellido,&datos.email,datos.dni),&Fecha(13,03,2025),100000.0) );

        assert!(sis1.retornar_fiat_usuario(&us1) == 10000.0);
        
        assert!(sis1.retirar_monto_usuario(&us1,&Fecha(13,03,2025),10000.0,&MediosPago::TransferenciaBancaria) );
        assert!(!sis1.retirar_monto_usuario(&Usuario::new(&"Daniel".to_string(),&datos.apellido,&datos.email,datos.dni),&Fecha(13,03,2025),100000.0,&MediosPago::TransferenciaBancaria) );
        
        assert!(sis1.retornar_fiat_usuario(&us1) == 0.0);

        //Resultado de comprobantes(El sistema dispone de comprobantes)
        assert!(!sis1.registro_transacciones.is_empty());
        assert!(sis1.registro_transacciones.len() == 2);
        
        //Operatoria de criptomonedas
        sis1.ingresar_monto_usuario(&us1,&Fecha(20,05,2025),10000.0);
        assert!(!sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),5.0,&c1.get_nombre()) );
        assert!(sis1.validar_usuario(&us1));
        assert!(!sis1.validar_usuario(&us1));

        assert!(sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),5.0,&c1.get_nombre()) );
        assert!(sis1.retornar_fiat_usuario(&us1) == 2500.0);

        assert!(!sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),5.0,&"Cripton2".to_string()) );
        assert!(!sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),15.0,&c3.get_nombre()) );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),1.0,&c2.get_nombre()) );

        assert_eq!(sis1.retornar_fiat_usuario(&us1),1500.0);
        assert!(sis1.registro_transacciones.len() == 5);

        assert!(sis1.vender_criptomoneda_usuario(&us1,&Fecha(10,08,2025),2.0,&c1.get_nombre()) );
        assert_eq!(sis1.retornar_fiat_usuario(&us1),4500.0);
        assert!(sis1.registro_transacciones.len() == 6);

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
        assert!(c1.agregar_blockchain(&b1) );
        assert!(c1.agregar_blockchain(&b4) );

        assert!(c2.agregar_blockchain(&b2) );
        assert!(c2.agregar_blockchain(&b1) );

        assert!(c3.agregar_blockchain(&b1) );
        assert!(c3.agregar_blockchain(&b3) );

        //Creacion de sistema
        let mut sis1 = Plataforma::new();

        assert!(sis1.registrar_criptomoneda(&c1.clone(),2500.0) );
        assert!(sis1.registrar_criptomoneda(&c2.clone(),1000.0) );
        assert!(sis1.registrar_criptomoneda(&c3.clone(),5000.0) );

        sis1.registrar_usuario(&us1);

        sis1.ingresar_monto_usuario(&us1,&Fecha(20,05,2025),100000.0);

        sis1.validar_usuario(&us1);

        assert!(sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),5.0,&c1.get_nombre()) );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),5.0,&c2.get_nombre()) );
        assert!(sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),5.0,&c3.get_nombre()) );

        //Transacciones con blockchains
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,&Fecha(23,06,2025),2.0,&c1.get_nombre(),&b1.get_nombre()) );
        assert!(!sis1.criptomoneda_a_blockchain_usuario(&us1,&Fecha(23,06,2025),2.0,&c1.get_nombre(),&"Bloque1".to_string()) );
        assert!(sis1.criptomoneda_a_blockchain_usuario(&us1,&Fecha(13,07,2025),2.0,&c1.get_nombre(),&b4.get_nombre()) );
        assert!(!sis1.criptomoneda_a_blockchain_usuario(&us1,&Fecha(21,07,2025),2.0,&c1.get_nombre(),&b3.get_nombre()) );
        assert_eq!(sis1.registro_transacciones.len(),6);

        assert!(sis1.blockchain_a_criptomoneda_usuario(&us1,&Fecha(27,08,2025),4.0,&c1.get_nombre(),&b1.get_nombre()) );
        assert!(!sis1.blockchain_a_criptomoneda_usuario(&us1,&Fecha(27,08,2025),4.0,&c1.get_nombre(),&b3.get_nombre()) );
        assert!(!sis1.blockchain_a_criptomoneda_usuario(&us1,&Fecha(27,08,2025),4.0,&c1.get_nombre(),&"Bloque1".to_string()) );
        assert_eq!(sis1.registro_transacciones.len(),7);
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
        let mut sis1 = Plataforma::new();

        sis1.registrar_usuario(&us1);

        sis1.registrar_criptomoneda(&c1.clone(),1500.0);
        sis1.registrar_criptomoneda(&c2.clone(),600.0);
        sis1.registrar_criptomoneda(&c3.clone(),2000.0);

        sis1.ingresar_monto_usuario(&us1,&Fecha(20,05,2025),900000.0);
        sis1.validar_usuario(&us1);

        assert!(sis1.criptomoneda_max_cant_compras().is_none());

        //Operaciones de compra y venta        
        assert!(sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),2.0,&c1.get_nombre()) );
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),2.0,&c1.get_nombre());
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),2.0,&c1.get_nombre());
        
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),2.0,&c2.get_nombre());
        
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),1.0,&c3.get_nombre());
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),1.0,&c3.get_nombre());
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),1.0,&c3.get_nombre());
        sis1.comprar_criptomoneda_usuario(&us1,&Fecha(23,05,2025),1.0,&c3.get_nombre());

        assert_eq!(sis1.registro_transacciones.len(),9);
        
        assert!(sis1.vender_criptomoneda_usuario(&us1,&Fecha(25,05,2025),2.0,&c1.get_nombre()) );
        sis1.vender_criptomoneda_usuario(&us1,&Fecha(25,05,2025),2.0,&c1.get_nombre());
        sis1.vender_criptomoneda_usuario(&us1,&Fecha(25,05,2025),2.0,&c1.get_nombre());

        sis1.vender_criptomoneda_usuario(&us1,&Fecha(25,05,2025),2.0,&c3.get_nombre());   

        assert_eq!(sis1.registro_transacciones.len(),13);

        if let Some(res) = sis1.criptomoneda_max_cant_compras(){
            assert_eq!(res ,c3.get_nombre() );
            assert!(!(res == "asd".to_string()) );
        }

        if let Some(res) = sis1.criptomoneda_max_monto_compras(){
            assert_eq!(res ,c1.get_nombre() );
            assert!(!(res == "asd".to_string()) );
        }

        if let Some(res) = sis1.criptomoneda_max_cant_ventas(){
            assert_eq!(res ,c1.get_nombre() );
            assert!(!(res == "asd".to_string()) );
        }
        
        if let Some(res) = sis1.criptomoneda_max_monto_ventas(){
            assert_eq!(res ,c1.get_nombre() );
            assert!(!(res == "asd".to_string()) );
        }

    }

}