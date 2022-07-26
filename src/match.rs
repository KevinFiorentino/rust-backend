fn main() {

    let edad = 18;

    let resultado = match edad {
        0..=16 => "Entre 0 y 16",
        17 => "El valor es 17",
        18 => "El valor es 18",
        19 | 20 | 21 => "El valor es 19, 20 o 21",
        _ => "Valor por defecto"
    };

    println!("{}", resultado);

}