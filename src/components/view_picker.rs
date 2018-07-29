use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::View;

#[derive(Clone)]
pub struct ViewPicker {}

pub struct ViewPickerProps<'a> {
  pub view: View,
  pub on_select_view: Box<FnMut(View) -> () + 'a>,
}

impl<'a> StatelessComponent<'a, ViewPickerProps<'a>> for ViewPicker {
  fn render(props: ViewPickerProps<'a>) -> HtmlToken<'a> {
    let view = props.view;
    let cell = Rc::new(RefCell::new(props));
    let cell_2 = cell.clone();
    let cell_3 = cell.clone();

    let set_all: Box<events::MouseEventHandler<'a>> = Box::new(move |_| {
      let mut props = cell.borrow_mut();
      (&mut props.on_select_view)(View::All)
    });

    let set_done: Box<events::MouseEventHandler<'a>> = Box::new(move |_| {
      let mut props = cell_2.borrow_mut();
      (&mut props.on_select_view)(View::Done)
    });

    let set_incomplete: Box<events::MouseEventHandler<'a>> = Box::new(move |_| {
      let mut props = cell_3.borrow_mut();
      (&mut props.on_select_view)(View::Incomplete)
    });

    jsx!(<div class="mt-2 btn-group">
      <button
        class={format!("btn btn-primary {}", if view == View::All {"active"} else {""})}
        on_click={set_all}
      >
        All
      </button>
      <button
        class={format!("btn btn-primary {}", if view == View::Done {"active"} else {""})}
        on_click={set_done}
      >
        Done
      </button>
      <button
        class={format!("btn btn-primary {}", if view == View::Incomplete {"active"} else {""})}
        on_click={set_incomplete}
      >
        Incomplete
      </button>
    </div>)
  }
}
