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
    pub fn new(fecha: &str, cliente: &Cliente, vendedor: &Vendedor, medio_pago: MediosDePago, productos: Vec<ProductoVendido>) -> Venta {
        Venta{
            fecha: fecha.to_string(),
            cliente: cliente.clone(),
            vendedor: vendedor.clone(),
            medio_pago,
            productos,
        }
    }
}

// El sistema mantiene el registro de todo y contiene los datos necesarios
struct Sistema {
    ventas: Vec<Venta>,
    vendedores : Vec<Vendedor>,
    productos : Vec<Producto>,
    descuentos_categorias: HashMap<Categorias, f64>, // Lista de categorías con descuento
    newsletter : String,
    descuento_newsletter: f64,                      // Porcentaje general por newsletter
}

impl Sistema {
    pub fn new(correo:&str,descuento_newsletter: f64) -> Self {
        Self {
            ventas: Vec::new(),
            vendedores : Vec::new(),
            productos : Vec::new(),
            descuentos_categorias: HashMap::new(),
            newsletter: correo.to_string(),
            descuento_newsletter,
        }
    }

    pub fn configurar_descuento_categoria(&mut self, categoria: Categorias, porcentaje: f64) {
        self.descuentos_categorias.insert(categoria, porcentaje);
    }

    pub fn registrar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }

    // ACCIÓN: Calcular el precio final de una venta aplicando las reglas correctamente
    pub fn calcular_precio_final(&self, venta: &Venta) -> f64 {
        let mut subtotal_venta = 0.0;

        for item in &venta.productos {
            // 1. Buscamos si la categoría del producto tiene descuento en el sistema
            let porc_desc_cat = self.descuentos_categorias.get(&item.producto.categoria).unwrap_or(&0.0);
            
            // 2. Calculamos el precio unitario con el descuento de categoría aplicado
            let precio_con_desc_cat = item.producto.precio_base * (1.0 - (porc_desc_cat / 100.0));
            
            // 3. Acumulamos multiplicando por la cantidad
            subtotal_venta += precio_con_desc_cat * (item.cantidad as f64);
        }

        // 4. Si el cliente tiene suscripción al newsletter, aplicamos el descuento general sobre el acumulado
        if venta.cliente.tiene_newsletter() {
            subtotal_venta *= 1.0 - (self.descuento_newsletter / 100.0);
        }

        subtotal_venta
    }

    // ACCIÓN: Reporte para visualizar las ventas totales por categoría de producto
    pub fn reporte_ventas_por_categoria(&self) -> HashMap<Categorias, f64> {
        let mut reporte = HashMap::new();
        
        for venta in &self.ventas {
            for item in &venta.productos {
                let porc_desc_cat = self.descuentos_categorias.get(&item.producto.categoria).unwrap_or(&0.0);
                let precio_final_unitario = item.producto.precio_base * (1.0 - (porc_desc_cat / 100.0));
                let mut monto_item = precio_final_unitario * (item.cantidad as f64);
                
                // Si la venta global tuvo descuento por newsletter, impacta proporcionalmente al item
                if venta.cliente.tiene_newsletter() {
                    monto_item *= 1.0 - (self.descuento_newsletter / 100.0);
                }

                let total_cat = reporte.entry(item.producto.categoria.clone()).or_insert(0.0);
                *total_cat += monto_item;
            }
        }
        reporte
    }

    // ACCIÓN: Reporte para visualizar las ventas totales por vendedor (identificado por legajo)
    pub fn reporte_ventas_por_vendedor(&self) -> HashMap<u64, f64> {
        let mut reporte = HashMap::new();
        
        for venta in &self.ventas {
            let monto_venta = self.calcular_precio_final(venta);
            let total_vendedor = reporte.entry(venta.vendedor.legajo).or_insert(0.0);
            *total_vendedor += monto_venta;
        }
        reporte
    }
}

/*
    CORREGIR CODIGO
    +revisar metodo de aplicar porcentaje de los descuentos
    +en como se verifican la existencia de los datos y la consulta de los mismos
    +como se procesan los datos
*/

#[cfg(test)]
mod test_ejercicio4{    
    use super::*;
    #[test]
    fn operar_producto(){
        let p = Producto::new(&"Shampoo".to_string(),&Categorias::default(),3500.0);
        assert_eq!(p.obtener_precio_sin_descuento(),3500.0);
        assert_eq!(p.obtener_precio_con_descuento(),3500.0);
        assert!(p.categoria_igual_a(&Categorias::default()));
        let p = Producto::new(&"Shampoo".to_string(),&Categorias::Limpieza(50.0),3500.0);
        assert_eq!(p.obtener_precio_sin_descuento(),3500.0);
        assert_eq!(p.obtener_precio_con_descuento(),1750.0);
        assert!(p.categoria_igual_a(&Categorias::Limpieza(50.0)));
    }

    #[test]
    fn validar_informacion(){
        let cli1 = Cliente::new(&"Marcos".to_string(), &"Lupe".to_string(), &"Av1".to_string(), 124341);
        let vendedor1 = Vendedor::new(&"Julieta".to_string(), &"Murias".to_string(), &"Cantilo".to_string(), 645634, 1234,1, 10000.0);
        assert!(cli1.datos_cliente.validar_datos(&cli1.datos_cliente));
        assert!(vendedor1.datos_vendedor.validar_datos(&vendedor1.datos_vendedor));
        assert!(!cli1.datos_cliente.validar_datos(&vendedor1.datos_vendedor));
        assert!(!vendedor1.datos_vendedor.validar_datos(&cli1.datos_cliente));
    }

    #[test]
    fn operar_venta(){
        //Personas
        let cli1 = Cliente::new(&"Lucas".to_string(), &"Daniel".to_string(), &"AvBelgrano".to_string(), 871265);
        let ven1 = Vendedor::new(&"Tobias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0);

        //Productos registrados
        let p1 = Producto::new(&"CocaCola".to_string(), &Categorias::Alimento(25.0), 3500.0);
        let p2 = Producto::new(&"Escoba".to_string(), &Categorias::Limpieza(30.0), 1000.0);
        let p3 = Producto::new(&"ElEjemplo".to_string(), &Categorias::Bazar(0.0), 1500.0);

        //Generar ventas
        let mut v1 = Venta::new(&Fecha::new(05, 02, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v1.agregar_producto(&ProductoVendido::new(&p1, 2));
        v1.agregar_producto(&ProductoVendido::new(&p2, 1));
        v1.agregar_producto(&ProductoVendido::new(&p3, 5));

        //Retorno de monto total sin descuento (ya que cli1 no tiene newsletter)
        assert_eq!(v1.monto_total(),15500.0);
    }

    #[test]
    fn operar_sistema(){
        //Sistema
        let mut sis = Sistema::new(&CategPorcentajes(0.0, 60.0, 40.0, 0.0), &"correo@example.com".to_string());

        //Personas
        let cli1 = Cliente::new(&"Lucas".to_string(), &"Daniel".to_string(), &"AvBelgrano".to_string(), 871265);
        let mut cli2 = Cliente::new(&"Mariana".to_string(), &"Santos".to_string(), &"Centenario".to_string(), 2987865);
        //Otorga cli2 el newsletter por parte del sistema
        sis.otorgar_newsletter(&mut cli2);
        let ven1 = Vendedor::new(&"Tobias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0);

        //Productos registrados
        let p1 = Producto::new(&"CocaCola".to_string(), &sis.definir_categoria(&Categorias::Alimento(0.0)), 3500.0);
        let p2 = Producto::new(&"Escoba".to_string(), &sis.definir_categoria(&Categorias::Limpieza(0.0)), 1000.0);
        let p3 = Producto::new(&"ElEjemplo".to_string(), &Categorias::Bazar(0.0), 1500.0);

        //Generar ventas
        let mut v1 = Venta::new(&Fecha::new(05, 02, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v1.agregar_producto(&ProductoVendido::new(&p1, 2));
        v1.agregar_producto(&ProductoVendido::new(&p2, 1));
        v1.agregar_producto(&ProductoVendido::new(&p3, 5));

        let mut v2 = Venta::new(&Fecha::new(15, 6, 2025), &cli2, &ven1, &MediosDePago::TarjetaDébito);
        v2.agregar_producto(&ProductoVendido::new(&p1, 1));
        v2.agregar_producto(&ProductoVendido::new(&p2, 2));
        v2.agregar_producto(&ProductoVendido::new(&p3, 3));
        
        let v3 = Venta::new(&Fecha::new(25, 8, 2025), &cli1, &ven1, &MediosDePago::TarjetaDébito);  //Sin productos y sin registrar en el sistema

        //Operar en el sistema
        assert!(sis.registrar_vendedor(&ven1));
        assert!(sis.registrar_producto(&p1));
        assert!(sis.registrar_producto(&p2));
        assert!(sis.registrar_producto(&p3));

        assert!(sis.registrar_venta(&v1));
        assert!(sis.registrar_venta(&v2));


        //Retorno de ventas por categorias
        let res = sis.retornar_ventas_por_categoria(&sis.definir_categoria(&Categorias::Otro(0.0)));
        assert!(res.is_empty());

        let res = sis.retornar_ventas_por_categoria(&sis.definir_categoria(&Categorias::Alimento(0.0)));
        assert!(!res.is_empty());

        //Retorno de ventas por vendedor
        let res = sis.retornar_ventas_por_vendedor(&ven1);
        assert!(!res.is_empty());

        let res = sis.retornar_ventas_por_vendedor(&Vendedor::new(&"Matias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0));
        assert!(res.is_empty());

        //Retorno monto final (sin descuento)
        if let Some(monto) = sis.monto_final_venta(&v1){
            assert_eq!(monto,15500.0);
        }

        //Retorno monto final (con descuento)
        if let Some(monto) = sis.monto_final_venta(&v2){
            assert_eq!(monto,9200.0);
        }

        //Retorno nulo de una venta no registrada
        assert!(sis.monto_final_venta(&v3).is_none());
    }
}