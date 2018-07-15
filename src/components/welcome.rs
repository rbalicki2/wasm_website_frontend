use jsx_macro::*;
use jsx_types::*;

use std::boxed::FnBox;

pub struct Welcome {
  click_count: i32,
}

impl Welcome {
  pub fn new() -> Welcome {
    Welcome {
      click_count: 0,
    }
  }
}

impl Component for Welcome {
  fn render(&mut self) -> HtmlToken {
    let click_handler: Box<FnBox(Event) -> ()> = Box::new(|_| {});
    jsx!(<div><h1 OnClick={click_handler}>Ive been clicked {self.click_count} times</h1></div>)
  }
}