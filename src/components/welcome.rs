use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Welcome {
  click_count: i32,
  is_hovering: bool,
}

impl Welcome {
  pub fn new() -> Welcome {
    Welcome {
      click_count: 0,
      is_hovering: false,
    }
  }
}

impl<'a> Component<'a> for Welcome {
  fn render(&'a mut self) -> HtmlToken<'a> {
    let click_count = self.click_count;
    let is_hovering = self.is_hovering;
    let times_pluralized = if click_count == 1 { " time" } else { " times" };

    let cell = Rc::new(RefCell::new(self));
    let cell_2 = cell.clone();
    let cell_3 = cell.clone();
    let cell_4 = cell.clone();
    let increment: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |&ref event| {
      let mut s = cell.borrow_mut();
      if event.alt_key && event.shift_key {
        s.click_count += 5;
      } else if event.alt_key {
        s.click_count -= 5;
      } else if event.shift_key {
        s.click_count -= 1;
      } else {
        s.click_count += 1;
      }
    });
    let decrement: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |_| {
      let mut s = cell_2.borrow_mut();
      s.click_count -= 1;
    });
    let start_hovering: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |_| {
      let mut s = cell_3.borrow_mut();
      s.is_hovering = true;
    });
    let stop_hovering: Box<FnMut(&events::MouseEvent) -> () + 'a> = Box::new(move |_| {
      let mut s = cell_4.borrow_mut();
      s.is_hovering = false;
    });

    let cursor_pointer = "cursor: pointer; user-select: none;";

      // <div OnClick={decrement} style={cursor_pointer}>-</div>
      // { if click_count > 2 {Some("you clicked a lot, yo, this many times: ")} else {None} }
      // { if click_count > 2 {Some(click_count)} else {None}}
    jsx!(<div>
      I have been clicked {click_count}{times_pluralized}
      <h1 OnMouseOver={start_hovering} OnMouseOut={stop_hovering}>Hover here</h1>
      { if is_hovering {Some("We hovering")} else {None} }
      <div OnClick={increment} style={cursor_pointer}>+</div>
    </div>)
  }
}