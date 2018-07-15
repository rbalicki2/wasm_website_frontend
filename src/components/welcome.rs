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
    jsx!(<div><div>this is <h1>welcome</h1></div></div>)
  }
}