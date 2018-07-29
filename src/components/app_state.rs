use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::input;
use super::view_picker;
use super::todo_item_display;

#[derive(Clone)]
pub struct AppState {
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
      todo_items: vec![],
      current_text: "".to_string(),
      view: View::All,
    }
  }

  pub fn create_todo_item(todo_items: &mut Vec<TodoItem>, current_text: &mut String) {
    let todo_item = TodoItem {
      text: current_text.clone(),
      is_done: false,
    };
    *current_text = "".to_string();
    todo_items.push(todo_item);
  }
}

fn clone_many_times<T>(n: usize, cell: &T) -> Vec<T> where T: Clone {
  let mut vec = Vec::with_capacity(n);
  for _ in 0..n {
    vec.push(cell.clone());
  }
  vec
}

impl<'a> Component<'a, ()> for AppState {
  fn render(&'a mut self, _props: ()) -> HtmlToken<'a> {
    let self_2 = self.clone();
    let todo_items_cell = Rc::new(RefCell::new(&mut self.todo_items));
    let todo_items_cells = clone_many_times(self_2.todo_items.len(), &todo_items_cell);
    let current_text_cell = Rc::new(RefCell::new(&mut self.current_text));
    let current_text_cell_2 = current_text_cell.clone();
    let view_cell = Rc::new(RefCell::new(&mut self.view));

    let input_props: input::InputProps<'a> = input::InputProps {
      value: self_2.current_text,
      on_input: Box::new(move |e| {
        let mut current_text = current_text_cell.borrow_mut();
        if let Some(ref val) = e.value {
          **current_text = val.to_string();
        }
      }),
      on_keydown: Box::new(move |e| {
        let mut todo_items = todo_items_cell.borrow_mut();
        let mut current_text = current_text_cell_2.borrow_mut();
        if e.key_code == 13 {
          AppState::create_todo_item(&mut todo_items, &mut current_text);
        }
      }),
    };

    let view_picker_props = view_picker::ViewPickerProps {
      view: self_2.view,
      on_select_view: Box::new(
        move |new_view| {
          let mut view = view_cell.borrow_mut();
          **view = new_view;
        }
      ),
    };

    let view = self_2.view;
    let is_visible = |todo_item: &TodoItem| {
      match view {
        View::All => true,
        View::Done => todo_item.is_done,
        View::Incomplete => !todo_item.is_done,
      }
    };

    let todo_item_displays = self_2.todo_items
      .iter()
      .zip(todo_items_cells.into_iter())
      .zip(0..self_2.todo_items.len())
      .map(|((todo_item, rc), i)| (todo_item, rc, i))
      .filter(|(t, _, _)| is_visible(t))
      .map(|(todo_item, rc, i)| {
        let todo_item = todo_item.clone();
        let todo_item_display_props = todo_item_display::TodoItemDisplayProps {
          todo_item,
          on_complete_item: Box::new(move || {
            let mut todo_items = rc.borrow_mut();
            let todo_item = todo_items.get_mut(i).unwrap();
            todo_item.is_done = !todo_item.is_done;
          }),
        };
        todo_item_display::TodoItemDisplay::render(todo_item_display_props)
      })
      .collect::<Vec<HtmlToken>>();

    jsx!(<div>
      Smithy Todo List
      <div>{view_picker::ViewPicker::render(view_picker_props)}</div>
      <div>{input::Input::render(input_props)}</div>
      {todo_item_displays}
    </div>)
  }
}
