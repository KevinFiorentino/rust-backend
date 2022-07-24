// Función disponible en todo el scope de main.rs
fn my_function() {
    println!("Llamado número 4");
}

// Módulo al cual se puede hacer referencia con 'super' desde otro módulo
mod another_module {
    pub fn my_function() {
        println!("Llamado número 5");
    }
}

// Módulo principal de la app
mod my_module {

    // Función disponible en todo el scope de my_module
    fn my_function() {
        println!("Llamado número 2");
    }
    
    // Submódulo de my_module, lo utilizamos con 'self' dentro de my_module
    mod another_module {
        pub fn my_function() {
            println!("Llamado número 3");
        }
    }
    
    pub fn start_calls() {

        println!("Llamado número 1");
        
        // Llamamos a my_function, el 'self' aquí es opcional pero evita ambigüedades y problemas si otra función tiene el mismo nombre
        self::my_function();
        
        // Llamamos a my_function perteneciente a another_module y DENTRO de my_module
        self::another_module::my_function();
        
        // Llamamos a my_function disponible en todo el scope con 'super'
        super::my_function();
        
        // Llamamos my_function perteneciente a another_module y FUERA de my_module con 'super'
        super::another_module::my_function();

        // Otra forma de utilizar una función de un módulo externo
        {
            // Utilizamos la palabra reservada 'crate' para invocar librerías externas y darles un nombre con 'as'
            use crate::another_module::my_function as new_name_function;
            new_name_function();
        }
    }
}

fn main() {
    my_module::start_calls();
}