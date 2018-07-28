use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct LeftComponent {
}

impl LeftComponent {
  pub fn new() -> LeftComponent {
    LeftComponent {}
  }
}

impl<'a> Component<'a> for LeftComponent {
  fn render(&'a mut self) -> HtmlToken<'a> {
    // let state = self.clone();
    jsx!(<div>
      left
    </div>)
  }
}