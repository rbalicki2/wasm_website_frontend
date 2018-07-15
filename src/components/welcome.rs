use jsx_macro::*;
use jsx_types::*;

pub struct Welcome {}

impl Welcome {
  pub fn new() -> Welcome {
    Welcome {}
  }
}

impl Component for Welcome {
  fn render(&self) -> HtmlToken {
    let click_handler: Box<FnOnce(Event) -> ()> = Box::new(|_| {});
    jsx!(<div>hihihi<h1 OnClick={click_handler}>welcome</h1></div>)
  }
}