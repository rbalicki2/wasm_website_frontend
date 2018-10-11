mod stateless_select;

use std::rc::Rc;
use std::cell::RefCell;
use jsx_macro::*;
use jsx_types::*;

// use web_sys::console::log_1;
// use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct CustomSelect {
  is_open: bool,
}

impl CustomSelect {
  pub fn new() -> Self {
    CustomSelect {
      is_open: false,
    }
  }
}

pub use self::stateless_select::*;

pub struct CustomSelectProps<'a> {
  pub items: &'a Vec<String>,
  pub on_click: Box<'a + FnMut(String) -> ()>,
}

impl<'a> Component<'a, CustomSelectProps<'a>> for CustomSelect {
  fn render(&'a mut self, mut props: CustomSelectProps<'a>) -> HtmlToken<'a> {
    let is_open = self.is_open;
    let cell_1 = Rc::new(RefCell::new(&mut self.is_open));
    let cell_2 = cell_1.clone();
    let items = props.items.clone();
    let on_click_cell = Rc::new(RefCell::new(&mut props.on_click));

    jsx!(<div>
      <h1>custom select</h1>
      <button
        on_click={Box::new(move |_| {
          let mut is_open_cell = cell_1.borrow_mut();
          **is_open_cell = !is_open;
        })}
      >
        toggle
      </button>
      {
        if is_open {
          Some(items.into_iter().map(|item| {
            let cell_2 = cell_2.clone();
            let item_2 = item.clone();
            let on_click_cell = on_click_cell.clone();
            let component = jsx_verbose!(
              <div
                on_click={Box::new(move |_| {
                  let mut is_open_cell = cell_2.borrow_mut();
                  **is_open_cell = false;
                  // let mut on_click = on_click_cell.borrow_mut();
                  // on_click(item_2);
                })}
              >
                {item}
              </div>
            );
            component
            // jsx!(<div />);
          }).collect::<Vec<HtmlToken>>())
        } else {
          None
        }
      }
    </div>)
  }
}
