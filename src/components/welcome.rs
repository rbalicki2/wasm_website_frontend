use jsx_macro::*;
use jsx_types::*;

// use std::boxed::FnBox;

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

impl<'a> Component<'a> for Welcome {
  fn render(&'a mut self) -> HtmlToken<'a> {
    let click_handler: Box<FnMut(Event) -> ()> = Box::new(|_| {
      // self.click_count += 1;
    });
    jsx!(<div><h1 OnClick={click_handler}>Ive been clicked {self.click_count}{' '} times</h1></div>)
  }
}