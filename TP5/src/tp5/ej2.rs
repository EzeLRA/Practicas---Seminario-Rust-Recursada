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
enum error_operatoria{
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
    pub fn get_titulo(&self)->&String{
        return &self.titulo;
    }
    pub fn get_artista(&self)->&String{
        return &self.artista;
    }
    pub fn get_genero(&self)->&Generos{
        return &self.genero
    }
    pub fn es_igual_a(&self,c:&Cancion)->bool{
        return (&self.titulo == c.get_titulo())&&(&self.artista == c.get_artista())&&
        (self.genero.es_igual_a(&c.get_genero()));
    }
    //Metodos primarios
    pub fn new(nom1:&str,nom2:&str,gen_in:Generos)->Cancion{
        return Cancion{
            titulo : nom1.to_string(),
            artista : nom2.to_string(),
            genero : gen_in
        }
    }    
}

impl PlayList{
    //Metodos secundarios
    pub fn get_nombre(&self)->&String{
        return &self.nombre; 
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
	    let mut file = File::create(&self.path)?;
	    let serialized = serde_json::to_string(&self.canciones)?;
        file.write_all(serialized.as_bytes())?;
        return Ok(())
    }
    //Metodos primarios
    pub fn new(nom:&str,path_in:&str)->PlayList{
        let list_canciones : Vec<Cancion> = match PlayList::recuperar_informacion(&path_in){
			Ok(dato) => {
                dato
            }
            Err(_) => {
                Vec::new()
            }
		};
        return PlayList { nombre: nom.to_string(), canciones: list_canciones , path: path_in.to_string() }
    }
    pub fn agregar_cancion(&mut self,c:Cancion)->Result<(),Errores>{
        self.canciones.push(c);
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
            return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre().clone())))
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())))
    }
    //la posicion oscila entre el rango (0..tam-1) , como lo hacen los metodos genericos de vec .get()
    pub fn mover_cancion(&mut self,c:&Cancion ,pos:usize) -> Result<(), Errores> {
        if !self.canciones.is_empty() {
            if pos < self.canciones.len() {
                
                if let Some(indice) = self.canciones.iter().position(|cancion| cancion.es_igual_a(c)) {
                    if indice != pos {
                        let cancion = self.canciones.remove(indice);
                        self.canciones.insert(pos, cancion);
                    }
                    
                    self.guardar_informacion()?;
                    return Ok(());
                }
                return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.get_nombre().clone())));
            }
            return Err(Errores::ErrorOperatoria(error_operatoria::SinDesplazamiento(self.get_nombre().clone())));
        }
        Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())))
    }
    
    pub fn buscar_cancion(&self,nom:&String)->Option<&Cancion>{
		let mut res : Option<&Cancion> = None;
		if !self.canciones.is_empty() {
			for cancion in &self.canciones{
				if cancion.get_titulo() == nom {
					res = Some(cancion);
                    break;
				}
			}
		}
		return res;
	}
    pub fn canciones_genero(&self,gen_in:&Generos)->Result<Vec<&Cancion>,Errores>{
        
        if !self.canciones.is_empty() {
            let mut res = Vec::new();
            for cancion in &self.canciones{
                if cancion.genero.es_igual_a(gen_in) {
                    res.push(cancion);
                    
                }
            }
            return Ok(res)
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())));
    }
    pub fn canciones_artista(&self,nom:&String)->Result<(Vec<&Cancion>),Errores>{
        
        if !self.canciones.is_empty() {
            let mut res = Vec::new();
            for cancion in &self.canciones{
                if cancion.get_artista() == nom {
                    res.push(cancion);
                    
                }
            }
            return Ok(res)
        }
        return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.get_nombre().clone())));
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
        assert_eq!(p.get_nombre(),&nom_pri.to_string());
        nom_pri = "repro1".to_string();
        p.modificar_titulo(&nom_pri);
        assert_eq!(p.get_nombre(),&nom_pri.to_string());
    }
    #[test]
    fn operatoria_canciones(){
        let mut p = PlayList::new(&"asd".to_string(),"./lista_canciones.json");
        let c = Cancion::new(&"pepe", &"pepito", Generos::Rap);
        assert!(p.agregar_cancion(c.clone()).is_ok());
        assert!(p.agregar_cancion(c.clone()).is_ok());
        assert!(p.buscar_cancion(&"pepe".to_string()).is_some_and(|c1| c1.es_igual_a(&c)),"No existe esa cancion");

        //Desplazamientos
        let c2 = Cancion::new(&"pepo", & "pepe", Generos::Rap);
        assert!(p.agregar_cancion(c2.clone()).is_ok());
        assert!(p.mover_cancion(&c2,0).is_ok());
        assert!(p.canciones.get(0).is_some_and(|c| c.es_igual_a(&c2)),"No existe esa cancion");
        let c3 = Cancion::new(&"pimpon", &"tito", Generos::Rap);
        assert!(p.agregar_cancion(c3.clone()).is_ok());
        assert!(p.mover_cancion(&c3,2).is_ok());
        assert!(p.canciones.get(2).is_some_and(|c| c.es_igual_a(&c3)),"No existe esa cancion");
        assert!(p.mover_cancion(&c2,3).is_ok());
        assert!(p.canciones.get(p.canciones.len()-1).is_some_and(|c| c.es_igual_a(&c2)),"No existe esa cancion");
        assert!(p.mover_cancion(&c2,4).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e, Errores::ErrorOperatoria(error_operatoria::SinDesplazamiento(_)))
        }),"Aqui debio fallar");

        //Limpieza completa
        assert!(p.eliminar_canciones().is_ok());
        assert!(p.mover_cancion(&c2,0).is_err_and(|e|{
            assert!(!e.to_string().is_empty());
            matches!(e, Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(_)))
        }),"Aqui debio fallar");
    }
    #[test]
    fn listado_canciones(){
        let mut p = PlayList::new(&"asd","./lista_canciones2.json");
        let c1 = Cancion::new(&"pepe", &"pepito", Generos::Rap);
        let c4 = Cancion::new(&"pepesito", &"pepito", Generos::Jazz);
        let c2 = Cancion::new(&"donPepe", &"donPepito", Generos::Rap);
        let c3 = Cancion::new(&"qwe", &"Qwe", Generos::Rock);
        assert!(p.agregar_cancion(c1.clone()).is_ok());
        assert!(p.agregar_cancion(c2).is_ok());
        assert!(p.agregar_cancion(c1).is_ok());
        assert!(p.agregar_cancion(c3.clone()).is_ok());
        assert!(p.agregar_cancion(c3).is_ok());
        assert!(p.agregar_cancion(c4).is_ok());

        //Listados
        assert!(p.canciones_genero(&Generos::Rap).is_ok_and(|l|{
            assert_eq!(l.len(),3);
            l.iter().all(|c| c.genero.es_igual_a(&Generos::Rap))
        }));
        
        assert!(p.canciones_artista(&"pepito".to_string()).is_ok_and(|l|{
            assert_eq!(l.len(),3);
            l.iter().all(|c| c.get_artista() == &"pepito".to_string())
        }));

        assert!(p.eliminar_canciones().is_ok());

        assert!(p.canciones_genero(&Generos::Rock).is_err_and(|e| matches!(e,Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(_)))));
        assert!(p.canciones_artista(&"Joaco".to_string()).is_err_and(|e| matches!(e,Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(_)))));
    }

    /*
		Casos especiales para la cobertura de coverage
	*/
	#[test]
	fn caso_especial_error_io() {
		// Se buscara forzar un ErrorIO usando una ruta cuyo directorio base no existe
		let path_imposible = "./carpeta_inexistente_123/x.json";

		let mut p = PlayList::new(&"asd",path_imposible);
        let c1 = Cancion::new(&"pepe", &"pepito", Generos::Rap);
        
		// Al intentar agregar el elemento, llamará internamente a File::create() en la ruta rota, provocando un ErrorIO
		assert!(p.agregar_cancion(c1).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorIO(_))
		}),"Aquí debió fallar");
	}

	#[test]
	fn caso_especial_error_serde() {
		let path_err = "./corrupto.json";
		
		// Se fuerza la escritura en el contenido temporal que NO cumple con el formato estructurado de un .JSON válido
		assert!(std::fs::write(path_err, "{ &&5435#$#$&42365_XXXX1234 : [::: ").is_ok(),"No debio fallar aqui");

		// Se invoca directamente el método para leer el archivo del path que buscara

		assert!(PlayList::recuperar_informacion(path_err).is_err_and(|e|{
			assert!(!e.to_string().is_empty());
			matches!(e, Errores::ErrorSerde(_))
		}),"Aquí debió fallar");

		assert!(std::fs::remove_file(path_err).is_ok(),"Error fuera de lo previsto");
		
	}
}