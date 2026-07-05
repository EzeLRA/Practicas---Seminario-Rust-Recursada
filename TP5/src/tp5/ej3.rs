/*
    IMPLEMENTACION DE EJERCICIO 3 - TP5
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
    EstructuraVacia(String)
}

impl Display for error_operatoria{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            error_operatoria::Inexistente(val) => write!(f, "No se encontro el elemento en la estructura {} ",val),
            error_operatoria::EstructuraVacia(val) => write!(f, "La estrucutra {} no dispone de elementos ",val)
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
    Extraccion Ejercicio9 TP3
*/
/* 
    Estructura Fecha - Ejercicio3 - TP3
*/

use std::collections::VecDeque;

//Atributos
#[derive(Debug,Clone,Serialize,Deserialize)]
struct Fecha{
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
mod testing_ejercicio9_fecha{
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
    Estructuras
*/

#[derive(Debug, Clone,Serialize,Deserialize)]
enum Animales{
    Perro,
    Gato,
    Caballo,
    Otro,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
struct Duenio {
    nombre: String,
    direccion: String,
    telefono: u32
}
#[derive(Debug, Clone,Serialize,Deserialize)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animales,
    duenio: Duenio
}
#[derive(Debug, Clone,Serialize,Deserialize)]
struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>
}
#[derive(Debug, Clone)]
struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>,
    atenciones_realizadas: Vec<Atencion>,
    path:String
}

/*
    Metodos asociados
*/
impl Animales{
    pub fn es_igual_a(&self,a:&Animales)->bool{
        match (self, a) {
            (Animales::Perro, Animales::Perro) => true,
            (Animales::Gato, Animales::Gato) => true,
            (Animales::Caballo, Animales::Caballo) => true,
            (Animales::Otro, Animales::Otro) => true,
            _ => false
        }
    }
}
impl Duenio {
    //Metodos Secundarios
    pub fn get_nombre(&self)->&String{
        return &self.nombre;
    }
    pub fn get_direccion(&self)->&String{
        return &self.direccion;
    }
    pub fn get_num_telefono(&self)->u32{
        return self.telefono
    }
    pub fn es_igual_a(&self,d:&Duenio)->bool{
        return (&self.nombre == d.get_nombre())&&(&self.direccion == d.get_direccion())&&(self.telefono == d.get_num_telefono());
    }
    pub fn datos_iguales_a(&self,nom:&String,tel:u32)->bool{
        return (&self.nombre==nom)&&(self.telefono==tel)
    }
    //Metodos Primarios
    pub fn new(nombre_in: &str,direccion_in: &str,telefono_in: u32) -> Duenio {
        return Duenio{
            nombre : nombre_in.to_string(),
            direccion : direccion_in.to_string(),
            telefono : telefono_in
        }
    }
    
}

impl Mascota {
    //Metodos secundarios
    pub fn get_nombre(&self)->&String{
        return &self.nombre;
    }
    pub fn get_edad(&self)->u32{
        return self.edad
    }
    pub fn get_tipo(&self)->&Animales{
        return &self.tipo
    }
    pub fn get_duenio(&self)->&Duenio{
        return &self.duenio
    }
    pub fn es_igual_a(&self,m:&Mascota)->bool{
        return (&self.nombre == m.get_nombre())&&(self.edad == m.get_edad())&&(self.tipo.es_igual_a(&m.get_tipo()))&&(self.duenio.es_igual_a(&m.get_duenio()));
    }
    pub fn datos_iguales_a(&self,nom:&String,nom_duenio:&String,tel:u32)->bool{
        return (&self.nombre==nom)&&(self.duenio.datos_iguales_a(&nom_duenio,tel))
    }
    //Metodos primarios
    pub fn new(nombre_in: &str,edad_in: u32,tipo_in: Animales,duenio_in: Duenio) -> Mascota {
        return Mascota{
            nombre : nombre_in.to_string(),
            edad : edad_in,
            tipo : tipo_in,
            duenio : duenio_in
        }
    }
}

impl Atencion {
    //Metodos secundarios
    pub fn get_mascota(&self)->&Mascota{
        return &self.mascota
    }
    pub fn datos_iguales_a(&self,nom_mascota:&String,nom_duenio:&String,tel:u32)->bool{
        return self.mascota.datos_iguales_a(&nom_mascota,&nom_duenio,tel)
    }
    pub fn es_igual_a(&self,ate:&Atencion)->bool{
        let mut tiene_fecha = false;
        if let Some(fecha1) = &self.proxima_visita{
            if let Some(fecha2) = &ate.proxima_visita{
                tiene_fecha = fecha1.es_igual_a(&fecha2);
            }
        }else{
            if ate.proxima_visita.is_none(){
                tiene_fecha = true;
            }
        }
        return self.mascota.es_igual_a(&ate.get_mascota())&&(self.diagnostico==ate.diagnostico)&&(self.tratamiento==ate.tratamiento)&&(tiene_fecha)
    }
    pub fn cambiar_diagnostico(&mut self,diag:&str){
        self.diagnostico = diag.to_string();
    }
    pub fn cambiar_fecha(&mut self,f:Option<Fecha>){
        self.proxima_visita = f;
    }
    //Metodos primarios
    pub fn new(mascota_in: Mascota,diagnostico_in: &str,tratamiento_in: &str,proxima_visita_in: Option<Fecha>) -> Atencion {
        Atencion{
            mascota : mascota_in,
            diagnostico : diagnostico_in.to_string(),
            tratamiento : tratamiento_in.to_string(),
            proxima_visita : proxima_visita_in
        }
    }
}

impl Veterinaria{
    //Metodos secundarios
    pub fn get_nombre(&self)->&String{
        return &self.nombre
    }
    pub fn get_direccion(&self)->&String{
        return &self.direccion
    }
    pub fn get_id(&self)->u32{
        return self.id
    }
    pub fn es_igual_a(&self,v:&Veterinaria)->bool{
        return (&self.nombre == v.get_nombre())&&(&self.direccion == v.get_direccion())&&(self.id == v.get_id());
    }
    /*
		Nuevos metodos del tp5
	*/
	fn recuperar_informacion(path:&str)-> Result<Vec<Atencion>,Errores>{
		let file = File::open(path).map_err(Errores::ErrorIO)?;
		let atenciones: Vec<Atencion> = serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
		Ok(atenciones)
	}
	fn guardar_informacion(&self) -> Result<(), Errores> {
	    let mut file = File::create(&self.path)?;
	    let serialized = serde_json::to_string(&self.atenciones_realizadas)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(())
    }
    //Metodos primarios
    pub fn new(nom_in:&str,dir_in:&str,id_in:u32,path_in:&str)->Veterinaria{
        let atenciones : Vec<Atencion> = match Veterinaria::recuperar_informacion(path_in){
            Ok(dato) => {
                dato
            }
            Err(_) => {
                Vec::new()
            }
        };
        return Veterinaria{
            nombre : nom_in.to_string(),
            direccion : dir_in.to_string(),
            id : id_in,
            cola_atencion : VecDeque::new(),
            atenciones_realizadas : atenciones,
            path: path_in.to_string()
        }
    }
    pub fn agregar_mascota(&mut self,m:Mascota){
        self.cola_atencion.push_back(m);
    }   
    pub fn priorizar_mascota(&mut self,m:Mascota){
        self.cola_atencion.push_front(m);
    }
    pub fn atender_mascota(&mut self)->Option<Mascota>{
        if self.cola_atencion.is_empty() {
            return None;
        }else{
            return self.cola_atencion.pop_front();
        }
    }
    pub fn eliminar_mascota(&mut self, m:&Mascota){
        if !self.cola_atencion.is_empty(){
            for i in 0..self.cola_atencion.len(){
                if self.cola_atencion[i].es_igual_a(&m) {
                    self.cola_atencion.remove(i);
                    break;
                }
            }
        }
    }
    pub fn registrar_atencion(&mut self,a:Atencion)->Result<(), Errores>{
        self.atenciones_realizadas.push(a);
        self.guardar_informacion()?;
        return Ok(())
    }
    pub fn buscar_atencion(&self,nom_mascota:&String,nom_duenio:&String,tel:u32)->Option<&Atencion>{
        let mut res : Option<&Atencion> = None;
        if !self.atenciones_realizadas.is_empty(){
            for i in (0..self.atenciones_realizadas.len()).rev(){
                if self.atenciones_realizadas[i].datos_iguales_a(&nom_mascota,&nom_duenio, tel){
                    res = Some(&self.atenciones_realizadas[i]);
                    break;
                }
            }
        }
        return res;
    }
    pub fn modificar_diagnostico(&mut self,ate:&Atencion,diag:&String)->Result<(),Errores>{
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas[i].cambiar_diagnostico(&diag.clone());
                    self.guardar_informacion()?;
                    return Ok(())
                }
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre().clone())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())))
    }
    pub fn modificar_fecha(&mut self,ate:&Atencion,fecha: Option<Fecha>)->Result<(),Errores> {
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate) {
                    self.atenciones_realizadas[i].cambiar_fecha(fecha);
                    self.guardar_informacion()?;
                    return Ok(())
                }
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre().clone())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())))
    }
    pub fn eliminar_atencion(&mut self,ate:&Atencion)->Result<(),Errores>{
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas.remove(i);
                    self.guardar_informacion()?;
                    return Ok(())
                }
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre().clone())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())))
    }
}
#[cfg(test)]
mod testing_ejercicio9{
    use super::*;

    #[test]
    fn creacion_veterinaria(){
        let v = Veterinaria::new(&"mordidas",&"av1",1,"./lista_atenciones.json");
        let v2 = Veterinaria::new(&"mordidas",&"av1",1,"./lista_atenciones.json");
        assert_eq!(v.es_igual_a(&v2),true);
    }

    #[test]
    fn operatoria_mascotas(){
        let mut v = Veterinaria::new(&"mordidas",&"av1",1,"./lista_atenciones");
        let d1 = Duenio::new(&"Marcos",&"av2",1234);
        let animal1 = Mascota::new(&"Luchito", 2, Animales::Perro, d1.clone());
        v.agregar_mascota(animal1);

        let animal2 = Mascota::new(&"Piecitos", 1, Animales::Gato, d1.clone());
        v.priorizar_mascota(animal2);

        //Atendiende un gato
        assert!(v.atender_mascota().is_some_and(|ani|{
            ani.es_igual_a(&Mascota::new(&"Piecitos", 1, Animales::Gato, d1.clone()))
        }),"No se encontro al animal");

        //Atendiende un perro
        assert!(v.atender_mascota().is_some_and(|ani|{
            ani.es_igual_a(&Mascota::new(&"Luchito", 2, Animales::Perro, d1.clone()))
        }),"No se encontro al animal");

        //Borra una mascota
        let animal3 = Mascota::new(&"Luchis", 2, Animales::Perro, d1);
        v.agregar_mascota(animal3.clone());
        v.eliminar_mascota(&animal3);
        assert_eq!(v.atender_mascota().is_none(),true);
    }

    #[test]
    fn operar_atenciones(){
        let mut v = Veterinaria::new(&"mordidas",&"av1",1,"./lista_atenciones.json");
        let d1 = Duenio::new(&"Marcos",&"av2",1234);
        let animal1 = Mascota::new(&"Luchito", 2, Animales::Perro, d1.clone());
        let animal2 = Mascota::new(&"Luchon", 2, Animales::Perro, d1.clone());
        v.agregar_mascota(animal1.clone());
        v.agregar_mascota(animal2);

        //Verificacion de estructura vacia en atenciones
        let ate_aux = Atencion::new(animal1,&"Pulgas",&"Pipeta",None);
        assert!(v.modificar_diagnostico(&ate_aux, &"algo".to_string()).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(_)))
        }));
        assert!(v.modificar_fecha(&ate_aux, None) .is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(_)))
        }));
        assert!(v.eliminar_atencion(&ate_aux).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e,Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(_)))
        }));

        //Primera recepcion
        assert!(v.atender_mascota().is_some_and(|ani|{
            let ate1 = Atencion::new(ani,&"Pulgas",&"Pipeta",None);
            v.registrar_atencion(ate1).is_ok()
        }),"No se atendio ningun animal");

        //Segunda recepcion
        assert!(v.atender_mascota().is_some_and(|ani|{
            let ate2 = Atencion::new(ani,&"Garrapatas",&"Pipeta",None);
            v.registrar_atencion(ate2).is_ok()
        }),"No se atendio ningun animal");
        
        //Búsqueda y eliminación de la última atención
        let atencion = v.buscar_atencion(&"Luchon".to_string(), &"Marcos".to_string(), 1234).cloned();
        assert!(atencion.is_some_and(|a| {
            assert!(a.mascota.nombre == "Luchon");
            assert!(v.eliminar_atencion(&a).is_ok());

            assert!(v.modificar_diagnostico(&a, &"algo".to_string()).is_err_and(|e|{
                assert!(!e.to_string().is_empty());
                matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
            }));
            assert!(v.modificar_fecha(&a, None) .is_err_and(|e|{
                assert!(!e.to_string().is_empty());
                matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
            }));
            assert!(v.eliminar_atencion(&a).is_err_and(|e|{
                assert!(!e.to_string().is_empty());
                matches!(e,Errores::ErrorOperatoria(error_operatoria::Inexistente(_)))
            }));
            
            true
        }), "No se encontro tal recepcion");

        //Búsqueda de la primer atención y modificación de diagnóstico
        let atencion = v.buscar_atencion(&"Luchito".to_string(), &"Marcos".to_string(), 1234).cloned();
        assert!(atencion.is_some_and(|a| {
            assert!(a.mascota.get_nombre() == "Luchito");
            v.modificar_diagnostico(&a, &"Vomitos".to_string()).is_ok()
        }), "No se encontro tal recepcion");

        //Búsqueda de atención y modificación de fecha
        let atencion = v.buscar_atencion(&"Luchito".to_string(), &"Marcos".to_string(), 1234).cloned();
        assert!(atencion.is_some_and(|a| {
            assert!(a.mascota.get_nombre() == "Luchito");
            v.modificar_fecha(&a, Some(Fecha::new(5, 5, 2025))).is_ok()
        }), "No se encontro tal recepcion");

        //Busqueda de atencion modificada 
        assert!(v.buscar_atencion(&"Luchito".to_string(),&"Marcos".to_string(),1234).is_some_and(|a|{
            let m = Mascota::new(&"Luchito", 2, Animales::Perro, d1.clone());
            let ate1 = Atencion::new(m,&"Vomitos",&"Pipeta",Some(Fecha::new(5,5,2025)) );
            a.es_igual_a(&ate1)
        }),"No se encontro tal recepcion");

        //Búsqueda y eliminación de la última atención modificada
        let atencion = v.buscar_atencion(&"Luchito".to_string(), &"Marcos".to_string(), 1234).cloned();
        assert!(atencion.is_some_and(|a| {
            assert!(a.mascota.nombre == "Luchito");
            v.eliminar_atencion(&a).is_ok()
        }), "No se encontro tal recepcion");

        
    }

    /*
		Casos especiales para la cobertura de coverage
	*/
	#[test]
	fn caso_especial_error_io() {
		// Se buscara forzar un ErrorIO usando una ruta cuyo directorio base NO EXISTE
		let path_err = "./carpeta_inexistente_123/x.json";

		let mut v = Veterinaria::new(&"mordiscos",&"av1",1,path_err);
        let d1 = Duenio::new(&"Marcos",&"av2",1234);
        let animal1 = Mascota::new(&"Luchito", 2, Animales::Perro, d1);
        v.agregar_mascota(animal1);

        // Al intentar registrar la atencion, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO
        assert!(v.atender_mascota().is_some_and(|ani|{
            let ate1 = Atencion::new(ani,&"Pulgas",&"Pipeta",None);
            v.registrar_atencion(ate1).is_err_and(|e|{
                assert!(!e.to_string().is_empty());
			    matches!(e, Errores::ErrorIO(_))
            })
        }),"Ocurrio un error imprevisto");
		
	}

	#[test]
	fn caso_especial_error_serde() {
		let path_err = "./corrupto.json";
		
		// Se fuerza la escritura en el contenido temporal que NO cumple con el formato estructurado de un .JSON válido
		assert!(std::fs::write(path_err, "{ &&5435#$#$&42365_XXXX1234 : [::: ").is_ok(),"No debio fallar aqui");

		// Se invoca directamente el método para leer el archivo del path que buscara

		assert!(Veterinaria::recuperar_informacion(path_err).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorSerde(_))
		}),"Aquí debió fallar");

		assert!(std::fs::remove_file(path_err).is_ok(),"Error fuera de lo previsto");
		
	}
}