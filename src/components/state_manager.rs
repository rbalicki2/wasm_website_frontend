use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::left::*;

#[derive(Clone)]
pub struct StateManager {
  pub is_left: bool,
  pub left_component: LeftComponent,
  pub my_magic_string: String,
}

impl StateManager {
  pub fn new() -> StateManager {
    StateManager {
      is_left: true,
      left_component: LeftComponent::new(),
      my_magic_string: "init!".to_string(),
    }
  }
}

impl<'a> Component<'a, ()> for StateManager {
  fn render(&'a mut self, _props: ()) -> HtmlToken<'a> {
    let state = self.clone();
    let state_2 = self.clone();
    let mut left_component = &mut self.left_component;
    let cell = Rc::new(RefCell::new(&mut self.my_magic_string));

    let left_component_props: LeftComponentProps<'a> = LeftComponentProps {
      magic_string: state.my_magic_string,
      set_magic_string: Box::new(move |val| {
        let mut state = cell.borrow_mut();
        **state = val;
      }),
    };

    jsx!(<div>
      state manager
      { if state_2.is_left { Some(left_component.render(left_component_props)) } else { None } }
    </div>)
  }
}