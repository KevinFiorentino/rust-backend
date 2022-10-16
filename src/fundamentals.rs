use std::collections::{HashMap};

/* use csv::{ReaderBuilder};
use std::{fs};

const FILENAME: &str = "my_file.csv"; */

fn main() {

    /* ********** Variables ********** */

    /* let name: &str = "Kevin";
    let mut a: i16 = 100;
    let b: u16 = 100;

    a = a + 1;

    println!("Mi nombre es {}", name);
    println!("El número es {} y {}", a, b); */


    /* ********** Inputs por Consola ********** */

    /* println!("Ingrese su nombre:");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("Bienvenido o bienvenida: {}", nombre);

    println!("********************");

    println!("Ingrese su edad:");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("La edad es: {}", edad_int); */


    /* ********** Condicionales ********** */

    /* let edad: i8 = 20;

    if edad > 18 {
        println!("Eres mayor de edad");
    } else {
        println!("Eres mayor de edad");
    } */


    /* ********** Loop ********** */

    /* loop {
        println!("Ingrese 123 para detener el loop:");
        let mut number: String = String::new();
        std::io::stdin().read_line(&mut number).unwrap();
        let number_int: i32 = number.trim().parse().unwrap();

        if number_int == 123 {
            break;
        }
    } */


    /* ********** Array ********** */

    /* let mut my_arr: Vec<String> = Vec::new();

    my_arr.push("¡Bienvenido".to_string());
    my_arr.push("a".to_string());
    my_arr.push("Platzi!".to_string());

    println!("Length: {}", my_arr.len());
    println!("{:?}", my_arr); */


    /* ********** FOR ********** */

    /* for word in my_arr {
        println!("{}", word);
    } */


    /* ********** Functions ********** */

    /* let sum = sumar_numeros(10, 10);
    println!("{}", sum); */


    /* ********** Lectura de Archivos ********** */
    /* let content: String = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        println!("{}", result.unwrap().get(0).unwrap().trim());
    } */


    /* ********** Default Error ********** */

    /* println!("Ingrese su edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_int: u8 = edad.trim().parse().unwrap_or(18);

    println!("Tienes {} años: ", edad_int); */


    /* ********** Structs ********** */

    /* let persona: Person = Person::new("Kevin".to_string(), "Fiorentino".to_string(), 27, "Argentina".to_string());

    println!("{}", persona.nombre);
    println!("{}", persona.apellido);
    println!("{}", persona.edad);
    println!("{}", persona.pais); */


    /* ********** HashMap ********** */

    /* let mut diccionario: HashMap<&str, &str> = HashMap::new();

    diccionario.insert("Manzana", "La manzana es roja.");
    diccionario.insert("Banana", "La banana es amarilla.");
    diccionario.insert("Naranja", "La naranja es... naranja.");

    println!("{}", diccionario["Manzana"]); */




    /* ********** Índice de Array ********** */

    /* let mut my_arr: Vec<String> = Vec::new();

    my_arr.push("Primer valor".to_string());
    my_arr.push("Segundo valor".to_string());
    my_arr.push("Tercer valor".to_string());

    for (index, word) in my_arr.iter().enumerate() {
        println!("{} {}", index, word);
    } */


    /* /* ********** Option, Some y None ********** */

    /* if let Some(result) = dividir_numeros(10, 2) {
        println!("{}", result)
    } else {
        println!("No puedes dividir por cero");
    } */ */

    let result = dividir_numeros(10, 2);

    match result {
        Some(value) => { println!("{}", value) },
        None => { println!("No puedes dividir por cero"); },
    }

}

fn dividir_numeros(numerador: i128, denominador: i128) -> Option<i128> {
    if denominador == 0 {
        None
    } else {
        Some(numerador / denominador)
    }
}

/* fn sumar_numeros(num1: i32, num2: i32) -> i32 {
    let sum: i32 = num1 + num2;
    return sum;
} */

/* struct Person {
    nombre: String,
    apellido: String,
    edad: i32,
    pais: String,
}

impl Person {
    fn new(nombre: String, apellido: String, edad: i32, pais: String) -> Person {
        return Person { nombre, apellido, edad, pais };
    }
} */
