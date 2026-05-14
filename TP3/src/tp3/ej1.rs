use std::ops::Add;

pub struct Persona{
    nombre : Option<String>,
    edad : Option<u32>,
    direccion : Option<String>,
}

impl Persona{
    fn new(nom : &String , anios : u32 , dir : &String)->Persona{
        Persona {
            nombre : Some(nom.clone()),
            edad : Some(anios),
            direccion : Some(dir.clone())
        }
    }    
    fn to_string(&self)->String{
        let mut res = String::from("");
        
        if let Some(nom) = &self.nombre {
            res.push_str(nom);
            res.push_str(" , ");
        }
        if let Some(anios) = self.edad {
            res.push_str(&anios.to_string());
            res.push_str(" , ");
        }
        if let Some(dir) = &self.direccion {
            res.push_str(dir);
            res.push_str(" ; ");
        }

        return res;
    }
    fn obtener_edad(&self)->u32{
        if let Some(anios) = &self.edad{
            return *anios;
        }
        return 0;
    }
    fn actualizar_direccion(&mut self,nueva_direccion : String){
        self.direccion = Some(nueva_direccion);
    }
}