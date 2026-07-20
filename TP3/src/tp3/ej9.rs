/* 
    Estructura Fecha - Ejercicio3
*/

use std::collections::VecDeque;

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
    Estructuras
*/

#[derive(Debug, Clone)]
enum Animales{
    Perro,
    Gato,
    Caballo,
    Otro,
}
#[derive(Debug, Clone)]
struct Duenio {
    nombre: String,
    direccion: String,
    telefono: u32
}
#[derive(Debug, Clone)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animales,
    duenio: Duenio
}
#[derive(Debug, Clone)]
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
    atenciones_realizadas: Vec<Atencion>
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
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn get_direccion(&self)->String{
        return self.direccion.clone();
    }
    pub fn get_num_telefono(&self)->u32{
        return self.telefono
    }
    pub fn es_igual_a(&self,d:&Duenio)->bool{
        return (self.nombre == d.get_nombre())&&(self.direccion == d.get_direccion())&&(self.telefono == d.get_num_telefono());
    }
    pub fn datos_iguales_a(&self,nom:&String,tel:u32)->bool{
        return (self.nombre==*nom)&&(self.telefono==tel)
    }
    //Metodos Primarios
    pub fn new(nombre_in: &String,direccion_in: &String,telefono_in: u32) -> Duenio {
        return Duenio{
            nombre : nombre_in.clone(),
            direccion : direccion_in.clone(),
            telefono : telefono_in
        }
    }
    
}

impl Mascota {
    //Metodos secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn get_edad(&self)->u32{
        return self.edad
    }
    pub fn get_tipo(&self)->Animales{
        return self.tipo.clone()
    }
    pub fn get_duenio(&self)->Duenio{
        return self.duenio.clone()
    }
    pub fn es_igual_a(&self,m:&Mascota)->bool{
        return (self.nombre == m.get_nombre())&&(self.edad == m.get_edad())&&(self.tipo.es_igual_a(&m.get_tipo()))&&(self.duenio.es_igual_a(&m.get_duenio()));
    }
    pub fn datos_iguales_a(&self,nom:&String,nom_duenio:&String,tel:u32)->bool{
        return (self.nombre==*nom)&&(self.duenio.datos_iguales_a(&nom_duenio,tel))
    }
    //Metodos primarios
    pub fn new(nombre_in: &String,edad_in: u32,tipo_in: &Animales,duenio_in: &Duenio) -> Mascota {
        return Mascota{
            nombre : nombre_in.clone(),
            edad : edad_in,
            tipo : tipo_in.clone(),
            duenio : duenio_in.clone()
        }
    }
}

impl Atencion {
    //Metodos secundarios
    pub fn get_mascota(&self)->Mascota{
        return self.mascota.clone()
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
    pub fn cambiar_diagnostico(&mut self,diag:&String){
        self.diagnostico = diag.clone();
    }
    pub fn cambiar_fecha(&mut self,f:&Option<Fecha>){
        self.proxima_visita = f.clone();
    }
    //Metodos primarios
    pub fn new(mascota_in: &Mascota,diagnostico_in: &String,tratamiento_in: &String,proxima_visita_in: &Option<Fecha>) -> Atencion {
        Atencion{
            mascota : mascota_in.clone(),
            diagnostico : diagnostico_in.clone(),
            tratamiento : tratamiento_in.clone(),
            proxima_visita : proxima_visita_in.clone()
        }
    }
}

impl Veterinaria{
    //Metodos secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone()
    }
    pub fn get_direccion(&self)->String{
        return self.direccion.clone()
    }
    pub fn get_id(&self)->u32{
        return self.id
    }
    pub fn es_igual_a(&self,v:&Veterinaria)->bool{
        return (self.nombre == v.get_nombre())&&(self.direccion == v.get_direccion())&&(self.id == v.get_id());
    }
    //Metodos primarios
    pub fn new(nom_in:&String,dir_in:&String,id_in:u32)->Veterinaria{
        return Veterinaria{
            nombre : nom_in.clone(),
            direccion : dir_in.clone(),
            id : id_in,
            cola_atencion : VecDeque::new(),
            atenciones_realizadas : Vec::new()
        }
    }
    pub fn agregar_mascota(&mut self,m:&Mascota){
        self.cola_atencion.push_back(m.clone());
    }   
    pub fn priorizar_mascota(&mut self,m:&Mascota){
        self.cola_atencion.push_front(m.clone());
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
    pub fn registrar_atencion(&mut self,a:&Atencion){
        self.atenciones_realizadas.push(a.clone());
    }
    pub fn buscar_atencion(&self,nom_mascota:&String,nom_duenio:&String,tel:u32)->Option<Atencion>{
        let mut res : Option<Atencion> = None;
        if !self.atenciones_realizadas.is_empty(){
            let mut atenciones = self.atenciones_realizadas.clone();
            atenciones.reverse();
            for ate in atenciones{
                if ate.datos_iguales_a(&nom_mascota,&nom_duenio, tel){
                    res = Some(ate);
                    break;
                }
            }
        }
        return res;
    }
    pub fn modificar_diagnostico(&mut self,ate:&Atencion,diag:&String){
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas[i].cambiar_diagnostico(&diag.clone());
                    break;
                }
            }
        }
    }
    pub fn modificar_fecha(&mut self,ate:&Atencion,fecha: &Option<Fecha>) {
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate) {
                    self.atenciones_realizadas[i].cambiar_fecha(&fecha);
                    break;
                }
            }
        }
    }
    pub fn eliminar_atencion(&mut self,ate:&Atencion){
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas.remove(i);
                    break;
                }
            }
        }
    }
}
#[cfg(test)]
mod testing_ejercicio9{
    use super::*;

    #[test]
    fn creacion_veterinaria(){
        let v = Veterinaria::new(&"mordidas".to_string(),&"av1".to_string(),1);
        let v2 = Veterinaria::new(&"mordidas".to_string(),&"av1".to_string(),1);
        assert_eq!(v.es_igual_a(&v2),true);
    }

    #[test]
    fn operatoria_mascotas(){
        let mut v = Veterinaria::new(&"mordidas".to_string(),&"av1".to_string(),1);
        let d1 = Duenio::new(&"Marcos".to_string(),&"av2".to_string(),1234);
        let animal1 = Mascota::new(&String::from("Luchito"), 2, &Animales::Perro, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal1);

        let animal2 = Mascota::new(&String::from("Piecitos"), 1, &Animales::Gato, &d1);
        v.priorizar_mascota(&animal2);

        //Atendiende un gato
        if let Some(ani) = v.atender_mascota(){
            assert_eq!(ani.es_igual_a(&animal2),true);
        }else{
            panic!("No se encontro el animal");
        }

        //Atendiende un perro
        if let Some(ani) = v.atender_mascota(){
            assert_eq!(ani.es_igual_a(&animal1),true);
        }else{
            panic!("No se encontro el animal");
        }

        //Borra el perro repetido(del anterior)
        v.eliminar_mascota(&animal1);
        assert_eq!(v.atender_mascota().is_none(),true);
    }

    #[test]
    fn operar_atenciones(){
        let mut v = Veterinaria::new(&"mordidas".to_string(),&"av1".to_string(),1);
        let d1 = Duenio::new(&"Marcos".to_string(),&"av2".to_string(),1234);
        let animal1 = Mascota::new(&String::from("Luchito"), 2, &Animales::Perro, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal1);

        let mut ate1 : Atencion;
        let mut ate2 : Atencion;
        //Primera recepcion
        if let Some(ani) = v.atender_mascota(){
            ate1 = Atencion::new(&ani,&"Pulgas".to_string(),&"Pipeta".to_string(),&None);
            v.registrar_atencion(&ate1);
        }else{
            panic!("No se atendio a ningun animal");
        }

        //Segunda recepcion
        if let Some(ani) = v.atender_mascota(){
            ate2 = Atencion::new(&ani,&"Garrapatas".to_string(),&"Pipeta".to_string(),&Some(Fecha::new(5,5,2025)));
            v.registrar_atencion(&ate2);
        }else{
            panic!("No se atendio a ningun animal");
        }
        
        //Busqueda y eliminacion de la ultima atencion
        if let Some(ate_actual) = v.buscar_atencion(&"Luchito".to_string(),&"Marcos".to_string(),1234){
            assert_eq!(ate_actual.es_igual_a(&ate2),true);
            v.eliminar_atencion(&ate2);
        }else{
            panic!("No se encontro tal recepcion");
        }
    
        //Busqueda de la primer atencion(la unica que queda en el registro)
        if let Some(ate_actual) = v.buscar_atencion(&"Luchito".to_string(),&"Marcos".to_string(),1234){
            assert_eq!(ate_actual.es_igual_a(&ate1),true);
            v.modificar_diagnostico(&ate_actual, &"Vomitos".to_string()); //ate1
        }else{
            panic!("No se encontro tal recepcion");
        }


        //Busqueda de atencion y modificacion de fecha
        if let Some(ate_actual) = v.buscar_atencion(&"Luchito".to_string(),&"Marcos".to_string(),1234){
            ate1 = Atencion::new(&animal1,&"Vomitos".to_string(),&"Pipeta".to_string(),&None);
            assert_eq!(ate_actual.es_igual_a(&ate1),true);
            v.modificar_fecha(&ate_actual,&Some(Fecha::new(5,5,2025)));
        }else{
            panic!("No se encontro tal recepcion");
        }

        //Busqueda de atencion modificada 
        if let Some(ate_actual) = v.buscar_atencion(&"Luchito".to_string(),&"Marcos".to_string(),1234){
            ate1 = Atencion::new(&animal1,&"Vomitos".to_string(),&"Pipeta".to_string(),&Some(Fecha::new(5,5,2025)) );
            assert_eq!(ate_actual.es_igual_a(&ate1),true);
        }else{
            panic!("No se encontro tal recepcion");
        }

    }
}