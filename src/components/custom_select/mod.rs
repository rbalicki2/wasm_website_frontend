mod stateless_select;

use std::rc::Rc;
use std::cell::RefCell;
use std::vec::IntoIter;
use jsx_macro::*;
use jsx_types::*;

use web_sys::console::log_1;
use wasm_bindgen::JsValue;

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

fn clone_many_times<T>(n: usize, cell: &T) -> Vec<T> where T: Clone {
  let mut vec = Vec::with_capacity(n);
  for _ in 0..n {
    vec.push(cell.clone());
  }
  vec
}

pub use self::stateless_select::*;

pub struct CustomSelectProps<'a> {
  pub items: &'a Vec<String>,
  pub on_click: Box<'a + Fn(String) -> ()>,
}

impl<'a> Component<'a, CustomSelectProps<'a>> for CustomSelect {
  fn render(&'a mut self, mut props: CustomSelectProps<'a>) -> HtmlToken<'a> {
    let is_open = self.is_open;
    let cell_1 = Rc::new(RefCell::new(&mut self.is_open));
    let cell_2 = cell_1.clone();
    let items = props.items.clone();
    let on_click_cell = Rc::new(RefCell::new(props.on_click));
    let on_click_cells = clone_many_times(items.len(), &on_click_cell);

    let maybe_items = {
        if is_open {
          Some(
            items
              .into_iter()
              .zip(on_click_cells.into_iter())
              .map(|(item, on_click_cell)| {
                let cell_2 = cell_2.clone();
                // let item_2 = item.clone();
                let item_cell = Rc::new(RefCell::new(item.clone()));

                let on_click_cell = on_click_cell.clone();
                jsx!(
                  <div
                    on_click={Box::new(move |_| {
                      let mut is_open_cell = cell_2.borrow_mut();
                      **is_open_cell = false;
                      let on_click = on_click_cell.borrow_mut();
                      let item = item_cell.borrow();
                      on_click(item.to_string());
                    })}
                  >
                    {item}
                  </div>
                )
              })
              .collect::<Vec<HtmlToken>>()
          )
        } else {
          None
        }
      };

    jsx!(<div
      // on_click={Box::new(move |_| {
      //   let mut on_click = on_click_cell.borrow_mut();
      //   log_1(&JsValue::from_str("inner on click handler"));
      //   on_click("some param from inside to outside".into());
      // })}
    >
      outer
      <h1>custom select</h1>
      <button
        on_click={Box::new(move |_| {
          let mut is_open_cell = cell_1.borrow_mut();
          **is_open_cell = !is_open;
        })}
      >
        toggle
      </button>
      { maybe_items }
    </div>)
  }
}
