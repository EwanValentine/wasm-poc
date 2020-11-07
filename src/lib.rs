mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(name: &str) -> String { 
    format!("Hello {}", name)
}

#[wasm_bindgen]
pub struct Counter {
  val: u32,
}

#[wasm_bindgen]
impl Counter {

  #[wasm_bindgen(constructor)]
  pub fn new() -> Counter {
    Counter {
      val: 0,
    }
  }

  #[wasm_bindgen(method)]
  pub fn increment(&mut self) {
    self.val += 1;
  }

  #[wasm_bindgen(method)]
  pub fn decrement(&mut self) {
    self.val -= 1;
  }

  #[wasm_bindgen(method)]
  pub fn get_value(&self) -> u32 {
    self.val
  }
}


#[wasm_bindgen]
pub struct Data {
  data: Vec<u8>,
}

#[wasm_bindgen]
impl Data {

    #[wasm_bindgen]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub fn process(&mut self, data: &mut [u8]) -> [u8] {
        self.data
    }
}
