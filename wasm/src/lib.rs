use wasm_bindgen::prelude::*;

// Indicamos a Rust que utilizaremos la función `alert()` de Javascript.
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// Función expuesta que utilizaremos desde el navegador
#[wasm_bindgen]
pub fn saludar(nombre: &str) {
    alert(&format!("Hola, {}, ¿como estas?", nombre))   
}
