#![feature(proc_macro_non_items)]

extern crate smithy;
extern crate jsx_types;
extern crate jsx_macro;

extern crate web_sys;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod components;

#[wasm_bindgen]
pub fn start() {
  let component = components::AppState::new();

  smithy::mount("app", Box::new(component));
}
