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
    ContratoActivo(String),
    Rechazado(String),
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
            ErroresOperatoria::ContratoActivo(val) => write!(
                f,
                "El usuario {} ya tiene un contrato activo y no puede emplear otro nuevo",
                val
            ),
            ErroresOperatoria::Rechazado(val) => {
                write!(f, "La operacion a sido rechazada por {}", val)
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

/*
    TP3 - Ej3 - Fecha
*/
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

/*
    Extraccion Ejercicio 3 - TP4
    Estructuras secundarias
*/

use core::hash;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
struct InfoMercadoPago {
    alias: String,
    cuil: u128,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
struct InfoTransferencia {
    cbu: u128,
    banco: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
struct InfoTarjeta {
    numero_tarjeta: u128,
    franquicia: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
struct InfoCripto {
    wallet_address: String,
    red: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
enum MediosDePago {
    Efectivo,
    MercadoPago(InfoMercadoPago),
    TransferenciaBancaria(InfoTransferencia),
    TarjetaDeCredito(InfoTarjeta),
    Criptomoneda(InfoCripto),
}

/*
    Estructuras primarias : Usuario y suscripcion
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContratoSuscripcion {
    //Referencia al usuario
    dni_usuario: u64,
    //Datos de contrato para la misma
    tipo_suscripcion: TipoSuscripcion,
    activo: bool,
    costo_mensual: f64,
    duracion_mes: u8,
    fecha_inicio: Fecha,
    tipo_pago: MediosDePago,
}

#[derive(PartialEq, Debug, Clone)]
struct Usuario {
    nombre: String,
    dni: u64,
}

/*
    Estructura plataforma
*/

struct Plataforma {
    usuarios: Vec<Usuario>,
    registro_suscripciones: Vec<ContratoSuscripcion>,
    path: String,
}

impl Usuario {
    pub fn new(nom: &str, dni_in: u64) -> Usuario {
        return Usuario {
            nombre: nom.to_string(),
            dni: dni_in,
        };
    }
    pub fn get_dni(&self) -> u64 {
        return self.dni;
    }
}

impl ContratoSuscripcion {
    pub fn new(
        dni: u64,
        tipo: &TipoSuscripcion,
        costo: f64,
        cant: u8,
        fecha: Fecha,
        medio: &MediosDePago,
    ) -> ContratoSuscripcion {
        return ContratoSuscripcion {
            dni_usuario: dni,
            tipo_suscripcion: tipo.clone(),
            activo: true,
            costo_mensual: costo,
            duracion_mes: cant,
            fecha_inicio: fecha,
            tipo_pago: medio.clone(),
        };
    }
    pub fn cancelar_suscripcion(&mut self) {
        self.activo = false;
    }
    pub fn upgrade_tipo(&mut self) -> bool {
        let mut exito = true;
        match self.tipo_suscripcion {
            TipoSuscripcion::Basic => self.tipo_suscripcion = TipoSuscripcion::Clasic,
            TipoSuscripcion::Clasic => self.tipo_suscripcion = TipoSuscripcion::Super,
            TipoSuscripcion::Super => exito = false,
            _ => exito = false,
        }

        return exito;
    }
    pub fn downgrade_tipo(&mut self) -> bool {
        let mut exito = true;

        match self.tipo_suscripcion {
            TipoSuscripcion::Super => self.tipo_suscripcion = TipoSuscripcion::Clasic,
            TipoSuscripcion::Clasic => self.tipo_suscripcion = TipoSuscripcion::Basic,
            TipoSuscripcion::Basic => exito = false,
            _ => exito = false,
        }

        return exito;
    }
    pub fn dni_igual(&self, dni: u64) -> bool {
        return self.dni_usuario == dni;
    }
}

impl Plataforma {
    pub fn new(path_in: &str) -> Plataforma {
        let suscripciones: Vec<ContratoSuscripcion> =
            match Plataforma::recuperar_informacion(path_in) {
                Ok(datos) => datos,
                Err(_) => Vec::new(),
            };
        return Plataforma {
            usuarios: Vec::new(),
            registro_suscripciones: suscripciones,
            path: path_in.to_string(),
        };
    }
    /*
        Nueva implementacion - TP5
    */
    fn recuperar_informacion(path: &str) -> Result<Vec<ContratoSuscripcion>, Errores> {
        let file = File::open(path).map_err(Errores::ErrorIO)?;
        let suscripciones: Vec<ContratoSuscripcion> =
            serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
        Ok(suscripciones)
    }
    fn guardar_informacion(&self) -> Result<(), Errores> {
        let mut file = File::create(&self.path)?;
        let serialized = serde_json::to_string(&self.registro_suscripciones)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(());
    }
    /*
        Metodos primarios
    */
    //Los dni son unicos
    fn usuario_en_sistema(&self, user_dni: u64) -> bool {
        return self.usuarios.iter().any(|user| user.get_dni() == user_dni);
    }
    pub fn registrar_usuario(&mut self, u: &Usuario) -> bool {
        if !self.usuario_en_sistema(u.get_dni()) {
            self.usuarios.push(u.clone());
        } else {
            return false;
        }
        return true;
    }
    pub fn registrar_contrato(&mut self, c: &ContratoSuscripcion) -> Result<(), Errores> {
        if self.usuario_en_sistema(c.dni_usuario) {
            if !self
                .registro_suscripciones
                .iter()
                .any(|s| s.dni_igual(c.dni_usuario) && s.activo)
            {
                self.registro_suscripciones.push(c.clone());
                self.guardar_informacion()?;
                return Ok(());
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::ContratoActivo(
                c.dni_usuario.to_string(),
            )));
        }
        return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
            String::from("Registro de usuarios"),
        )));
    }
    pub fn upgrade(&mut self, u: &Usuario) -> Result<(), Errores> {
        if self.usuario_en_sistema(u.get_dni()) {
            if let Some(sus) = self
                .registro_suscripciones
                .iter_mut()
                .rev()
                .find(|s| s.dni_igual(u.get_dni()) && s.activo)
            {
                let mut sus_nuevo = sus.clone();
                if sus_nuevo.upgrade_tipo() {
                    sus.cancelar_suscripcion();
                    self.registro_suscripciones.push(sus_nuevo);
                    self.guardar_informacion()?;
                    return Ok(());
                }
                return Err(Errores::ErrorOperatoria(ErroresOperatoria::Rechazado(
                    String::from("limite alcanzado para hacer upgrade"),
                )));
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::Rechazado(
                String::from("sin suscripciones activas"),
            )));
        }
        return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
            String::from("Registro de usuarios"),
        )));
    }
    pub fn downgrade(&mut self, u: &Usuario) -> Result<(), Errores> {
        if self.usuario_en_sistema(u.get_dni()) {
            if let Some(sus) = self
                .registro_suscripciones
                .iter_mut()
                .rev()
                .find(|s| s.dni_igual(u.get_dni()) && s.activo)
            {
                let mut sus_nuevo = sus.clone();
                sus.cancelar_suscripcion();
                if sus_nuevo.downgrade_tipo() {
                    self.registro_suscripciones.push(sus_nuevo);
                }

                self.guardar_informacion()?;
                return Ok(());
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::Rechazado(
                String::from("sin suscripciones activas para hacer downgrade"),
            )));
        }
        return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
            String::from("Registro de usuarios"),
        )));
    }
    pub fn cancelar_suscripcion(&mut self, u: &Usuario) -> Result<(), Errores> {
        if self.usuario_en_sistema(u.get_dni()) {
            if let Some(sus) = self
                .registro_suscripciones
                .iter_mut()
                .rev()
                .find(|s| s.dni_igual(u.get_dni()) && s.activo)
            {
                sus.cancelar_suscripcion();
                self.guardar_informacion()?;
                return Ok(());
            }
            return Err(Errores::ErrorOperatoria(ErroresOperatoria::Rechazado(
                String::from("sin suscripciones activas para cancelar"),
            )));
        }
        return Err(Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(
            String::from("Registro de usuarios"),
        )));
    }
    fn listado_suscripciones(&self, activos: bool) -> Vec<ContratoSuscripcion> {
        return self
            .registro_suscripciones
            .iter()
            .filter(|s| s.activo == activos)
            .cloned()
            .collect();
    }
    pub fn metodopago_max_suscripciones_activas(&self) -> Option<MediosDePago> {
        let mut res = None;
        let mut listado = self.listado_suscripciones(true);

        if !listado.is_empty() {
            let mut contador_tipos: HashMap<MediosDePago, u32> = HashMap::new();
            listado.iter().for_each(|s| {
                *contador_tipos.entry(s.tipo_pago.clone()).or_insert(0) += 1;
            });

            res = contador_tipos
                .into_iter()
                .max_by_key(|&(_, cant)| cant)
                .map(|(tipo, _)| tipo);
        }

        return res;
    }
    pub fn metodopago_max_suscripciones_inactivas(&self) -> Option<MediosDePago> {
        let mut res = None;
        let mut listado = self.listado_suscripciones(false);

        if !listado.is_empty() {
            let mut contador_tipos: HashMap<MediosDePago, u32> = HashMap::new();
            listado.iter().for_each(|s| {
                *contador_tipos.entry(s.tipo_pago.clone()).or_insert(0) += 1;
            });

            res = contador_tipos
                .into_iter()
                .max_by_key(|&(_, cant)| cant)
                .map(|(tipo, _)| tipo);
        }

        return res;
    }
    pub fn tipo_suscripcion_max_activas(&self) -> Option<TipoSuscripcion> {
        let mut res = None;
        let mut listado = self.listado_suscripciones(true);

        if !listado.is_empty() {
            let mut contador_tipos: HashMap<TipoSuscripcion, u32> = HashMap::new();
            listado.iter().for_each(|s| {
                *contador_tipos
                    .entry(s.tipo_suscripcion.clone())
                    .or_insert(0) += 1;
            });

            res = contador_tipos
                .into_iter()
                .max_by_key(|&(_, cant)| cant)
                .map(|(tipo, _)| tipo);
        }

        return res;
    }
    pub fn tipo_suscripcion_max_inactivas(&self) -> Option<TipoSuscripcion> {
        let mut res = None;
        let mut listado = self.listado_suscripciones(false);

        if !listado.is_empty() {
            let mut contador_tipos: HashMap<TipoSuscripcion, u32> = HashMap::new();
            listado.iter().for_each(|s| {
                *contador_tipos
                    .entry(s.tipo_suscripcion.clone())
                    .or_insert(0) += 1;
            });

            res = contador_tipos
                .into_iter()
                .max_by_key(|&(_, cant)| cant)
                .map(|(tipo, _)| tipo);
        }

        return res;
    }
}

#[cfg(test)]
mod test_ejercicio3 {
    use super::*;

    #[test]
    fn cambio_suscripcion() {
        let mut s1 = ContratoSuscripcion::new(
            1234,
            &TipoSuscripcion::Basic,
            100.0,
            2,
            Fecha::new(12, 05, 2026),
            &MediosDePago::Efectivo,
        );
        let mut s2 = ContratoSuscripcion::new(
            1234,
            &TipoSuscripcion::Super,
            100.0,
            2,
            Fecha::new(12, 05, 2026),
            &MediosDePago::Efectivo,
        );

        //Cambio nulo
        assert!(!s1.downgrade_tipo());
        assert_eq!(s1.tipo_suscripcion, TipoSuscripcion::Basic);

        assert!(!s2.upgrade_tipo());
        assert_eq!(s2.tipo_suscripcion, TipoSuscripcion::Super);

        //Cambio hecho
        assert!(s1.upgrade_tipo());
        assert_eq!(s1.tipo_suscripcion, TipoSuscripcion::Clasic);

        assert!(s2.downgrade_tipo());
        assert_eq!(s2.tipo_suscripcion, TipoSuscripcion::Clasic);
    }

    #[test]
    fn registro_inicial() {
        let mut sistema = Plataforma::new("./lista_suscripciones.json");
        let mut user1 = Usuario::new(&"Marco", 12345);
        let mut user2 = Usuario::new(&"Marco", 1234);

        assert!(sistema.registrar_usuario(&user1));
        assert!(sistema.registrar_usuario(&user2));
        assert_eq!(sistema.usuarios.len(), 2);
        assert!(!sistema.registrar_usuario(&user1));
        assert!(!sistema.registrar_usuario(&user2));
        assert_eq!(sistema.usuarios.len(), 2);

        let mut s1 = ContratoSuscripcion::new(
            1234,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );
        let mut s2 = ContratoSuscripcion::new(
            12345,
            &TipoSuscripcion::Super,
            5000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );
        let mut s3 = ContratoSuscripcion::new(
            2345,
            &TipoSuscripcion::Super,
            5000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );

        assert!(sistema.registrar_contrato(&s1).is_ok());
        assert!(sistema.registrar_contrato(&s2).is_ok());
        assert!(sistema.registrar_contrato(&s3).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::Inexistente(_))
            )
        }));
        assert!(sistema.registrar_contrato(&s1).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::ContratoActivo(_))
            )
        }));
        let mut s2 = ContratoSuscripcion::new(
            12345,
            &TipoSuscripcion::Clasic,
            5000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );
        assert!(sistema.registrar_contrato(&s2).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(
                e,
                Errores::ErrorOperatoria(ErroresOperatoria::ContratoActivo(_))
            )
        }));
        assert_eq!(sistema.listado_suscripciones(true).len(), 2);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_suscripciones.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    #[test]
    fn registro_operatoria() {
        let mut sistema = Plataforma::new("./lista_suscripciones2.json");
        let mut user1 = Usuario::new(&"Patricio", 12345);
        let mut user2 = Usuario::new(&"Patricio", 1234);

        sistema.registrar_usuario(&user1);
        sistema.registrar_usuario(&user2);

        //Suscripcion para user2
        let mut s1 = ContratoSuscripcion::new(
            1234,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );
        //Suscripcion para user1
        let mut s2 = ContratoSuscripcion::new(
            12345,
            &TipoSuscripcion::Super,
            5000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );

        assert!(sistema.registrar_contrato(&s1).is_ok());
        assert!(sistema.registrar_contrato(&s2).is_ok());

        assert!(sistema.downgrade(&user2).is_ok());
        assert!(sistema.upgrade(&user1).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(e, Errores::ErrorOperatoria(ErroresOperatoria::Rechazado(_)))
        }));

        assert_eq!(sistema.registro_suscripciones.len(), 2);

        assert!(sistema.upgrade(&user2).is_err_and(|e| {
            assert!(!e.to_string().is_empty());
            matches!(e, Errores::ErrorOperatoria(ErroresOperatoria::Rechazado(_)))
        }));
        assert!(sistema.downgrade(&user1).is_ok());
        assert!(sistema.downgrade(&user1).is_ok());

        s1 = ContratoSuscripcion::new(
            1234,
            &TipoSuscripcion::Clasic,
            2500.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );
        assert!(sistema.registrar_contrato(&s1).is_ok());
        assert!(sistema.cancelar_suscripcion(&user1).is_ok());

        assert_eq!(sistema.registro_suscripciones.len(), 5);
        assert_eq!(sistema.listado_suscripciones(true).len(), 1);
        assert_eq!(sistema.listado_suscripciones(false).len(), 4);

        //Limpieza para prevenir acumulacion de archivos
        assert!(
            std::fs::remove_file("./lista_suscripciones2.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    fn construir_sistema() -> Result<Plataforma, Errores> {
        let mut sistema = Plataforma::new("./lista_suscripciones3.json");
        let user1 = Usuario::new(&"Patricio", 12345);
        let user2 = Usuario::new(&"Patricio", 1234);
        let user3 = Usuario::new(&"Matias", 4554);
        let user4 = Usuario::new(&"David", 3487);

        sistema.registrar_usuario(&user1);
        sistema.registrar_usuario(&user2);
        sistema.registrar_usuario(&user3);
        sistema.registrar_usuario(&user4);

        let s1 = ContratoSuscripcion::new(
            12345,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );
        let s2 = ContratoSuscripcion::new(
            1234,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::MercadoPago(InfoMercadoPago {
                alias: "zapato".to_string(),
                cuil: 123456,
            }),
        );
        let s3 = ContratoSuscripcion::new(
            4554,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Criptomoneda(InfoCripto {
                wallet_address: "asd2354tg42t".to_string(),
                red: "%#1234".to_string(),
            }),
        );
        let s4 = ContratoSuscripcion::new(
            3487,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );

        sistema.registrar_contrato(&s1)?;
        sistema.registrar_contrato(&s2)?;
        sistema.registrar_contrato(&s3)?;
        sistema.registrar_contrato(&s4)?;

        return Ok(sistema);
    }

    #[test]
    fn validacion_maximos() {
        let mut sis = Plataforma::new("./lista_suscripciones_vacio.json");

        //Plataforma vacia
        assert!(sis.metodopago_max_suscripciones_activas().is_none());
        assert!(sis.tipo_suscripcion_max_activas().is_none());
        assert!(sis.metodopago_max_suscripciones_inactivas().is_none());
        assert!(sis.tipo_suscripcion_max_inactivas().is_none());

        assert!(
            construir_sistema().is_ok_and(|s| {
                sis = s;
                true
            }),
            "Aqui no debio fallar"
        );

        //Suscripciones activas
        assert!(
            sis.metodopago_max_suscripciones_activas()
                .is_some_and(|res| { matches!(res, MediosDePago::Efectivo) }),
            "Debio retornar un maximo"
        );

        assert!(
            sis.tipo_suscripcion_max_activas()
                .is_some_and(|res| { matches!(res, TipoSuscripcion::Basic) }),
            "Debio retornar un maximo"
        );

        assert!(sis.metodopago_max_suscripciones_inactivas().is_none());
        assert!(sis.tipo_suscripcion_max_inactivas().is_none());

        //Con registro de operaciones con suscripciones
        assert!(sis.downgrade(&Usuario::new(&"Patricio", 12345)).is_ok());
        assert!(sis.downgrade(&Usuario::new(&"Patricio", 1234)).is_ok());
        assert!(sis.upgrade(&Usuario::new(&"Matias", 4554)).is_ok());
        assert!(sis.upgrade(&Usuario::new(&"David", 3487)).is_ok());
        assert!(sis.upgrade(&Usuario::new(&"David", 3487)).is_ok());
        assert!(
            sis.cancelar_suscripcion(&Usuario::new(&"David", 3487))
                .is_ok()
        );

        //Activas
        assert!(
            sis.metodopago_max_suscripciones_activas()
                .is_some_and(|res| { matches!(res, MediosDePago::Criptomoneda(_)) }),
            "Debio retornar un maximo"
        );

        assert!(
            sis.tipo_suscripcion_max_activas()
                .is_some_and(|res| { matches!(res, TipoSuscripcion::Clasic) }),
            "Debio retornar un maximo"
        );

        //Inactivas
        assert!(
            sis.metodopago_max_suscripciones_inactivas()
                .is_some_and(|res| { matches!(res, MediosDePago::Efectivo) }),
            "Debio retornar un maximo"
        );

        assert!(
            sis.tipo_suscripcion_max_inactivas()
                .is_some_and(|res| { matches!(res, TipoSuscripcion::Basic) }),
            "Debio retornar un maximo"
        );

        //Limpieza para prevenir exceso de archivos
        assert!(
            std::fs::remove_file("./lista_suscripciones3.json").is_ok(),
            "Error fuera de lo previsto"
        );
    }

    /*
        Casos especiales para la cobertura de coverage
    */
    #[test]
    fn caso_especial_error_io_() {
        // Se buscara forzar un ErrorIO usando una ruta cuyo directorio base no existe
        let path_err = "./carpeta_inexistente_123/x.json";

        let mut sis = Plataforma::new(path_err);
        let user1 = Usuario::new(&"Patricio", 12345);

        sis.registrar_usuario(&user1);

        let s1 = ContratoSuscripcion::new(
            12345,
            &TipoSuscripcion::Basic,
            1000.0,
            5,
            Fecha::new(20, 01, 2025),
            &MediosDePago::Efectivo,
        );

        // Al intentar registrar una suscripcion, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO

        assert!(
            sis.registrar_contrato(&s1).is_err_and(|e| {
                assert!(!e.to_string().is_empty());
                matches!(e, Errores::ErrorIO(_))
            }),
            "Ocurrio un error imprevisto"
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
            Plataforma::recuperar_informacion(path_err).is_err_and(|e| {
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
