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

pub struct LeftComponentProps<'a> {
  pub magic_string: String,
  pub set_magic_string: Box<FnMut(String) -> () +'a>,
}

impl<'a> Component<'a, LeftComponentProps<'a>> for LeftComponent {
  fn render(&'a mut self, props: LeftComponentProps<'a>) -> HtmlToken<'a> {
    let magic_string = props.magic_string;
    let mut set_magic_string = props.set_magic_string;
    let update_text: Box<events::InputEventHandler<'a>> = Box::new(move |e| {
      if let Some(ref val) = e.value {
        set_magic_string(val.clone());
      }
    });
    // let state = self.clone();
    jsx!(<div>
      left: { magic_string.clone() }
      <input value={magic_string } on_input={update_text} />
    </div>)
  }
}