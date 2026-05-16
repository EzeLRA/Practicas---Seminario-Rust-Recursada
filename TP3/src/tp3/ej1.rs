
#[derive(Debug)]
pub struct Persona{
    nombre : Option<String>,
    edad : Option<u32>,
    direccion : Option<String>,
}

impl Persona{
    fn new(nom : Option<String> , anios : Option<u32> , dir : Option<String>)->Persona{
        Persona {
            nombre : nom,
            edad : anios,
            direccion : dir
        }
    }    
    fn to_string(&self)->String{
        let mut res = String::from("");
        
        if let Some(nom) = &self.nombre {
            res.push_str(nom);
        }

        res.push_str(",");
        
        if let Some(anios) = &self.edad {
            res.push_str(&anios.to_string());
        }

        res.push_str(",");
        
        if let Some(dir) = &self.direccion {
            res.push_str(dir);
        }

        res.push_str(";");

        return res;
    }
    fn obtener_edad(&self)->u32{
        if let Some(anios) = &self.edad{
            return *anios;
        }
        return 0;
    }
    fn actualizar_direccion(&mut self,nueva_direccion : &String){
        self.direccion = Some(nueva_direccion.clone());
    }
}

#[cfg(test)]
mod testing_ejercicio1{
    use crate::tp3::ej1::Persona;

    #[test]
    fn persona_con_datos(){
        
        let mut persona = Persona::new(Some("Javier".to_string()), Some(30), Some("Calle 8".to_string()));
        
        //Verificacion de datos
        assert_eq!(persona.obtener_edad(),30);

        assert_eq!(persona.to_string(),"Javier,30,Calle 8;".to_string());

        //Modificacion del domicilio
        persona.actualizar_direccion(&"Calle 109".to_string());
        assert_eq!(persona.to_string(),"Javier,30,Calle 109;".to_string());
    }

    #[test]
    fn persona_sin_datos(){
        
        let mut persona = Persona::new(None, None, None);
        
        //Verificacion de datos
        assert_eq!(persona.obtener_edad(),0);

        assert_eq!(persona.to_string(),",,;".to_string());

        //Modificacion del domicilio
        persona.actualizar_direccion(&"Calle 10".to_string());
        assert_eq!(persona.to_string(),",,Calle 10;".to_string());
    }
}