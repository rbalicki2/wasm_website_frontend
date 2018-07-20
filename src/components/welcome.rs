use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

// use std::boxed::FnBox;

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
    let times_pluralized = if click_count == 1 { " time" } else { " times" };

    let cell = Rc::new(RefCell::new(self));
    let cell_2 = cell.clone();
    let cell_3 = cell.clone();
    let click_handler: Box<FnMut(Event) -> () + 'a> = Box::new(move |_| {
      let mut s = cell.borrow_mut();
      s.click_count += 1;
    });
    let on_mouse_over_handler: Box<FnMut(Event) -> () + 'a> = Box::new(move |_| {
      let mut s = cell_2.borrow_mut();
      s.is_hovering = true;
    });
    let on_mouse_out_handler: Box<FnMut(Event) -> () + 'a> = Box::new(move |_| {
      let mut s = cell_3.borrow_mut();
      // s.is_hovering = false;
      s.click_count = 0;
    });
    // let click_handler: () = click_handler;
    // let click_handler_2: Box<FnMut(Event) -> ()> = Box::new(|_| {
    //   self.click_count += 2;
    // });
    jsx!(<div
      OnClick={click_handler}
      OnMouseOut={on_mouse_out_handler}
    >
      I have been clicked {click_count}{times_pluralized}
    </div>)
  }
}