#[derive(Debug)]
pub struct Rectangulo{
    longitud : f32 ,
    ancho : f32
}

impl Rectangulo{
    fn new(long : f32,anc : f32)->Rectangulo{
        Rectangulo { longitud: long, ancho: anc }
    }
    fn calcular_area(&self)->f32{
        return self.longitud * self.ancho;
    }
    fn calcular_perimetro(&self)->f32{
        return self.longitud * 2.0 + self.ancho * 2.0
    }
    fn es_cuadrado(&self)->bool{
        return (self.ancho == self.longitud)
    }
}

#[cfg(test)]
mod testing_ejercicio2{
    use crate::tp3::ej2::Rectangulo;

    #[test]
    fn figura_vacia(){
        let fig = Rectangulo::new(0.0, 0.0);
        assert_eq!(fig.calcular_area(),0.0);
        assert_eq!(fig.calcular_perimetro(),0.0);
        assert!(fig.es_cuadrado());
    }

    #[test]
    fn cuadrados_y_rectangulos(){
        let rec = Rectangulo::new(12.5, 10.0);
        assert_eq!(rec.calcular_area(),125.0);
        assert_eq!(rec.calcular_perimetro(),45.0);
        assert!(!rec.es_cuadrado());

        let cuadrado = Rectangulo::new(12.0, 12.0);
        assert_eq!(cuadrado.calcular_area(),144.0);
        assert_eq!(cuadrado.calcular_perimetro(),48.0);
        assert!(cuadrado.es_cuadrado());
    }
}