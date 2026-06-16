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
    pub fn new(producto: &Producto, cantidad: u64) -> ProductoVendido {
        ProductoVendido{
            producto: producto.clone(),
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

#[derive(PartialEq,Debug,Clone)]
struct Cliente {
    datos: DatosPersona,
    correo_newsletter: Option<String>,
}

impl Cliente {
    pub fn new(nombre: &str, apellido: &str, direccion: &str, dni: u64) -> Cliente {
        Cliente {
            datos: DatosPersona {
                nombre: nombre.to_string(),
                apellido: apellido.to_string(),
                direccion: direccion.to_string(),
                dni,
            },
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
            datos: DatosPersona {
                nombre: nombre.to_string(),
                apellido: apellido.to_string(),
                direccion: direccion.to_string(),
                dni,
            },
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
    pub fn new(fecha: &str, cliente: &Cliente, vendedor: &Vendedor, medio_pago: &MediosDePago, productos: &Vec<ProductoVendido>) -> Venta {
        Venta{
            fecha: fecha.to_string(),
            cliente: cliente.clone(),
            vendedor: vendedor.clone(),
            medio_pago: medio_pago.clone(),
            productos: productos.clone(),
        }
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

    pub fn registrar_vendedor(&mut self,v:&Vendedor)->bool{
        let mut exito = false;
        
        if !self.vendedores.iter().any(|vendedor| vendedor.legajo == v.legajo){
            self.vendedores.push(v.clone());
            exito = true;
        }

        return exito
    }

    pub fn registrar_venta(&mut self, venta: &Venta)->bool{
        let mut exito = false;
        if self.vendedores.iter().any(|vendedor| vendedor.legajo == venta.vendedor.legajo){
            self.ventas.push(venta.clone());
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
        /* 
        for item in &venta.productos {
            let descuento_categoria = if let Some(porcentaje) = self.descuentos_categorias.get(&item.producto.categoria){*porcentaje}else{0.0};

            let precio_con_descuento = item.producto.precio_base * (1.0 - (descuento_categoria / 100.0));
            
            subtotal_venta += precio_con_descuento * (item.cantidad as f64);
        }
        */
        if venta.cliente.tiene_newsletter() {
            subtotal_venta *= 1.0 - (self.descuento_newsletter / 100.0);
        }

        subtotal_venta
    }

    pub fn reporte_ventas_por_categoria(&self) -> Vec<(Categorias, f64)> {
        let mut reporte = HashMap::new();
        
        self.ventas.iter().flat_map(|venta|{ 
            let tiene_descuento = venta.cliente.tiene_newsletter();
            venta.productos.iter().map(move|p|(tiene_descuento,p))
        }).for_each(|(descuento_general,p)|{
            let descuento_categoria = if let Some(porcentaje) = self.descuentos_categorias.get(&p.producto.categoria){*porcentaje}else{0.0};
            let precio_final_unitario = p.producto.precio_base * (1.0 - (descuento_categoria / 100.0));
            let mut monto_item = precio_final_unitario * (p.cantidad as f64);
                
            // Si la venta global tuvo descuento por newsletter, impacta proporcionalmente al item
            if descuento_general {
                monto_item *= 1.0 - (self.descuento_newsletter / 100.0);
            }

            let total_cat = reporte.entry(p.producto.categoria.clone()).or_insert(0.0);
            *total_cat += monto_item;
        });
        /* 
        for venta in &self.ventas {
            for item in &venta.productos {
                let descuento_categoria = if let Some(porcentaje) = self.descuentos_categorias.get(&item.producto.categoria){*porcentaje}else{0.0};
                let precio_final_unitario = item.producto.precio_base * (1.0 - (descuento_categoria / 100.0));
                let mut monto_item = precio_final_unitario * (item.cantidad as f64);
                
                // Si la venta global tuvo descuento por newsletter, impacta proporcionalmente al item
                if venta.cliente.tiene_newsletter() {
                    monto_item *= 1.0 - (self.descuento_newsletter / 100.0);
                }

                let total_cat = reporte.entry(item.producto.categoria.clone()).or_insert(0.0);
                *total_cat += monto_item;
            }
        }*/

        let vector_reporte : Vec<(Categorias, f64)> = reporte.into_iter().collect();
        return vector_reporte
    }

    //Consultar por la forma de entrega del reporte (por vendedor o legajo)
    pub fn reporte_ventas_por_vendedor(&self) -> Vec<(u64, f64)> {
        let mut reporte = HashMap::new();
        
        for venta in &self.ventas {
            let monto_venta = self.calcular_precio_final(venta);
            let total_vendedor = reporte.entry(venta.vendedor.legajo).or_insert(0.0);
            *total_vendedor += monto_venta;
        }

        let vector_reporte : Vec<(u64, f64)> = reporte.into_iter().collect();
        return vector_reporte
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
        assert!(muestra.0.registrar_vendedor(&muestra.1));
        assert!(muestra.0.registrar_vendedor(&vendedor2));
        assert!(!muestra.0.registrar_vendedor(&vendedor2));
    }

    #[test]
    fn registro_ventas(){
        let mut muestra = construir_sistema();
        let vendedor2 = Vendedor::new("Mariana","Sanchez", "Calle 123", 1234567, 891231, 1, 50000.0);
        muestra.0.registrar_vendedor(&muestra.1);

        let cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        let v1 = Venta::new("1/5/26", &cli1, &muestra.1,&MediosDePago::Efectivo, &retornar_productos());

        assert!(muestra.0.registrar_venta(&v1));
        
        let v2 = Venta::new("1/5/26", &cli1, &vendedor2,&MediosDePago::TransferenciaBancaria, &retornar_productos());
        assert!(!muestra.0.registrar_venta(&v2));

    }

    #[test]
    fn venta_sin_productos(){
        let mut muestra = construir_sistema();

        let cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        let v1 = Venta::new("1/5/26", &cli1, &muestra.1,&MediosDePago::Efectivo, &Vec::new());

        //Se tiene en cuenta que el sistema no concidera si la venta se registro en el sistema o no
        //Por lo que puede calcular el monto de una venta recibida 

        assert_eq!(muestra.0.calcular_precio_final(&v1),0.0);

    }

    #[test]
    fn venta_con_productos(){
        let mut muestra = construir_sistema();

        let cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        let v1 = Venta::new("1/5/26", &cli1, &muestra.1,&MediosDePago::Efectivo, &retornar_productos());

        assert_eq!(muestra.0.calcular_precio_final(&v1),13850.0);

    }

    #[test]
    fn venta_con_newsletter(){
        let mut muestra = construir_sistema();

        let mut cli1 = Cliente::new("Juan","Golosito", "Av 5", 987123);

        muestra.0.otorgar_newsletter(&mut cli1);

        let v1 = Venta::new("1/5/26", &cli1, &muestra.1,&MediosDePago::Efectivo, &retornar_productos());

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

        let v1 = Venta::new("1/5/26", &cli1, &muestra.1,&MediosDePago::Efectivo, &l1);
        let v2 = Venta::new("1/5/26", &cli2, &muestra.1,&MediosDePago::Efectivo, &l2);
        let v3 = Venta::new("1/5/26", &cli1, &muestra.1,&MediosDePago::Efectivo, &l3);

        muestra.0.registrar_vendedor(&muestra.1);
        muestra.0.registrar_venta(&v1);
        muestra.0.registrar_venta(&v2);
        muestra.0.registrar_venta(&v3);

        assert_eq!(muestra.0.reporte_ventas_por_categoria().len(),4);
        assert_eq!(muestra.0.reporte_ventas_por_vendedor().len(),1);
        
    }

    #[test]
    fn reporte_sin_ventas(){
        let mut muestra = construir_sistema();

        muestra.0.registrar_vendedor(&muestra.1);

        assert_eq!(muestra.0.reporte_ventas_por_categoria().len(),0);
        assert_eq!(muestra.0.reporte_ventas_por_vendedor().len(),0);
        
    }
    
}
