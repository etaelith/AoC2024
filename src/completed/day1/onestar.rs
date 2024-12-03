use std::io;
fn onestar() {
    println!("Introduce una lista de numeros separados por espacios:");

    let mut entrada = String::new();
    let mut vec: Vec<i32> = Vec::new();
    loop {
        let mut linea = String::new();
        io::stdin()
            .read_line(&mut linea)
            .expect("Error al leer la línea");
        if linea.trim().is_empty() {
            break;
        }
        entrada.push_str(&linea);
    }
    let numeros: Vec<i32> = entrada
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Error al convertir el numero"))
        .collect();

    let (mut izquierda, mut derecha): (Vec<i32>, Vec<i32>) =
        numeros.chunks(2).map(|chunk| (chunk[0], chunk[1])).unzip();

    izquierda.sort();
    derecha.sort();
    for i in 0..izquierda.len() {
        let izq = izquierda[i];
        let der = derecha[i];
        let diferencia = der - izq;
        vec.push(diferencia.abs());
    }
    println!("Izquierda: {:?}", izquierda);
    println!("Derecha: {:?}", derecha);
    println!(
        "Length izquierda: {:?}\n Length derecha: {:?}",
        izquierda.len(),
        derecha.len()
    );
    let suma: i32 = vec.iter().sum();
    println!("Diferencias: {:?}", suma)
}
