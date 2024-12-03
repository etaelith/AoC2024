use std::io;

fn main() {
    println!("hola");
    input();
}

fn input() {
    println!("Introduce la lista:");
    let mut entrada = String::new();
    let mut reports_safe: i16 = 0;
    loop {
        let mut linea = String::new();
        io::stdin()
            .read_line(&mut linea)
            .expect("Error al leer la linea");
        if linea.trim().is_empty() {
            break;
        }
        entrada.push_str(&linea);
    }
    let vectores: Vec<Vec<i32>> = entrada
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
    for (_i, vector) in vectores.iter().enumerate() {
        if is_sorted_ascending_with_diff(vector) {
            reports_safe += 1;
        } else if is_sorted_descending_with_diff(vector) {
            reports_safe += 1;
        }
    }
    println!("La cantidad de reportes safe es: {}", reports_safe);
}

fn is_sorted_ascending_with_diff(vector: &[i32]) -> bool {
    vector
        .windows(2)
        .all(|pair| pair[0] < pair[1] && (pair[1] - pair[0] <= 3))
}
fn is_sorted_descending_with_diff(vector: &[i32]) -> bool {
    vector
        .windows(2)
        .all(|pair| pair[0] > pair[1] && (pair[0] - pair[1] <= 3))
}
