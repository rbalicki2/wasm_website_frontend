#![feature(proc_macro, wasm_custom_section, wasm_import_module, proc_macro_non_items)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate jsx_types;
use jsx_types::*;

extern crate jsx_macro;
use jsx_macro::jsx;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  let dom = jsx!(<h3 />);
  match dom {
    HtmlToken::Text(t) => alert(&t),
    HtmlToken::DomElement(d) => alert(&format!("This is a {}", d.node_type)),
  }
}
