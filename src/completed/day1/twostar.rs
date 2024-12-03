use std::{collections::HashMap, io};
fn twostar() {
    println!("Introduce una lista de numeros separados por espacios:");

    let mut entrada = String::new();
    let mut vec: Vec<i32> = Vec::new();
    let mut cantidad = HashMap::new();
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

    for num in derecha {
        *cantidad.entry(num).or_insert(0) += 1;
    }

    for num in izquierda {
        let count = cantidad.get(&num).unwrap_or(&0);
        vec.push(num * count)
    }

    let suma: i32 = vec.iter().sum();
    println!("Diferencias: {:?}", suma)
}
