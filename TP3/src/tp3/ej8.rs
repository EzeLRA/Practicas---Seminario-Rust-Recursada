/*
    Estructuras : Cancion , Generos y PlayList
*/
#[derive(Debug,Clone)]
pub enum Generos{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}
#[derive(Debug,Clone)]
pub struct Cancion{
    titulo : String,
    artista : String,
    genero : Generos
}
#[derive(Debug)]
pub struct PlayList{
    nombre: String,
    canciones : Vec<Cancion>
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
    pub fn es_igual_a(&self,c:&Cancion)->bool{
        return (self.titulo == c.get_titulo())&&(self.artista == c.get_artista())&&
        (self.genero.es_igual_a(&c.genero));
    }
    //Metodos primarios
    pub fn new(nom1:String,nom2:String,gen_in:Generos)->Cancion{
        return Cancion{
            titulo : nom1,
            artista : nom2,
            genero : gen_in
        }
    }    
}

impl PlayList{
    //Metodos secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone(); 
    }
    //Metodos primarios
    pub fn new(nom:&String)->PlayList{
        return PlayList { nombre: nom.to_string(), canciones: Vec::new() }
    }
    pub fn agregar_cancion(&mut self,c:&Cancion){
        self.canciones.push(c.clone());
    }
    pub fn eliminar_cancion(&mut self,c:&Cancion){
        if !self.canciones.is_empty() {
            for i in 0..self.canciones.len(){
                if let Some(cancion) = self.canciones.get(i){
                    if cancion.es_igual_a(&c) {
                        self.canciones.remove(i);
                        break;
                    }
                }
            }
        }
    }
    pub fn mover_cancion(&mut self,c:&Cancion,pos:usize){
        if !self.canciones.is_empty()&&(pos<=self.canciones.len()){
            for i in 0..self.canciones.len(){
                if self.canciones[i].es_igual_a(&c) {
                    let cancion = self.canciones[i].clone();
                    self.canciones.remove(i);
                    self.canciones.insert(pos, cancion);
                    break;
                }
            }
        }
    }
    pub fn buscar_cancion(&self,nom:String)->Option<Cancion>{
		let mut res : Option<Cancion> = None;
		if !self.canciones.is_empty() {
			for cancion in self.canciones.clone(){
				if cancion.get_titulo() == nom {
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
                    break;
                }
            }
        }
        return res;
    }
    pub fn canciones_artista(&self,nom:String)->Vec<Cancion>{
        let mut res : Vec<Cancion> = Vec::new();
        if !self.canciones.is_empty() {
            for cancion in self.canciones.clone(){
                if cancion.get_artista() == nom {
                    res.push(cancion);
                    break;
                }
            }
        }
        return res;
    }
    pub fn modificar_titulo(&mut self,nom_nuevo:&String){
        self.nombre = nom_nuevo.to_string();
    }
    pub fn eliminar_canciones(&mut self) {
        self.canciones.clear();
    }
}


#[cfg(test)]
mod testing_ejercicio8{
    use super::*;

    #[test]
    fn manipulacion_playlist(){
        let mut nom_pri = "reproductor1".to_string();
        let mut p = PlayList::new(&nom_pri);
        assert_eq!(p.get_nombre(),nom_pri);
        nom_pri = "repro1".to_string();
        p.modificar_titulo(&nom_pri);
        assert_eq!(p.get_nombre(),nom_pri);
    }
    #[test]
    fn operatoria_canciones(){
        let mut p = PlayList::new(&"asd".to_string());
        let c = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        p.agregar_cancion(&c);
        p.agregar_cancion(&c);
        if let Some(aux) = p.buscar_cancion("pepe".to_string()){
            assert_eq!(aux.es_igual_a(&c),true);
        }else{
            panic!("No existe esa cancion");
        }

        let c2 = Cancion::new(String::from("pepo"), String::from("pepe"), Generos::Rap);
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
        let mut p = PlayList::new(&"asd".to_string());
        let c1 = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        let c2 = Cancion::new(String::from("donPepe"), String::from("donPepito"), Generos::Rap);
        let c3 = Cancion::new(String::from("qwe"), String::from("Qwe"), Generos::Rock);
        p.agregar_cancion(&c1);
        p.agregar_cancion(&c2);
        p.agregar_cancion(&c1);
        p.agregar_cancion(&c3);
        p.agregar_cancion(&c3);

        //Listados de un unico uso para el test
        let lista1 = p.canciones_genero(&Generos::Rap);
        
        if !lista1.is_empty(){
            for cancion in lista1{
                assert_eq!(cancion.genero.es_igual_a(&Generos::Rap),true);
            }
        }else{
            panic!("Lista 1 no generada");
        }

        let lista2 = p.canciones_artista("pepito".to_string());

        if !lista2.is_empty(){
            for cancion in lista2{
                assert_eq!(cancion.get_artista() == "pepito".to_string(),true);
            }
        }else{
            panic!("Lista 2 no generada");
        }
    }
}