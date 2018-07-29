use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::input::*;
use super::view_picker;

#[derive(Clone)]
pub struct AppState {
  components: AppStateComponents,
  state: AppStateState,
}

#[derive(Clone)]
pub struct AppStateComponents {
  pub input: Input,
  pub view_picker: view_picker::ViewPicker,
}

#[derive(Clone)]
pub struct AppStateState {
  pub todo_items: Vec<TodoItem>,
  pub current_text: String,
  pub view: View,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
  All,
  Done,
  Incomplete,
}


#[derive(Clone)]
pub struct TodoItem {
  pub text: String,
  pub is_done: bool,
}


impl AppState {
  pub fn new() -> AppState {
    AppState {
      components: AppStateComponents {
        input: Input {},
        view_picker: view_picker::ViewPicker {},
      },
      state: AppStateState {
        todo_items: vec![],
        current_text: "".to_string(),
        view: View::All,
      },
    }
  }

  pub fn create_todo_item(state: &mut AppStateState) {
    let todo_item = TodoItem {
      // TODO avoid cloning
      text: state.current_text.clone(),
      is_done: false,
    };
    state.current_text = "".to_string();
    state.todo_items.push(todo_item);
  }
}

impl<'a> Component<'a, ()> for AppState {
  fn render(&'a mut self, _props: ()) -> HtmlToken<'a> {
    let self_2 = self.clone();
    let cell = Rc::new(RefCell::new(&mut self.state));
    let cell_2 = cell.clone();
    let cell_3 = cell.clone();
    let input = &mut self.components.input;
    let view_picker = &mut self.components.view_picker;

    let input_props: InputProps<'a> = InputProps {
      value: self_2.state.current_text,
      on_input: Box::new(move |e| {
        let mut state = cell.borrow_mut();
        if let Some(ref val) = e.value {
          state.current_text = val.to_string();
        }
      }),
      on_keydown: Box::new(move |e| {
        let mut state = cell_2.borrow_mut();
        if e.key_code == 13 {
          AppState::create_todo_item(&mut state);
        }
      }),
    };

    let view_picker_props = view_picker::ViewPickerProps {
      view: self_2.state.view,
      on_select_view: Box::new(
        move |view| {
          let mut state = cell_3.borrow_mut();
          state.view = view;
        }
      ),
    };

    jsx!(<div>
      Smithy Todo List
      <div>{view_picker.render(view_picker_props)}</div>
      <div>{input.render(input_props)}</div>
    </div>)
  }
}

