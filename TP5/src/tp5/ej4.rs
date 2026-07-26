/*
    Implementacion EJ4-TP5
*/
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;
use std::io;
use std::{
    fs::File,
    io::{Error, Read, Write},
};

/*
    Tipos de errores
*/
#[derive(Debug)]
enum ErroresOperatoria {
    Inexistente(String),
    EstructuraVacia(String),
    PrestamoActivo,
    LimitePrestamos,
    SinStock(String),
}

impl Display for ErroresOperatoria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErroresOperatoria::Inexistente(val) => {
                write!(f, "No se encontro el elemento en la estructura {} ", val)
            }
            ErroresOperatoria::EstructuraVacia(val) => {
                write!(f, "La estrucutra {} no dispone de elementos ", val)
            }
            ErroresOperatoria::PrestamoActivo => {
                write!(f, "El usuario tiene un prestamo activo para el libro")
            }
            ErroresOperatoria::LimitePrestamos => write!(
                f,
                "El usuario excedio el limite de prestamos que puede solicitar"
            ),
            ErroresOperatoria::SinStock(val) => {
                write!(f, "No hay mas copias disponibles para el libro {}", val)
            }
        }
    }
}

#[derive(Debug)]
enum Errores {
    ErrorOperatoria(ErroresOperatoria),
    ErrorIO(io::Error),
    ErrorSerde(serde_json::Error),
}

impl Display for Errores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errores::ErrorOperatoria(err) => write!(f, "{}", err),
            Errores::ErrorIO(err) => write!(f, "Error de E/S al guardar: {}", err),
            Errores::ErrorSerde(err) => write!(f, "Error de serialización: {}", err),
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

/***
 *
 *      EXTRACCION DE LA ESTRUCTURA FECHA DEL ejercicio 3 - TP3
 *
***/
//Atributos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fecha {
    pub dia: u8,
    pub mes: u8,
    pub anio: u16,
}

/*
    Metodos
*/

impl Fecha {
    //Metodos Secundarios
    pub fn get_dia(&self) -> u8 {
        return self.dia;
    }
    pub fn get_mes(&self) -> u8 {
        return self.mes;
    }
    pub fn get_anio(&self) -> u16 {
        return self.anio;
    }
    pub fn es_igual_a(&self, f: &Fecha) -> bool {
        return if (self.get_dia() == f.get_dia())
            && (self.get_mes() == f.get_mes())
            && (self.get_anio() == f.get_anio())
        {
            true
        } else {
            false
        };
    }
    /*
       Metodos Primarios
    */
    pub fn new(d: u8, m: u8, a: u16) -> Fecha {
        return Fecha {
            dia: d,
            mes: m,
            anio: a,
        };
    }
    pub fn es_fecha_valida(&self) -> bool {
        if (self.mes > 0) && (self.mes <= 12) && (self.anio > 0) && (self.dia > 0) {
            match self.mes {
                2 => {
                    if self.es_bisiesto() {
                        return self.dia <= 29;
                    } else {
                        return self.dia <= 28;
                    }
                }
                9 | 4 | 6 | 11 => return self.dia <= 30,
                _ => return self.dia <= 31,
            }
        }

        return false;
    }

    pub fn es_bisiesto(&self) -> bool {
        return (self.anio % 4 == 0 && self.anio % 100 != 0) || (self.anio % 400 == 0);
    }

    //Auxiliar para determinar el ultimo dia de un mes
    fn ultimo_dia(&self) -> u8 {
        match self.mes {
            2 => {
                if self.es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            9 | 4 | 6 | 11 => 30,
            _ => 31,
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
    pub fn sumar_dias(&mut self, mut dias_sumar: u32) {
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
    fn retroceder_mes(&mut self) {
        if self.mes == 1 {
            self.mes = 12;
            self.anio -= 1;
        } else {
            self.mes -= 1;
        }
        self.dia = self.ultimo_dia();
    }

    //Se considera que la fecha es valida
    //Y que no se llegara a una fecha negativa(anio negativo)
    pub fn restar_dias(&mut self, mut dias_restar: u32) {
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

    pub fn es_mayor(&self, f: &Fecha) -> bool {
        return if self.anio > f.anio {
            true
        } else if (self.anio == f.anio) && (self.mes > f.mes) {
            true
        } else if (self.mes == f.mes) && (self.dia > f.dia) {
            true
        } else {
            false
        };
    }
}

#[cfg(test)]
mod testing_ejercicio3 {
    use super::Fecha;

    #[test]
    fn creacion_fecha() {
        let f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 1, 2025)), true);
    }

    #[test]
    fn validacion_de_fecha() {
        let mut f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_fecha_valida(), true);
        f = Fecha::new(31, 2, 2004);
        assert_eq!(f.es_fecha_valida(), false);
        f = Fecha::new(32, 2, 2005);
        assert_eq!(f.es_fecha_valida(), false);
    }

    #[test]
    fn validar_bisiesto() {
        let mut f = Fecha::new(1, 1, 2028);
        assert_eq!(f.es_bisiesto(), true);
        f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_bisiesto(), false);
        f = Fecha::new(1, 1, 100);
        assert_eq!(f.es_bisiesto(), false);
        f = Fecha::new(1, 1, 400);
        assert_eq!(f.es_bisiesto(), true);
    }

    #[test]
    fn adicion_fecha() {
        let mut f = Fecha::new(1, 1, 2028);
        f.sumar_dias(30);
        assert_eq!(f.es_igual_a(&Fecha::new(31, 1, 2028)), true);
        f.sumar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 2, 2028)), true);
        f.sumar_dias(29);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 3, 2028)), true);
    }

    #[test]
    fn sustraccion_fecha() {
        let mut f = Fecha::new(10, 4, 2028);
        f.restar_dias(9);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 4, 2028)), true);
        f.restar_dias(31);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 3, 2028)), true);
        f.restar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(29, 2, 2028)), true);
    }

    #[test]
    fn comparacion_fechas() {
        let f1 = Fecha::new(25, 5, 2000);
        let f2 = Fecha::new(25, 2, 2004);
        assert_eq!(f1.es_mayor(&f2), false);
        assert_eq!(f2.es_mayor(&f1), true);
    }
}

/***
 *      Extraccion del ejercicio 10 - TP3
 *
***/

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otro,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Estado {
    EnPrestamo,
    Devuelto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Libro {
    isbn: u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LibrosDispone(Libro, u32);
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cliente {
    nombre: String,
    telefono: u32,
    correo: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    vencimiento: Fecha,
    estado: Estado,
    devolucion: Option<Fecha>,
}

#[derive(Debug)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    disponibles: Vec<LibrosDispone>,
    prestamos: Vec<Prestamo>,
    path_prestamos: String,
    path_libros: String,
}
impl Estado {
    pub fn es_igual_a(&self, e: &Estado) -> bool {
        match (self, e) {
            (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            (Estado::Devuelto, Estado::Devuelto) => true,
            _ => false,
        }
    }
}

impl Genero {
    pub fn es_igual_a(&self, gen_in: &Genero) -> bool {
        match (self, gen_in) {
            (Genero::Novela, Genero::Novela) => true,
            (Genero::Infantil, Genero::Infantil) => true,
            (Genero::Tecnico, Genero::Tecnico) => true,
            (Genero::Otro, Genero::Otro) => true,
            _ => false,
        }
    }
}

impl Libro {
    pub fn get_titulo(&self) -> String {
        return self.titulo.clone();
    }
    pub fn get_autor(&self) -> String {
        return self.autor.clone();
    }
    pub fn es_igual_a(&self, l: &Libro) -> bool {
        return (self.isbn == l.isbn)
            && (self.titulo == l.get_titulo())
            && (self.autor == l.get_autor())
            && (self.paginas == l.paginas)
            && (self.genero.es_igual_a(&l.genero));
    }
    pub fn new(num: u32, ti: String, au: String, pag: u32, gen_in: Genero) -> Libro {
        return Libro {
            isbn: num,
            titulo: ti,
            autor: au,
            paginas: pag,
            genero: gen_in,
        };
    }
}

impl Cliente {
    pub fn get_nombre(&self) -> String {
        return self.nombre.clone();
    }
    pub fn get_correo(&self) -> String {
        return self.correo.clone();
    }
    pub fn es_igual_a(&self, c: &Cliente) -> bool {
        return (self.nombre == c.get_nombre())
            && (self.telefono == c.telefono)
            && (self.correo == c.get_correo());
    }
    pub fn new(nom: String, tel: u32, cor: String) -> Cliente {
        return Cliente {
            nombre: nom,
            telefono: tel,
            correo: cor,
        };
    }
}

impl Prestamo {
    pub fn new(libro: &Libro, cliente: &Cliente, vencimiento: &Fecha) -> Prestamo {
        return Prestamo {
            libro: libro.clone(),
            cliente: cliente.clone(),
            vencimiento: vencimiento.clone(),
            estado: Estado::EnPrestamo,
            devolucion: None,
        };
    }
    pub fn es_igual(&self, lib: &Libro, c: &Cliente) -> bool {
        return self.libro.es_igual_a(&lib) && self.cliente.es_igual_a(&c);
    }
}

impl Biblioteca {
    pub fn get_direccion(&self) -> String {
        return self.direccion.clone();
    }
    pub fn get_nombre(&self) -> String {
        return self.nombre.clone();
    }
    pub fn es_igual_a(&self, b: &Biblioteca) -> bool {
        return (self.nombre == b.get_nombre()) && (self.direccion == b.get_direccion());
    }
    pub fn new(nombre: String, direccion: String, ruta_l: &str, ruta_p: &str) -> Biblioteca {
        let libros: Vec<LibrosDispone> = match Biblioteca::recuperar_info_libros(ruta_l) {
            Ok(dato) => dato,
            Err(_) => Vec::new(),
        };
        let prestamos_lista: Vec<Prestamo> = match Biblioteca::recuperar_info_prestamos(ruta_p) {
            Ok(dato) => dato,
            Err(_) => Vec::new(),
        };
        return Biblioteca {
            nombre,
            direccion,
            disponibles: libros,
            prestamos: prestamos_lista,
            path_libros: ruta_l.to_string(),
            path_prestamos: ruta_p.to_string(),
        };
    }
    /*
        Nuevos metodos del tp5
    */
    fn recuperar_info_prestamos(path: &str) -> Result<Vec<Prestamo>, Errores> {
        let file = File::open(path).map_err(Errores::ErrorIO)?;
        let prestamos: Vec<Prestamo> =
            serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
        Ok(prestamos)
    }
    fn guardar_info_prestamos(&self) -> Result<(), Errores> {
        let mut file = File::create(&self.path_prestamos)?;
        let serialized = serde_json::to_string(&self.prestamos)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(());
    }
    fn recuperar_info_libros(path: &str) -> Result<Vec<LibrosDispone>, Errores> {
        let file = File::open(path).map_err(Errores::ErrorIO)?;
        let libros: Vec<LibrosDispone> =
            serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
        Ok(libros)
    }
    fn guardar_info_libros(&self) -> Result<(), Errores> {
        let mut file = File::create(&self.path_libros)?;
        let serialized = serde_json::to_string(&self.disponibles)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(());
    }

    /*
        Metodos primarios
    */
    //Agregar libros en biblioteca
    pub fn agregar_libro(&mut self, libro: Libro, cantidad: u32) -> Result<(), Errores> {
        let mut registrado = false;

        for libro_disponible in &mut self.disponibles {
            if libro_disponible.0.es_igual_a(&libro) {
                libro_disponible.1 += cantidad;
                registrado = true;
                break;
            }
        }
        // Sea si el Vec esta vacío o el libro no se encuentra en el catálogo , se lo agrega como nuevo
        if !registrado {
            self.disponibles.push(LibrosDispone(libro, cantidad));
        }

        self.guardar_info_libros()?;
        return Ok(());
    }

    pub fn copias(&self, libro: &Libro) -> u32 {
        let mut copias = 0;
        for libro_biblo in &self.disponibles {
            if libro_biblo.0.es_igual_a(&libro) {
                copias = libro_biblo.1;
                break;
            }
        }
        return copias;
    }

    pub fn decrementar(&mut self, libro: &Libro) -> Result<(), Errores> {
        if !self.disponibles.is_empty() {
            for libros in &mut self.disponibles {
                if libros.0.es_igual_a(&libro) && (libros.1 > 0) {
                    libros.1 -= 1;
                    self.guardar_info_libros()?;
                    return Ok(());
                }
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
                self.get_nombre(),
            )));
        }
        return Err(Errores::ErrorOperatoria(
            ErroresOperatoria::EstructuraVacia(self.get_nombre()),
        ));
    }
    pub fn incrementar(&mut self, libro: &Libro) -> Result<(), Errores> {
        if !self.disponibles.is_empty() {
            for libros in &mut self.disponibles {
                if libros.0.es_igual_a(&libro) {
                    libros.1 += 1;
                    self.guardar_info_libros()?;
                    return Ok(());
                }
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
                self.get_nombre(),
            )));
        }
        return Err(Errores::ErrorOperatoria(
            ErroresOperatoria::EstructuraVacia(self.get_nombre()),
        ));
    }
    pub fn prestamos(&self, cliente: &Cliente) -> u32 {
        let mut cantidad = 0;
        for prestamo in &self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente))
                && (prestamo.estado.es_igual_a(&Estado::EnPrestamo))
            {
                cantidad = cantidad + 1;
            }
        }
        return cantidad;
    }

    //Auxiliar de existencia de prestamo del mismo libro solicitado , para un cliente
    fn tiene_prestamo_del_libro(&self, cliente: &Cliente, libro: &Libro) -> bool {
        let mut existe = false;
        for prestamo in &self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente))
                && (prestamo.estado.es_igual_a(&Estado::EnPrestamo))
                && (prestamo.libro.es_igual_a(&libro))
            {
                existe = true;
                break;
            }
        }

        return existe;
    }

    pub fn prestar(
        &mut self,
        cliente: Cliente,
        libro: &Libro,
        vencimiento: Fecha,
    ) -> Result<(), Errores> {
        if self.copias(&libro) > 0 {
            if self.prestamos(&cliente) < 5 {
                if !self.tiene_prestamo_del_libro(&cliente, &libro) {
                    self.prestamos
                        .push(Prestamo::new(&libro.clone(), &cliente, &vencimiento));
                    self.decrementar(&libro.clone())?;
                    self.guardar_info_prestamos()?;
                    return Ok(());
                }
                return Err(Errores::ErrorOperatoria(ErroresOperatoria::PrestamoActivo));
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::LimitePrestamos));
        }
        return Err(Errores::ErrorOperatoria(ErroresOperatoria::SinStock(
            libro.get_titulo(),
        )));
    }

    //Parametro auxiliar de fecha para el calculo de proximidad
    //Se filtran solo los que esten dentro del lapso inicial hasta al limite indicado en los dias
    pub fn vencimientos_proximos(&self, fecha_act: &Fecha, dias: u32) -> Vec<&Prestamo> {
        let mut fecha_lim = fecha_act.clone();
        fecha_lim.sumar_dias(dias);
        let mut prestamos: Vec<&Prestamo> = Vec::new();
        for prestamo in &self.prestamos {
            if (prestamo.estado.es_igual_a(&Estado::EnPrestamo))
                && (!fecha_act.es_mayor(&prestamo.vencimiento))
                && (!prestamo.vencimiento.es_mayor(&fecha_lim))
            {
                prestamos.push(&prestamo);
            }
        }

        return prestamos;
    }

    //Solo apunta como vencidos a los prestamos = (vencimiento < actual) equivalente (actual > vencimiento)
    pub fn prestamos_vencidos(&self, f: &Fecha) -> Vec<&Prestamo> {
        let mut fecha_actual = f.clone();
        let mut prestamos: Vec<&Prestamo> = Vec::new();
        for prestamo in &self.prestamos {
            if (prestamo.estado.es_igual_a(&Estado::EnPrestamo))
                && (fecha_actual.es_mayor(&prestamo.vencimiento))
            {
                prestamos.push(prestamo);
            }
        }

        return prestamos;
    }

    //La busqueda debe retornar el prestamo "mas reciente" , por ello
    //se recorre a la inversa nuestro registro de prestamos y solo retornar el ultimo registrado del cliente
    fn buscar(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        let mut res = None;
        if !self.prestamos.is_empty() {
            for i in (0..self.prestamos.len()).rev() {
                if self.prestamos[i].es_igual(&libro, &cliente) {
                    res = Some(&self.prestamos[i]);
                    break;
                }
            }
        }

        return res;
    }

    fn devolver(
        &mut self,
        fecha_devolucion: Fecha,
        libro: &Libro,
        cliente: &Cliente,
    ) -> Result<(), Errores> {
        if !self.prestamos.is_empty() {
            for prestamo in &mut self.prestamos {
                if prestamo.es_igual(&libro, &cliente)
                    && prestamo.estado.es_igual_a(&Estado::EnPrestamo)
                {
                    prestamo.estado = Estado::Devuelto;
                    prestamo.devolucion = Some(fecha_devolucion);
                    //Ya se guarda aqui
                    self.incrementar(&libro.clone())?;
                    return Ok(());
                }
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
                self.get_nombre(),
            )));
        }
        return Err(Errores::ErrorOperatoria(
            ErroresOperatoria::EstructuraVacia(self.get_nombre()),
        ));
    }
}

#[cfg(test)]
mod testing_ejercicio10 {
    use super::*;

    #[test]
    fn stock_basico_biblioteca() {
        let nombre = String::from("Silencio");
        let direccion = String::from("1 e 2 y 3");

        let mut biblioteca = Biblioteca::new(
            nombre,
            direccion,
            "./lista_libros1.json",
            "./lista_prestamo1.json",
        );
        assert_eq!(
            biblioteca.es_igual_a(&Biblioteca::new(
                "Silencio".to_string(),
                "1 e 2 y 3".to_string(),
                "./lista_libros.json1",
                "./lista_prestamo1.json"
            )),
            true
        );

        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let libro2 = Libro::new(
            20,
            "Libro2".to_string(),
            "Autor2".to_string(),
            50,
            Genero::Novela,
        );
        let libro3 = Libro::new(
            30,
            "Libro3".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro4 = Libro::new(
            40,
            "Libro4".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );

        //Estructura vacia
        assert!(biblioteca.incrementar(&libro1).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::EstructuraVacia(_))
            )
        }));
        assert!(biblioteca.decrementar(&libro2).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::EstructuraVacia(_))
            )
        }));

        //Agrego libro1
        assert!(biblioteca.agregar_libro(libro1.clone(), 0).is_ok());
        assert!(biblioteca.incrementar(&libro1).is_ok());

        //Inexistencia de libro2
        assert!(biblioteca.decrementar(&libro2).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(_))
            )
        }));
        assert!(biblioteca.incrementar(&libro2).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(_))
            )
        }));

        //Fuera de stock del libro1
        assert!(biblioteca.decrementar(&libro1).is_ok());
        assert!(biblioteca.decrementar(&libro1).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(_))
            )
        }));

        assert!(
            biblioteca.agregar_libro(libro1.clone(), 3).is_ok(),
            "No debio fallar porque el libro ya estaba registrado para reponer stock"
        );
        assert!(biblioteca.agregar_libro(libro2.clone(), 3).is_ok());
        assert!(biblioteca.decrementar(&libro2).is_ok());
        assert!(biblioteca.agregar_libro(libro3.clone(), 3).is_ok());
        assert!(biblioteca.agregar_libro(libro4.clone(), 4).is_ok());

        assert_eq!(biblioteca.copias(&libro1), 3);
        assert_eq!(biblioteca.copias(&libro2), 2);
        assert_eq!(biblioteca.copias(&libro3), 3);
        assert_eq!(biblioteca.copias(&libro4), 4);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_libros1.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn nuevos_prestamos() {
        let mut biblioteca = Biblioteca::new(
            String::from("Sabio"),
            String::from("Av1"),
            "./lista_libros2.json",
            "./lista_prestamos2.json",
        );

        //Libros
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let libro2 = Libro::new(
            20,
            "Libro2".to_string(),
            "Autor2".to_string(),
            50,
            Genero::Novela,
        );

        assert!(biblioteca.agregar_libro(libro1.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro2.clone(), 5).is_ok());

        let cliente1 = Cliente::new("Carlos".to_string(), 1, "Carlos.com".to_string());
        let cliente2 = Cliente::new("Mateo".to_string(), 2, "Mateo.com".to_string());

        //Prestamos
        let mut quince_dias = Fecha::new(15, 5, 2026);

        assert!(
            biblioteca
                .prestar(cliente1.clone(), &libro1, quince_dias.clone())
                .is_ok(),
            "No se pudo hacer el prestamo"
        );
        assert!(
            biblioteca
                .prestar(cliente2.clone(), &libro2, quince_dias.clone())
                .is_ok(),
            "No se pudo hacer el prestamo"
        );

        assert_eq!(biblioteca.prestamos(&cliente1), 1);
        assert_eq!(biblioteca.prestamos(&cliente2), 1);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_libros2.json").is_ok(),
            "Error fuera de lo previsto"
        );
        assert!(
            std::fs::remove_file("./lista_prestamos2.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn prestamos_restringidos() {
        let mut biblioteca = Biblioteca::new(
            String::from("Sabioso"),
            String::from("Av12"),
            "./lista_libros3.json",
            "./lista_prestamos3.json",
        );

        //Libros
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let libro2 = Libro::new(
            20,
            "Libro2".to_string(),
            "Autor2".to_string(),
            50,
            Genero::Novela,
        );
        let libro3 = Libro::new(
            30,
            "Libro3".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro4 = Libro::new(
            40,
            "Libro4".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );
        let libro5 = Libro::new(
            30,
            "Libro5".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro6 = Libro::new(
            40,
            "Libro6".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );

        let cliente3 = Cliente::new("Juan".to_string(), 3, "Juan.com".to_string());
        let cliente4 = Cliente::new("Pedro".to_string(), 4, "Pedro.com".to_string());

        //Sin libros para prestar
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro1, Fecha::new(10, 05, 2026))
                .is_err_and(|e| {
                    assert!(!e.to_string().is_empty());
                    matches!(e, Errores::ErrorOperatoria(ErroresOperatoria::SinStock(_)))
                })
        );

        assert!(biblioteca.agregar_libro(libro1.clone(), 1).is_ok());
        assert!(biblioteca.agregar_libro(libro2.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro3.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro4.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro5.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro6.clone(), 5).is_ok());

        //Prestamos
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro1, Fecha::new(10, 05, 2026))
                .is_ok(),
            "No se pudo hacer el prestamo"
        );
        //Restriccion por stock insuficiente
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro1, Fecha::new(10, 05, 2026))
                .is_err_and(|e| {
                    assert!(!e.to_string().is_empty());
                    matches!(e, Errores::ErrorOperatoria(ErroresOperatoria::SinStock(_)))
                })
        );

        assert!(biblioteca.incrementar(&libro1).is_ok());
        //Restriccion del prestamo para el mismo libro pendiente
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro1, Fecha::new(10, 05, 2026))
                .is_err_and(|e| {
                    assert!(!e.to_string().is_empty());
                    matches!(
                        e,
                        Errores::ErrorOperatoria(ErroresOperatoria::PrestamoActivo)
                    )
                })
        );

        //Restriccion por maximo de prestamos otorgados
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro2, Fecha::new(10, 05, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro3, Fecha::new(10, 05, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro4, Fecha::new(10, 05, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro5, Fecha::new(10, 05, 2026))
                .is_ok()
        );

        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro6, Fecha::new(10, 05, 2026))
                .is_err_and(|e| {
                    assert!(!e.to_string().is_empty());
                    matches!(
                        e,
                        Errores::ErrorOperatoria(ErroresOperatoria::LimitePrestamos)
                    )
                })
        );

        assert_eq!(biblioteca.prestamos(&cliente3), 5);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_libros3.json").is_ok(),
            "Error fuera de lo previsto"
        );
        assert!(
            std::fs::remove_file("./lista_prestamos3.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn prestamos_proximos_a_vencer() {
        let mut biblioteca = Biblioteca::new(
            String::from("Sabioso"),
            String::from("Av12"),
            "./lista_libros4.json",
            "./lista_prestamos4.json",
        );

        //Libros
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let libro2 = Libro::new(
            20,
            "Libro2".to_string(),
            "Autor2".to_string(),
            50,
            Genero::Novela,
        );
        let libro3 = Libro::new(
            30,
            "Libro3".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro4 = Libro::new(
            40,
            "Libro4".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );
        let libro5 = Libro::new(
            30,
            "Libro5".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro6 = Libro::new(
            40,
            "Libro6".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );

        assert!(biblioteca.agregar_libro(libro1.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro2.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro3.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro4.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro5.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro6.clone(), 5).is_ok());

        let cliente3 = Cliente::new("Juan".to_string(), 3, "Juan.com".to_string());
        let cliente4 = Cliente::new("Pedro".to_string(), 4, "Pedro.com".to_string());

        //Fechas de referencia actual
        let mut actual = Fecha::new(1, 5, 2026);

        //Sin prestamos hechos
        assert!(biblioteca.vencimientos_proximos(&actual, 19).is_empty());

        //Lapso que evalua (1/5/26 <= vencimiento <= 20/05/26)
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro1, Fecha::new(12, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro1, Fecha::new(1, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro2, Fecha::new(25, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro6, Fecha::new(2, 6, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro3, Fecha::new(20, 5, 2026))
                .is_ok()
        );

        assert_eq!(biblioteca.vencimientos_proximos(&actual, 19).len(), 3);

        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro1, &cliente3)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro2, &cliente3)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro1, &cliente4)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro6, &cliente4)
                .is_ok()
        );

        assert_eq!(biblioteca.vencimientos_proximos(&actual, 19).len(), 1);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_libros4.json").is_ok(),
            "Error fuera de lo previsto"
        );
        assert!(
            std::fs::remove_file("./lista_prestamos4.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn prestamos_vencidos() {
        let mut biblioteca = Biblioteca::new(
            String::from("Sabioso"),
            String::from("Av12"),
            "./lista_libros5.json",
            "./lista_prestamos5.json",
        );

        //Libros
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let libro2 = Libro::new(
            20,
            "Libro2".to_string(),
            "Autor2".to_string(),
            50,
            Genero::Novela,
        );
        let libro3 = Libro::new(
            30,
            "Libro3".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro4 = Libro::new(
            40,
            "Libro4".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );
        let libro5 = Libro::new(
            30,
            "Libro5".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro6 = Libro::new(
            40,
            "Libro6".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );

        assert!(biblioteca.agregar_libro(libro1.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro2.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro3.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro4.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro5.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro6.clone(), 5).is_ok());

        let cliente3 = Cliente::new("Juan".to_string(), 3, "Juan.com".to_string());
        let cliente4 = Cliente::new("Pedro".to_string(), 4, "Pedro.com".to_string());

        //Fechas de referencia actual
        let mut actual = Fecha::new(1, 5, 2026);

        //Sin prestamos hechos
        assert!(biblioteca.prestamos_vencidos(&actual).is_empty());

        //Lapso que evalua (vencimiento < 1/5/26)
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro1, Fecha::new(12, 4, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro1, Fecha::new(2, 4, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro2, Fecha::new(25, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro6, Fecha::new(1, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro3, Fecha::new(20, 5, 2026))
                .is_ok()
        );

        assert_eq!(biblioteca.prestamos_vencidos(&actual).len(), 2);

        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro1, &cliente3)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro6, &cliente4)
                .is_ok()
        );

        assert_eq!(biblioteca.prestamos_vencidos(&actual).len(), 1);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_libros5.json").is_ok(),
            "Error fuera de lo previsto"
        );
        assert!(
            std::fs::remove_file("./lista_prestamos5.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn busqueda_de_prestamos() {
        let mut biblioteca = Biblioteca::new(
            String::from("Sabioso"),
            String::from("Av12"),
            "./lista_libros6.json",
            "./lista_prestamos6.json",
        );

        //Libros
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let libro2 = Libro::new(
            20,
            "Libro2".to_string(),
            "Autor2".to_string(),
            50,
            Genero::Novela,
        );
        let libro3 = Libro::new(
            30,
            "Libro3".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro4 = Libro::new(
            40,
            "Libro4".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );
        let libro5 = Libro::new(
            30,
            "Libro5".to_string(),
            "Autor3".to_string(),
            50,
            Genero::Tecnico,
        );
        let libro6 = Libro::new(
            40,
            "Libro6".to_string(),
            "Autor4".to_string(),
            50,
            Genero::Otro,
        );

        let cliente2 = Cliente::new("Juanito".to_string(), 2, "Juanito.com".to_string());
        let cliente3 = Cliente::new("Juan".to_string(), 3, "Juan.com".to_string());
        let cliente4 = Cliente::new("Pedro".to_string(), 4, "Pedro.com".to_string());

        assert!(biblioteca.agregar_libro(libro1.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro2.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro3.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro4.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro5.clone(), 5).is_ok());
        assert!(biblioteca.agregar_libro(libro6.clone(), 5).is_ok());

        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro1, &cliente3)
                .is_err_and(|e| {
                    assert!(!e.to_string().is_empty());
                    matches!(
                        e,
                        Errores::ErrorOperatoria(ErroresOperatoria::EstructuraVacia(_))
                    )
                })
        );

        //Fechas de referencia
        //let actual = Fecha::new(1,5,2026);

        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro1, Fecha::new(12, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro1, Fecha::new(1, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro2, Fecha::new(25, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente4.clone(), &libro6, Fecha::new(2, 6, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente3.clone(), &libro3, Fecha::new(20, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente2.clone(), &libro4, Fecha::new(20, 5, 2026))
                .is_ok()
        );
        assert!(
            biblioteca
                .prestar(cliente2.clone(), &libro6, Fecha::new(20, 5, 2026))
                .is_ok()
        );

        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro1, &cliente3)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro3, &cliente3)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro1, &cliente4)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro6, &cliente4)
                .is_ok()
        );
        assert!(
            biblioteca
                .devolver(Fecha::new(1, 5, 2026), &libro6, &cliente2)
                .is_ok()
        );

        //Un prestamo devuelto
        assert!(
            biblioteca
                .buscar(&libro6, &cliente2)
                .is_some_and(|p| p.estado.es_igual_a(&Estado::Devuelto)),
            "El prestamo no se devolvio anteriormente o no existe"
        );

        //Un prestamo pendiente
        assert!(
            biblioteca
                .buscar(&libro2, &cliente3)
                .is_some_and(|p| p.estado.es_igual_a(&Estado::EnPrestamo)),
            "El prestamo tendria que estar pendiente o existir"
        );

        //Un prestamo inexistente
        assert!(
            biblioteca.buscar(&libro1, &cliente2).is_none(),
            "No deberia existir el prestamo en el registro"
        );

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_libros6.json").is_ok(),
            "Error fuera de lo previsto"
        );
        assert!(
            std::fs::remove_file("./lista_prestamos6.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    /*
        Casos especiales para la cobertura de coverage
    */
    #[test]
    fn caso_especial_error_io_libros() {
        // Se buscara forzar un ErrorIO usando una ruta cuyo directorio base NO EXISTE
        let path_err = "./carpeta_inexistente_123/x.json";

        let mut biblioteca = Biblioteca::new(
            String::from("Karateca"),
            String::from("Av12"),
            path_err,
            "./cosa/equisde.json",
        );
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );

        // Al intentar agregar un libro, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO

        assert!(
            biblioteca.agregar_libro(libro1, 1).is_err_and(|e| {
                assert!(!e.to_string().is_empty());
                matches!(e, Errores::ErrorIO(_))
            }),
            "Ocurrio un error imprevisto"
        );
    }

    #[test]
    fn caso_especial_error_io_prestamos() {
        // Se buscara forzar un ErrorIO usando una ruta cuyo directorio base NO EXISTE
        let path_err = "./carpeta_inexistente_123/x.json";

        let mut biblioteca = Biblioteca::new(
            String::from("Karateca"),
            String::from("Av12"),
            "./equisde.json",
            path_err,
        );
        let libro1 = Libro::new(
            10,
            "Libro1".to_string(),
            "Autor1".to_string(),
            50,
            Genero::Infantil,
        );
        let cliente2 = Cliente::new("Juanito".to_string(), 2, "Juanito.com".to_string());
        // Al intentar hacer un prestamo, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO

        assert!(biblioteca.agregar_libro(libro1.clone(), 1).is_ok());
        assert!(
            biblioteca
                .prestar(cliente2, &libro1, Fecha::new(4, 4, 2024))
                .is_err_and(|e| {
                    assert!(!e.to_string().is_empty());
                    matches!(e, Errores::ErrorIO(_))
                }),
            "Ocurrio un error imprevisto"
        );
        assert!(
            std::fs::remove_file("./equisde.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn caso_especial_error_serde() {
        let path_err = "./corrupto.json";

        // Se fuerza la escritura en el contenido temporal que NO cumple con el formato estructurado de un .JSON válido
        assert!(
            std::fs::write(path_err, "{ &&5435#$#$&42365_XXXX1234 : [::: ").is_ok(),
            "No debio fallar aqui"
        );

        // Se invoca directamente el método para leer el archivo del path que buscara

        assert!(
            Biblioteca::recuperar_info_libros(path_err).is_err_and(|e| {
                assert!(!e.to_string().is_empty());
                matches!(e, Errores::ErrorSerde(_))
            }),
            "Aquí debió fallar"
        );

        assert!(
            Biblioteca::recuperar_info_prestamos(path_err).is_err_and(|e| {
                assert!(!e.to_string().is_empty());
                matches!(e, Errores::ErrorSerde(_))
            }),
            "Aquí debió fallar"
        );

        assert!(
            std::fs::remove_file(path_err).is_ok(),
            "Error fuera de lo previsto"
        );
    }
}
