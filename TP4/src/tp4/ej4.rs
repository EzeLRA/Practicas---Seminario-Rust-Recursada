use std::collections::HashMap;

#[derive(PartialEq,Eq,Hash,Debug,Clone)]
enum Categorias {
    Alimento,
    Bazar,
    Limpieza,
    Otro,
}

#[derive(PartialEq,Debug,Clone)]
enum MediosDePago {
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
    Efectivo,
}

#[derive(PartialEq,Debug,Clone)]
struct Producto {
    nombre: String,
    categoria: Categorias,
    precio_base: f64,
}

impl Producto {
    pub fn new(nombre: &str, categoria: Categorias, precio_base: f64) -> Producto {
        Producto{
            nombre: nombre.to_string(),
            categoria,
            precio_base,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct ProductoVendido {
    producto: Producto,
    cantidad: u64,
}

impl ProductoVendido {
    pub fn new(prod: Producto, cantidad: u64) -> ProductoVendido {
        ProductoVendido{
            producto: prod,
            cantidad,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct DatosPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u64,
}

impl DatosPersona{
    pub fn new(nom:&str,ape:&str,dir:&str,dni_in:u64)->DatosPersona{
        DatosPersona { nombre: nom.to_string(), apellido: ape.to_string(), direccion: dir.to_string(), dni: dni_in }
    }
}
#[derive(PartialEq,Debug,Clone)]
struct Cliente {
    datos: DatosPersona,
    correo_newsletter: Option<String>,
}

impl Cliente {
    pub fn new(nombre: &str, apellido: &str, direccion: &str, dni: u64) -> Cliente {
        Cliente {
            datos : DatosPersona::new(nombre, apellido, direccion, dni),
            correo_newsletter: None,
        }
    }
    pub fn suscribir_newsletter(&mut self, correo: &str) {
        self.correo_newsletter = Some(correo.to_string());
    }
    pub fn tiene_newsletter(&self) -> bool {
        self.correo_newsletter.is_some()
    }
}

#[derive(PartialEq,Debug,Clone)]
struct Vendedor {
    datos: DatosPersona,
    legajo: u64,
    antiguedad: u8,
    salario: f64,
}

impl Vendedor {
    pub fn new(nombre: &str, apellido: &str, direccion: &str, dni: u64, legajo: u64, antiguedad: u8, salario: f64) -> Vendedor {
        Vendedor {
            datos: DatosPersona::new(nombre, apellido, direccion, dni),
            legajo,
            antiguedad,
            salario,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct Venta {
    fecha: String,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_pago: MediosDePago,
    productos: Vec<ProductoVendido>,
}

impl Venta {
    pub fn new(fecha: &str, cli: Cliente, vend: Vendedor, m_pago: MediosDePago, products: Vec<ProductoVendido>) -> Venta {
        Venta{
            fecha: fecha.to_string(),
            cliente: cli,
            vendedor: vend,
            medio_pago: m_pago,
            productos: products,
        }
    }
}

#[derive(Debug)]
struct ReporteCategoria{
    categoria: Categorias,
    monto_total: f64
}

impl ReporteCategoria{
    pub fn new(categ:Categorias,total:f64)->ReporteCategoria{
        return ReporteCategoria { categoria: categ, monto_total: total }
    }
}

#[derive(Debug)]
struct ReporteVendedor{
    legajo_v: u64,
    monto_total : f64,
}

impl ReporteVendedor{
    pub fn new(legajo:u64,total:f64)->ReporteVendedor{
        return ReporteVendedor { legajo_v: legajo, monto_total: total }
    }
}

#[derive(Debug)]
struct Reporte<T>{
    listado: Vec<T>
}

impl<T> Reporte<T>{
    pub fn new()->Reporte<T>{
        return Reporte { listado: Vec::new(), }
    }
    pub fn agregar(&mut self, item: T) {
        self.listado.push(item);
    }
}

// El sistema mantiene el registro de todo y contiene los datos necesarios
struct Sistema {
    ventas: Vec<Venta>,
    vendedores : Vec<Vendedor>,
    descuentos_categorias: HashMap<Categorias, f64>, // Lista de categorías con descuento
    newsletter : String,
    descuento_newsletter: f64,                      // Porcentaje general por newsletter
}

impl Sistema {
    pub fn new(correo:&str,descuento_newsletter: f64) -> Self {
        Self {
            ventas: Vec::new(),
            vendedores : Vec::new(),
            descuentos_categorias: HashMap::new(),
            newsletter: correo.to_string(),
            descuento_newsletter,
        }
    }
    pub fn otorgar_newsletter(&self,c:&mut Cliente){
        c.suscribir_newsletter(&self.newsletter.clone());
    }
    pub fn configurar_descuento_categoria(&mut self, categoria: Categorias, porcentaje: f64) {
        self.descuentos_categorias.insert(categoria, porcentaje);
    }

    pub fn registrar_vendedor(&mut self,v:Vendedor)->bool{
        let mut exito = false;
        
        if !self.vendedores.iter().any(|vendedor| vendedor.legajo == v.legajo){
            self.vendedores.push(v);
            exito = true;
        }

        return exito
    }

    pub fn registrar_venta(&mut self, venta: Venta)->bool{
        let mut exito = false;
        if self.vendedores.iter().any(|vendedor| vendedor.legajo == venta.vendedor.legajo){
            self.ventas.push(venta);
            exito = true;
        }
        return exito
    }

    pub fn calcular_precio_final(&self, venta: &Venta) -> f64 {
        let mut subtotal_venta = 0.0;

        // Aplicacion de descuento para las categorias de cada producto
        subtotal_venta = venta.productos.iter().map(|p|{
            let descuento_categoria = if let Some(porcentaje) = self.descuentos_categorias.get(&p.producto.categoria){*porcentaje}else{0.0};

            let precio_con_descuento = p.producto.precio_base * (1.0 - (descuento_categoria / 100.0));
            
            precio_con_descuento * (p.cantidad as f64)
        }).sum();
        
        if venta.cliente.tiene_newsletter() {
            subtotal_venta *= 1.0 - (self.descuento_newsletter / 100.0);
        }

        return subtotal_venta
    }

    pub fn reporte_ventas_por_categoria(&self) -> Option<Reporte<ReporteCategoria>>{
        let mut reporte = HashMap::new();
        
        self.ventas.iter().for_each(|v|{
            let tiene_descuento = v.cliente.tiene_newsletter();
            v.productos.iter().for_each(|p|{
                let descuento_categoria = if let Some(porcentaje) = self.descuentos_categorias.get(&p.producto.categoria){*porcentaje}else{0.0};
                let precio_final_unitario = p.producto.precio_base * (1.0 - (descuento_categoria / 100.0));
                let mut monto_item = precio_final_unitario * (p.cantidad as f64);
                    
                // Si la venta global tuvo descuento por newsletter, impacta proporcionalmente al item
                if tiene_descuento {
                    monto_item *= 1.0 - (self.descuento_newsletter / 100.0);
                }

                let total_cat = reporte.entry(p.producto.categoria.clone()).or_insert(0.0);
                *total_cat += monto_item;
            });
        });

        if !reporte.is_empty(){
            let mut res = Reporte::new();
            reporte.iter().for_each(|r|{
                res.agregar(ReporteCategoria::new(r.0.clone(),*r.1));
            });
            return Some(res)
        }

        return None
    }

    pub fn reporte_ventas_por_vendedor(&self) -> Option<Reporte<ReporteVendedor>> {
        let mut reporte = HashMap::new();
        
        self.ventas.iter().for_each(|venta|{
            let monto_venta = self.calcular_precio_final(venta);
            let total_vendedor = reporte.entry(venta.vendedor.legajo).or_insert(0.0);
            *total_vendedor += monto_venta;
        });
        if !reporte.is_empty(){
            let mut res = Reporte::new();
            reporte.iter().for_each(|r|{
                res.agregar(ReporteVendedor::new(*r.0, *r.1));
            });
            return Some(res)
        }

        return None
    }
}

#[cfg(test)]
mod test_ejercicio4{
    use super::*;
    
    fn construir_sistema()->(Sistema,Vendedor){
        let mut sistema = Sistema::new("bienestar@mail.com", 50.0);
        
        let vendedor1 = Vendedor::new("Mariano","Sanchez", "Calle 123", 1234567, 891234, 1, 50000.0);

        sistema.configurar_descuento_categoria(Categorias::Alimento, 15.0);
        sistema.configurar_descuento_categoria(Categorias::Limpieza, 10.0);

        return (sistema,vendedor1)
    }

    fn retornar_productos()->Vec<ProductoVendido>{
        let mut lista = Vec::new();
        
        lista.push(ProductoVendido { producto: Producto::new("ASD", Categorias::Otro, 1000.0 ), cantidad: 5 });
        lista.push(ProductoVendido { producto: Producto::new("Limpiecito", Categorias::Limpieza, 2000.0 ), cantidad: 2 });
        lista.push(ProductoVendido { producto: Producto::new("MisterPapas", Categorias::Alimento, 5000.0 ), cantidad: 1 });
        lista.push(ProductoVendido { producto: Producto::new("Coso", Categorias::Bazar, 500.0 ), cantidad: 2 });

        return lista
    }

    #[test]
    fn registro_vendedores(){
        let mut muestra = construir_sistema();
        let vendedor2 = Vendedor::new("Mariana","Sanchez", "Calle 123", 1234567, 891231, 1, 50000.0);
        assert!(muestra.0.registrar_vendedor(muestra.1));
        assert!(muestra.0.registrar_vendedor(vendedor2.clone()));
        assert!(!muestra.0.registrar_vendedor(vendedor2));
    }

    #[test]
    fn registro_ventas(){
        let mut muestra = construir_sistema();
        let vendedor2 = Vendedor::new("Mariana","Sanchez", "Calle 123", 1234567, 891231, 1, 50000.0);
        muestra.0.registrar_vendedor(muestra.1.clone());

        let cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        let v1 = Venta::new("1/5/26", cli1.clone(), muestra.1,MediosDePago::Efectivo, retornar_productos());

        assert!(muestra.0.registrar_venta(v1));
        
        let v2 = Venta::new("1/5/26", cli1.clone(), vendedor2,MediosDePago::TransferenciaBancaria, retornar_productos());
        assert!(!muestra.0.registrar_venta(v2));

    }

    #[test]
    fn venta_sin_productos(){
        let mut muestra = construir_sistema();

        let cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        let v1 = Venta::new("1/5/26", cli1, muestra.1,MediosDePago::Efectivo, Vec::new());

        //Se tiene en cuenta que el sistema no concidera si la venta se registro en el sistema o no
        //Por lo que solo calcula el monto de una venta recibida 

        assert_eq!(muestra.0.calcular_precio_final(&v1),0.0);

    }

    #[test]
    fn venta_con_productos(){
        let mut muestra = construir_sistema();

        let cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        let v1 = Venta::new("1/5/26", cli1, muestra.1,MediosDePago::Efectivo, retornar_productos());

        assert_eq!(muestra.0.calcular_precio_final(&v1),13850.0);

    }

    #[test]
    fn venta_con_newsletter(){
        let mut muestra = construir_sistema();

        let mut cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        muestra.0.otorgar_newsletter(&mut cli1);

        let v1 = Venta::new("1/5/26", cli1, muestra.1,MediosDePago::Efectivo, retornar_productos());

        assert_eq!(muestra.0.calcular_precio_final(&v1),6925.0);

    }

    #[test]
    fn reporte_con_ventas(){
        let mut muestra = construir_sistema();

        let mut cli1 = Cliente::new("Damian","Goloso", "Av 5", 9823);
        let mut cli2 = Cliente::new("Fabio","Bordon", "Av 5", 4823);

        muestra.0.otorgar_newsletter(&mut cli1);

        let productos_base = retornar_productos();

        let l1 = productos_base[0..2].to_vec(); 
        let l2 = productos_base[1..4].to_vec();
        let l3 = productos_base.clone();

        let v1 = Venta::new("1/5/26", cli1.clone() , muestra.1.clone(),MediosDePago::Efectivo, l1);
        let v2 = Venta::new("1/5/26", cli2, muestra.1.clone(),MediosDePago::Efectivo, l2);
        let v3 = Venta::new("1/5/26", cli1, muestra.1.clone(),MediosDePago::Efectivo, l3);

        muestra.0.registrar_vendedor(muestra.1.clone());
        muestra.0.registrar_venta(v1);
        muestra.0.registrar_venta(v2);
        muestra.0.registrar_venta(v3);
        
        if let Some(reporte1) = muestra.0.reporte_ventas_por_categoria(){
            assert_eq!(reporte1.listado.len(),4);
            //Evaluar un monto para una categoria
            if let Some(categ) = reporte1.listado.iter().find(|c| c.categoria == Categorias::Limpieza){
                assert_eq!(categ.monto_total,7200.0);
            }else{
                panic!("No deberia de haber fallado");
            }
        }else{
            panic!("Se esperaba un informe");
        }

        if let Some(reporte2) = muestra.0.reporte_ventas_por_vendedor(){
            //Evaluar el monto del vendedor y su legajo
            assert_eq!(reporte2.listado.len(),1);
            assert_eq!(reporte2.listado[0].legajo_v,891234);
            assert_eq!(reporte2.listado[0].monto_total,20075.0);
        }else{
            panic!("Se esperaba un informe");
        }
        
    }

    #[test]
    fn reporte_sin_ventas(){
        let mut muestra = construir_sistema();

        muestra.0.registrar_vendedor(muestra.1);

        assert!(muestra.0.reporte_ventas_por_categoria().is_none(),"No tendria que haber retornado algo");
        assert!(muestra.0.reporte_ventas_por_vendedor().is_none(),"No tendria que haber retornado algo");
        
    }
    
}
