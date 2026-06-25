use std::fmt::Display;
use serde::{Serialize, Deserialize};
use serde_json;
use std::{fs::File, io::{Error, Read, Write}};
use std::io;

/*
    Implementacion TP5 - Ej2
*/

/*
	Tipos de errores
*/
#[derive(Debug)]
pub enum error_operatoria{
    SinDesplazamiento(String),
	Inexistente(String),
	EstructuraVacia(String)
}

impl Display for error_operatoria{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self{
            error_operatoria::SinDesplazamiento(val) => write!(f, "La posicion recibida no es valida para la operacion en la estructura {} ",val),
			error_operatoria::Inexistente(val) => write!(f, "No se encontro el elemento en la estructura {} ",val),
			error_operatoria::EstructuraVacia(val) => write!(f, "La estrucutra {} no dispone de elementos ",val)
		}
	}
}

#[derive(Debug)]
pub enum Errores{
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
    Extraccion Ejercicio 8 - TP3
    Estructuras : Cancion , Generos y PlayList
*/

#[derive(Debug,Clone,Serialize,Deserialize)]
enum Generos{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Cancion{
    titulo : String,
    artista : String,
    genero : Generos
}

#[derive(Debug)]
struct PlayList{
    nombre: String,
    canciones : Vec<Cancion>,
    path: String
}

/*
    Metodos asociados
*/

//Metodos para Cancion
impl Generos{
    pub fn es_igual_a(&self,g:&Generos)->bool{
        match (self, g) {
            (Generos::Rock, Generos::Rock) => true,
            (Generos::Pop, Generos::Pop) => true,
            (Generos::Rap, Generos::Rap) => true,
            (Generos::Jazz, Generos::Jazz) => true,
            (Generos::Otros, Generos::Otros) => true,
            _ => false
        }
    }
}
impl Cancion{
    //Metodos secundarios
    pub fn get_titulo(&self)->String{
        return self.titulo.clone();
    }
    pub fn get_artista(&self)->String{
        return self.artista.clone();
    }
    pub fn get_genero(&self)->Generos{
        return self.genero.clone()
    }
    pub fn es_igual_a(&self,c:&Cancion)->bool{
        return (self.titulo == c.get_titulo())&&(self.artista == c.get_artista())&&
        (self.genero.es_igual_a(&c.get_genero()));
    }
    //Metodos primarios
    pub fn new(nom1:&String,nom2:&String,gen_in:&Generos)->Cancion{
        return Cancion{
            titulo : nom1.clone(),
            artista : nom2.clone(),
            genero : gen_in.clone()
        }
    }    
}

impl PlayList{
    //Metodos secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone(); 
    }
    /*
		Nuevos metodos del tp5
	*/
	fn recuperar_informacion(path:&str)-> Result<Vec<Cancion>,Errores>{
		let file = File::open(path).map_err(Errores::ErrorIO)?;
		let canciones: Vec<Cancion> = serde_json::from_reader(file).map_err(Errores::ErrorSerde)?;
		Ok(canciones)
	}
	fn guardar_informacion(&self) -> Result<(), Errores> {
	    let mut file = File::create(&self.path).map_err(Errores::ErrorIO)?;
	    let serialized = serde_json::to_string(&self.canciones).map_err(Errores::ErrorSerde)?;
        file.write_all(serialized.as_bytes()).map_err(Errores::ErrorIO)
    }
    //Metodos primarios
    pub fn new(nom:&String,path_in:&str)->PlayList{
        return PlayList { nombre: nom.to_string(), canciones: Vec::new() , path: path_in.to_string() }
    }
    pub fn agregar_cancion(&mut self,c:&Cancion)->Result<(),Errores>{
        self.canciones.push(c.clone());
        self.guardar_informacion()?;
        return Ok(())
    }
    pub fn eliminar_cancion(&mut self,c:&Cancion)->Result<(),Errores>{
        if !self.canciones.is_empty() {
            let mut pos = None;
            for i in 0..self.canciones.len(){
                if self.canciones[i].es_igual_a(&c) {
                    pos = Some(i);
                    break;
                }
            }
            if let Some(indice_c) = pos {
                self.canciones.remove(indice_c);
                self.guardar_informacion()?;
                return Ok(())
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre())))
    }
    pub fn mover_cancion(&mut self,c:&Cancion,pos:usize)->Result<(),Errores>{
        if !self.canciones.is_empty()&&(pos<=self.canciones.len()){
            let mut pos_aux = None;
            for i in 0..self.canciones.len(){
                if self.canciones[i].es_igual_a(&c) {
                    pos_aux = Some(i);
                    break;
                }
            }
            if let Some(indice) = pos_aux{
                let cancion = self.canciones.remove(indice);
                self.canciones.insert(pos, cancion);
                self.guardar_informacion()?;
                return Ok(())
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::SinDesplazamiento(self.get_nombre())))
    }
    pub fn buscar_cancion(&self,nom:&String)->Option<Cancion>{
		let mut res : Option<Cancion> = None;
		if !self.canciones.is_empty() {
			for cancion in self.canciones.clone(){
				if cancion.get_titulo() == *nom {
					res = Some(cancion);
                    break;
				}
			}
		}
		return res;
	}
    pub fn canciones_genero(&self,gen_in:&Generos)->Vec<Cancion>{
        let mut res : Vec<Cancion> = Vec::new();
        if !self.canciones.is_empty() {
            for cancion in self.canciones.clone(){
                if cancion.genero.es_igual_a(gen_in) {
                    res.push(cancion);
                    
                }
            }
        }
        return res;
    }
    pub fn canciones_artista(&self,nom:&String)->Vec<Cancion>{
        let mut res : Vec<Cancion> = Vec::new();
        if !self.canciones.is_empty() {
            for cancion in self.canciones.clone(){
                if cancion.get_artista() == *nom {
                    res.push(cancion);
                    
                }
            }
        }
        return res;
    }
    pub fn modificar_titulo(&mut self,nom_nuevo:&String){
        self.nombre = nom_nuevo.to_string();
    }
    pub fn eliminar_canciones(&mut self)->Result<(),Errores>{
        self.canciones.clear();
        self.guardar_informacion()?;
        return Ok(())
    }
}


#[cfg(test)]
mod testing_ejercicio8{
    use super::*;

    #[test]
    fn manipulacion_playlist(){
        let mut nom_pri = "reproductor1".to_string();
        let mut p = PlayList::new(&nom_pri,"");
        assert_eq!(p.get_nombre(),nom_pri);
        nom_pri = "repro1".to_string();
        p.modificar_titulo(&nom_pri);
        assert_eq!(p.get_nombre(),nom_pri);
    }
    #[test]
    fn operatoria_canciones(){
        let mut p = PlayList::new(&"asd".to_string(),"");
        let c = Cancion::new(&String::from("pepe"), &String::from("pepito"), &Generos::Rap);
        p.agregar_cancion(&c);
        p.agregar_cancion(&c);
        if let Some(aux) = p.buscar_cancion(&"pepe".to_string()){
            assert_eq!(aux.es_igual_a(&c),true);
        }else{
            panic!("No existe esa cancion");
        }

        let c2 = Cancion::new(&String::from("pepo"), &String::from("pepe"), &Generos::Rap);
        p.agregar_cancion(&c2);
        p.mover_cancion(&c2,0);
        if let Some(aux) = p.canciones.get(0){
            assert_eq!(aux.es_igual_a(&c2),true);
        }else{
            panic!("No existe esa cancion");
        }

        p.eliminar_canciones();
        assert_eq!(p.canciones.is_empty(),true);
    }
    #[test]
    fn listado_canciones(){
        let mut p = PlayList::new(&"asd".to_string(),"");
        let c1 = Cancion::new(&String::from("pepe"), &String::from("pepito"), &Generos::Rap);
        let c4 = Cancion::new(&String::from("pepesito"), &String::from("pepito"), &Generos::Jazz);
        let c2 = Cancion::new(&String::from("donPepe"), &String::from("donPepito"), &Generos::Rap);
        let c3 = Cancion::new(&String::from("qwe"), &String::from("Qwe"), &Generos::Rock);
        p.agregar_cancion(&c1);
        p.agregar_cancion(&c2);
        p.agregar_cancion(&c1);
        p.agregar_cancion(&c3);
        p.agregar_cancion(&c3);
        p.agregar_cancion(&c4);

        //Listados
        let lista1 = p.canciones_genero(&Generos::Rap);
        
        if !lista1.is_empty(){
            assert_eq!(lista1.len(),3);
            for cancion in lista1{
                assert_eq!(cancion.genero.es_igual_a(&Generos::Rap),true);
            }
        }else{
            panic!("Lista 1 no generada");
        }

        let lista2 = p.canciones_artista(&"pepito".to_string());

        if !lista2.is_empty(){
            assert_eq!(lista2.len(),3);
            for cancion in lista2{
                assert_eq!(cancion.get_artista() == "pepito".to_string(),true);
            }
        }else{
            panic!("Lista 2 no generada");
        }
    }
}