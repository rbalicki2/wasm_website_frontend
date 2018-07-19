use jsx_macro::*;
use jsx_types::*;

// use std::boxed::FnBox;

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

    let click_handler: Box<FnMut(Event) -> () + 'a> = Box::new(move |_| {
      self.click_count += 1;
    });
    // let click_handler: () = click_handler;
    // let click_handler_2: Box<FnMut(Event) -> ()> = Box::new(|_| {
    //   self.click_count += 2;
    // });
    jsx!(<div OnClick={click_handler}>I have been clicked {click_count}{times_pluralized}</div>)
  }
}