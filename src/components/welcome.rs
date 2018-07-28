use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Welcome {
  click_count: i32,
  is_hovering: bool,
  text_input: String,
}

impl Welcome {
  pub fn new() -> Welcome {
    Welcome {
      click_count: 0,
      is_hovering: false,
      text_input: "foo".to_string(),
    }
  }
}

impl<'a> Component<'a> for Welcome {
  fn render(&'a mut self) -> HtmlToken<'a> {
    let state = self.clone();
    // let click_count = self.click_count;
    // let is_hovering = self.is_hovering;
    // let times_pluralized = if click_count == 1 { " time" } else { " times" };

    let cell = Rc::new(RefCell::new(self));
    // let cell_2 = cell.clone();
    let cell_3 = cell.clone();
    // let cell_4 = cell.clone();
    // let increment: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |&ref event| {
    //   let mut s = cell.borrow_mut();
    //   if event.alt_key && event.shift_key {
    //     s.click_count += 5;
    //   } else if event.alt_key {
    //     s.click_count -= 5;
    //   } else if event.shift_key {
    //     s.click_count -= 1;
    //   } else {
    //     s.click_count += 1;
    //   }
    //   s.text_input = "".to_string();
    // });
    // let decrement: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |_| {
    //   let mut s = cell_2.borrow_mut();
    //   s.click_count -= 1;
    // });
    // let start_hovering: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |_| {
    //   let mut s = cell_3.borrow_mut();
    //   s.is_hovering = true;
    // });
    // let stop_hovering: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |_| {
    //   let mut s = cell_4.borrow_mut();
    //   s.is_hovering = false;
    // });
    let update_text: Box<events::InputEventHandler<'a>> = Box::new(move |e| {
      let mut s = cell_3.borrow_mut();
      if let Some(ref val) = e.value {
        if val != "foo" {
          s.text_input = val.to_string();
        } else {
          s.text_input = "bar".to_string();
        }
      }
    });

    let cursor_pointer = "cursor: pointer; user-select: none;";

      // <div OnClick={increment} style={cursor_pointer}>+</div>
      // <div OnClick={decrement} style={cursor_pointer}>-</div>
      // I have been clicked {click_count}{times_pluralized}
      // And I typed {state.text_input.clone()}
    jsx_verbose!(
      <input on_input={update_text} value={state.text_input.clone()} />
    )
  }
}