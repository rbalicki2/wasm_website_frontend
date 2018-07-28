use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

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
    let click_count = self.click_count;
    let times_pluralized = if click_count == 1 { " time" } else { " times" };

    let cell = Rc::new(RefCell::new(self));
    let cell_2 = cell.clone();
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

    let cursor_pointer = "cursor: pointer; user-select: none;";

    jsx_verbose!(<div>
      I have been clicked {click_count}{times_pluralized}
      <div OnClick={increment} style={cursor_pointer}>+</div>
      <div OnClick={decrement} style={cursor_pointer}>-</div>
      { if click_count > 1 {Some("you clicked a lot, yo")} else {None} }
    </div>)
  }
}