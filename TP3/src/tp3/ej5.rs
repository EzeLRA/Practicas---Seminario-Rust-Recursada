/* 
    Estructura Producto
*/

//Atributos
#[derive(Debug)]
pub struct Producto{
    nombre : String,
    precio_bruto : f32,
    num_identificacion : i32
}

//Metodos
impl Producto{
    //Metodos secundarios
    pub fn es_igual_a(&self,p:&Producto)->bool{
        return (self.nombre == p.nombre)&&(self.precio_bruto == p.precio_bruto)&&(self.num_identificacion == p.num_identificacion);
    }
    //Metodos primarios
    pub fn new(nom:String,precio:f32,num:i32)->Producto{
        Producto { nombre: nom, precio_bruto: precio, num_identificacion: num }
    }
    //Se calcula el valor del monto de los impuestos en base al precio bruto (no el monto final con el impuesto)
    pub fn calcular_impuestos(&self,porcentaje:f32)->f32{
        return self.precio_bruto*(porcentaje/100.0);
    }
    //Se calcula el valor del monto del descuento en base al precio bruto (no el monto final con el descuento)
    pub fn aplicar_descuento(&self,porcentaje:f32)->f32{
        return self.precio_bruto*(porcentaje/100.0);
    }
    pub fn calcular_precio_total(&self,porcentaje_de_impuestos:Option<f32>,porcentaje_descuento:Option<f32>)->f32{
        let mut monto = self.precio_bruto;
        
        if let Some(p) = porcentaje_de_impuestos{
            monto += self.calcular_impuestos(p);
        }

        if let Some(p) = porcentaje_descuento{
            monto -= self.aplicar_descuento(p);
        }

        return monto;
    }
    
}


#[cfg(test)]
mod testing_producto{
    use super::Producto;

    #[test]
    fn crear_producto(){
        let p = Producto::new(String::from("Serenito"), 8500.0, 12452);
        assert_eq!(p.es_igual_a(&Producto::new(String::from("Serenito"), 8500.0, 12452)),true);
    }

    #[test]
    fn calcular_precios(){
        let p = Producto::new(String::from("Baggio"), 5000.0, 5432);
        assert_eq!(p.calcular_impuestos(10.0),500.0);
        assert_eq!(p.calcular_impuestos(0.0),0.0);
        assert_eq!(p.aplicar_descuento(10.0),500.0);
        assert_eq!(p.aplicar_descuento(0.0),0.0);
    }

    #[test]
    fn estimar_precio(){
        let p = Producto::new(String::from("Milka"), 1000.0, 8932);
        assert_eq!(p.calcular_precio_total(Some(10.0), Some(10.0)),1000.0);
        assert_eq!(p.calcular_precio_total(None, None),1000.0);
    }

}