/***
 * 
 *      EXTRACCION DE LA ESTRUCTURA FECHA DEL ejercicio 3
 * 
***/
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
mod testing_ejercicio10_fecha{
    use super::Fecha;

    #[test]
    fn creacion_fecha(){
        let f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 1, 2025)),true);
    }

    #[test]
    fn validacion_de_fecha(){
        //Si las fechas tienen aproximadamente un formato correcto (Respeta el sistema gregoriano)
        let mut f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_fecha_valida(),true);
        f = Fecha::new(31, 2, 2004);
        assert_eq!(f.es_fecha_valida(),false);
        //Si determina cual es la ultima fecha(dia) en el que acaba el mes
        let mut f = Fecha::new(1, 9, 2025);
        assert_eq!(f.ultimo_dia(),30);
        assert!(f.es_fecha_valida());

        let mut f = Fecha::new(1, 4, 2025);
        assert_eq!(f.ultimo_dia(),30);
        assert!(f.es_fecha_valida());

        let mut f = Fecha::new(1, 6, 2025);
        assert_eq!(f.ultimo_dia(),30);
        assert!(f.es_fecha_valida());

        let mut f = Fecha::new(1, 11, 2025);
        assert_eq!(f.ultimo_dia(),30);
        assert!(f.es_fecha_valida());
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
        //Prueba con una fecha 1°
        let mut f = Fecha::new(1, 1, 2028);
        f.sumar_dias(30);
        assert_eq!(f.es_igual_a(&Fecha::new(31, 1, 2028)),true);
        f.sumar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 2, 2028)),true);
        f.sumar_dias(29);
        assert_eq!(f.es_igual_a(&Fecha::new(1,3,2028)),true);
        //Prueba con una fecha 2°
        let mut f = Fecha::new(31, 12, 2024);
        f.sumar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 1, 2025)),true);
    }

    #[test]
    fn sustraccion_fecha(){
        //Prueba con una fecha 1°
        let mut f = Fecha::new(10, 4, 2028);
        f.restar_dias(9);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 4, 2028)),true);
        f.restar_dias(31);
        assert_eq!(f.es_igual_a(&Fecha::new(1,3,2028)),true);
        f.restar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(29, 2, 2028)),true);

        //Prueba con una fecha 2°
        let mut f = Fecha::new(1, 1, 2025);
        f.restar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(31, 12, 2024)),true);
    }

    #[test]
    fn comparacion_fechas(){
        let f1 = Fecha::new(25, 5, 2000);
        let f2 = Fecha::new(25, 2, 2004);
        assert_eq!(f1.es_mayor(&f2),false);
        assert_eq!(f2.es_mayor(&f1),true);
    }

}

/***
 * 
 * 
***/


#[derive(Debug, Clone)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otro,
}

#[derive(Debug, Clone)]
pub enum Estado {
    EnPrestamo,
    Devuelto
}

#[derive(Debug, Clone)]
pub struct Libro { 
    isbn : u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero
}

#[derive(Debug, Clone)]
pub struct LibrosDispone {
    libro: Libro,
    cantidad: u32
}

#[derive(Debug, Clone)]
pub struct Cliente { 
    nombre: String,
    telefono: u32,
    correo: String
}
#[derive(Debug, Clone)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    vencimiento: Fecha,
    estado: Estado,
    devolucion: Option<Fecha>,
}

#[derive(Debug)]
pub struct Biblioteca {
    nombre: String,
    direccion: String,
    disponibles: Vec<LibrosDispone>,
    prestamos: Vec<Prestamo>
}
impl Estado{
    pub fn es_igual_a(&self,e:&Estado)->bool{
        match (self, e) {
            (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            (Estado::Devuelto, Estado::Devuelto) => true,
            _ => false
        }
    }
}

impl Genero{
    pub fn es_igual_a(&self,gen_in:&Genero)->bool{
        match (self, gen_in) {
            (Genero::Novela, Genero::Novela) => true,
            (Genero::Infantil, Genero::Infantil) => true,
            (Genero::Tecnico, Genero::Tecnico) => true,
            (Genero::Otro, Genero::Otro) => true,
            _ => false
        }
    }
}

impl Libro {
    pub fn get_titulo(&self)->String{
        return self.titulo.clone();
    }
    pub fn get_autor(&self)->String{
        return self.autor.clone();
    }
    pub fn es_igual_a(&self,l:&Libro)->bool{
        return (self.isbn == l.isbn)&&(self.titulo == l.get_titulo())&&(self.autor == l.get_autor())&&(self.paginas == l.paginas)&&(self.genero.es_igual_a(&l.genero));
    }
    pub fn new(num : u32 ,ti: String,au: String,pag: u32,gen_in: Genero) -> Libro {
        return Libro{
            isbn : num,
            titulo : ti,
            autor : au,
            paginas : pag,
            genero : gen_in
        }
    }
}

impl Cliente {
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn get_correo(&self)->String{
        return self.correo.clone();
    }
    pub fn es_igual_a(&self,c:&Cliente)->bool{
        return (self.nombre == c.get_nombre())&&(self.telefono == c.telefono)&&(self.correo == c.get_correo());
    }
    pub fn new(nom: String,tel: u32,cor: String) -> Cliente {
        return Cliente{
            nombre : nom,
            telefono : tel,
            correo : cor,
        }
    }
}

impl Prestamo {
    pub fn new(libro: &Libro,cliente: &Cliente,vencimiento: &Fecha) -> Prestamo {
        return Prestamo{
            libro: libro.clone(),
            cliente: cliente.clone(),
            vencimiento: vencimiento.clone(),
            estado:Estado::EnPrestamo,
            devolucion:None
        }
    }
    pub fn es_igual(&self,lib:&Libro,c:&Cliente)->bool{
        return self.libro.es_igual_a(&lib)&&self.cliente.es_igual_a(&c);
    }
}

impl Biblioteca {
    pub fn get_direccion(&self)->String{
        return self.direccion.clone();
    }
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn es_igual_a(&self,b:&Biblioteca)->bool{
        return (self.nombre == b.get_nombre())&&(self.direccion == b.get_direccion());
    }
    pub fn new(nombre: String,direccion: String) -> Biblioteca {
        let disponibles = Vec::new();
        let prestamos =  Vec::new();
        return Biblioteca{
            nombre,
            direccion,
            disponibles,
            prestamos,
        }
    }

    //Agregar libros en biblioteca
    pub fn agregar_libro(&mut self,libro:Libro,cantidad:u32) {
        self.disponibles.push(LibrosDispone{libro,cantidad});
    }

    pub fn copias(&self,libro:&Libro) -> u32 {
        let mut copias = 0;
        for libro_biblo in &self.disponibles {
            if libro_biblo.libro.es_igual_a(&libro) {
                copias = libro_biblo.cantidad;
            }
        }
        return copias
    }

    pub fn decrementar(&mut self,libro:&Libro) {
        for i in 0..self.disponibles.len(){
            if self.disponibles[i].libro.es_igual_a(&libro)&&(self.disponibles[i].cantidad > 0) {
                self.disponibles[i].cantidad -=1;
            }
        }
    }
    pub fn incrementar(&mut self,libro:&Libro) {
        for i in 0..self.disponibles.len(){
            if self.disponibles[i].libro.es_igual_a(&libro) {
                self.disponibles[i].cantidad +=1;
            }
        }
    }
    pub fn prestamos(&self,cliente:&Cliente) -> u32 {
        let mut cantidad = 0;
        for prestamo in &self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente)) && (prestamo.estado.es_igual_a(&Estado::EnPrestamo)) {
                cantidad = cantidad + 1;
            }
        }
        return cantidad;
    }
    
    pub fn prestar(&mut self,cliente:Cliente,libro:&Libro,vencimiento:Fecha) -> bool {
        if (self.copias(&libro)>0) && (self.prestamos(&cliente)<=5) {
            self.prestamos.push(Prestamo::new(&libro.clone(), &cliente, &vencimiento));
            self.decrementar(&libro.clone());
            return true
        } else {
            return false
        }
    }
    
    //Parametro auxiliar de fecha para el calculo de proximidad
    pub fn vencimientos_proximos(&self,f:&Fecha,dias:u32) -> Vec<Prestamo> {
        let mut fecha = f.clone();
        fecha.sumar_dias(dias);
        let mut prestamos: Vec<Prestamo> = Vec::new();
        for prestamo in &self.prestamos {
            if (fecha.es_mayor(&prestamo.vencimiento)) && (prestamo.estado.es_igual_a(&Estado::EnPrestamo)) {
                prestamos.push(prestamo.clone());
            }
        }

        return prestamos
    }

    //Reutiliza el metodo de arriba
    pub fn prestamos_vencidos(&self,f:&Fecha) -> Vec<Prestamo> {
        return self.vencimientos_proximos(&f,0);
    }

    fn buscar(&self,libro:&Libro,cliente:&Cliente) -> Option<Prestamo> {
        let mut res = None;
        for prestamo in &self.prestamos {
            if prestamo.es_igual(&libro, &cliente){
                res = Some(prestamo.clone());
                break;
            }
            
        }
        return res
    }

    fn devolver(&mut self,f:&Fecha,libro:&Libro,cliente:&Cliente) {
        let mut pude = false;
        for prestamo in &mut self.prestamos {
            if prestamo.es_igual(&libro, &cliente) {
                prestamo.estado = Estado::Devuelto;
                //Se utiliza una fecha definida para la prueba
                prestamo.devolucion = Some(f.clone());
                pude = true;
            }
        }
        if pude {self.incrementar(&libro.clone());}
    }
}

#[cfg(test)]
mod testing_ejercicio10 {
    use super::*;

    #[test]
    fn libros_biblioteca(){
        let nombre = String::from("Silencio");
        let direccion = String::from("1 e 2 y 3");
       
        let mut biblioteca = Biblioteca::new(nombre,direccion);
        assert_eq!(biblioteca.es_igual_a(&Biblioteca::new("Silencio".to_string(),"1 e 2 y 3".to_string())),true);

        let libro1 = Libro::new(10, "Libro1".to_string(), "Autor1".to_string(), 50 , Genero::Infantil);
        let libro2 = Libro::new(20, "Libro2".to_string(), "Autor2".to_string(), 50 , Genero::Novela);
        let libro3 = Libro::new(30, "Libro3".to_string(), "Autor3".to_string(), 50 , Genero::Tecnico);
        let libro4 = Libro::new(40, "Libro4".to_string(), "Autor4".to_string(), 50 , Genero::Otro);

        biblioteca.agregar_libro(libro1.clone(),0);
        biblioteca.incrementar(&libro1);
        biblioteca.agregar_libro(libro2.clone(),3);
        biblioteca.decrementar(&libro2);
        biblioteca.agregar_libro(libro3.clone(),3);
        biblioteca.agregar_libro(libro4.clone(),4);

        assert_eq!(biblioteca.copias(&libro1),1);
        assert_eq!(biblioteca.copias(&libro2),2);
    }

    #[test]
    fn operatoria_prestamos() {
        let nombre = String::from("Silencio");
        let direccion = String::from("1 e 2 y 3");
       
        let mut biblioteca = Biblioteca::new(nombre,direccion);
        
        //Clientela
        let cliente1 = Cliente::new("Carlos".to_string(),1,"Carlos.com".to_string());
        let cliente2 = Cliente::new("Mateo".to_string(),2,"Mateo.com".to_string());
        let cliente3 = Cliente::new("Juan".to_string(),3,"Juan.com".to_string());
        let cliente4 = Cliente::new("Pedro".to_string(),4,"Pedro.com".to_string());

        //Libros
        let libro1 = Libro::new(10, "Libro1".to_string(), "Autor1".to_string(), 50 , Genero::Infantil);
        let libro2 = Libro::new(20, "Libro2".to_string(), "Autor2".to_string(), 50 , Genero::Novela);
        let libro3 = Libro::new(30, "Libro3".to_string(), "Autor3".to_string(), 50 , Genero::Tecnico);
        let libro4 = Libro::new(40, "Libro4".to_string(), "Autor4".to_string(), 50 , Genero::Otro);

        biblioteca.agregar_libro(libro1.clone(),5);
        biblioteca.agregar_libro(libro2.clone(),5);
        biblioteca.agregar_libro(libro3.clone(),3);
        biblioteca.agregar_libro(libro4.clone(),4);

        //Operacion de los prestamos
        let mut ayer = Fecha::new(14, 5, 2025);
        let mut quince_dias = ayer.clone();
        quince_dias.sumar_dias(15);
        ayer.restar_dias(1);
        let mut nunca = ayer.clone();
        nunca.sumar_dias(99999);    //Fecha maxima de ejemplo

        biblioteca.prestar(cliente1.clone(), &libro1, quince_dias.clone());
        biblioteca.prestar(cliente2.clone(), &libro2, quince_dias.clone());
        biblioteca.prestar(cliente3.clone(), &libro3, nunca.clone());
        biblioteca.prestar(cliente4.clone(), &libro1, nunca.clone());
        biblioteca.prestar(cliente4.clone(), &libro2, nunca.clone());
        biblioteca.prestar(cliente4.clone(), &libro3, ayer.clone());
        biblioteca.prestar(cliente4.clone(), &libro4, ayer.clone());
        assert_eq!(biblioteca.prestamos(&cliente1),1);
        assert_eq!(biblioteca.prestamos(&cliente4),4);

        //Copias prestadas en total de "libro1"
        assert_eq!(biblioteca.copias(&libro1),3); 

        //Fecha a operar
        let act = Fecha::new(15, 5, 2025);
        
        if let Some(pres) = biblioteca.buscar(&libro1, &cliente1){
            assert_eq!(pres.estado.es_igual_a(&Estado::EnPrestamo), true);
        }else{
            panic!("No se registro el prestamo");
        }

        //Contabiliza los prestamos de ayer y de 15 dias(definido arriba)
        assert_eq!(biblioteca.vencimientos_proximos(&act,20).len(),4);
        
        //Contabiliza los prestamos de ayer
        assert_eq!(biblioteca.prestamos_vencidos(&act).len(),2);

        //Contabiliza las copias de "libro1"
        biblioteca.devolver(&act,&libro1, &cliente1);
        assert_eq!(biblioteca.disponibles[0].cantidad,4);
        
        let prestamo = biblioteca.prestamos[0].clone();
        assert_eq!(prestamo.estado.es_igual_a(&Estado::Devuelto), true);
        
        if let Some(f) = prestamo.devolucion{
            assert_eq!(f.es_igual_a(&act),true);
        }else{
            panic!("No se registro la devolucion");
        }
    }
}
