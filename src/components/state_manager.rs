use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::left::*;

#[derive(Clone)]
pub struct StateManager {
  pub is_left: bool,
  pub left_component: LeftComponent,
}

impl StateManager {
  pub fn new() -> StateManager {
    StateManager {
      is_left: true,
      left_component: LeftComponent::new(),
    }
  }
}

impl<'a> Component<'a, ()> for StateManager {
  fn render(&'a mut self, props: ()) -> HtmlToken<'a> {
    let state = self.clone();
    jsx!(<div>
      state manager
      { if state.is_left { Some(self.left_component.render(())) } else { None } }
    </div>)
  }
}