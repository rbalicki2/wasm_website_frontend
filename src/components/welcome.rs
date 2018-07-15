use jsx_macro::*;
use jsx_types::*;

use std::boxed::FnBox;

pub struct Welcome {}

impl Welcome {
  pub fn new() -> Welcome {
    Welcome {}
  }
}

impl Component for Welcome {
  fn render(&mut self) -> HtmlToken {
    let click_handler: Box<FnBox(Event) -> ()> = Box::new(|_| {});
    jsx!(<div><h1 OnClick={click_handler}>welcome</h1></div>)
  }
}