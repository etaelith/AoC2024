use std::io;
fn main() {
    println!("Introduce una lista de numeros separados por espacios:");

    let mut entrada = String::new();
    loop {
        let mut linea = String::new();
        io::stdin()
            .read_line(&mut linea)
            .expect("Error al leer la línea");
        if linea.trim().is_empty() {
            break; // Salir del bucle al encontrar una línea vacía
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

    println!("Lista izquierda ordenada: {:?}", izquierda);
    println!("Lista derecha ordenada: {:?}", derecha);
}
