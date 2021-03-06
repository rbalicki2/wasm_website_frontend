use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem::transmute;

use super::input;
use super::view_picker;
use super::todo_item_display;
use super::event_tester;
use super::custom_select;

use web_sys::{
  Event,
  HtmlInputElement,
  InputEvent,
  EventTarget,
};
use web_sys::console::log_1;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct AppState {
  todo_items: Vec<TodoItem>,
  current_text: String,
  view: View,
  select: custom_select::CustomSelect,
  select_vec_items: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
  All,
  Done,
  Incomplete,
}

#[derive(Clone, Debug)]
pub struct TodoItem {
  pub text: String,
  pub is_done: bool,
  pub is_hovered: bool,
}

impl AppState {
  pub fn new() -> AppState {
    AppState {
      todo_items: vec![
        TodoItem {
          text: "Learn Smithy".to_string(),
          is_done: false,
          is_hovered: false,
        },
        TodoItem {
          text: "Rust, but verify".to_string(),
          is_done: true,
          is_hovered: false,
        },
      ],
      current_text: "".to_string(),
      view: View::All,
      select: custom_select::CustomSelect::new(),
      select_vec_items: vec![
        "one".into(),
        "two".into(),
      ],
    }
  }

  pub fn create_todo_item(todo_items: &mut Vec<TodoItem>, current_text: &mut String) {
    let todo_item = TodoItem {
      text: current_text.clone(),
      is_done: false,
      is_hovered: false,
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
    let state_formatted = format!("{:?}", self);
    let self_2 = self.clone();
    let items = &self.select_vec_items;
    let todo_items_cell = Rc::new(RefCell::new(&mut self.todo_items));
    let todo_items_cells = clone_many_times(self_2.todo_items.len(), &todo_items_cell);
    let current_text_cell = Rc::new(RefCell::new(&mut self.current_text));
    let current_text_cell_2 = current_text_cell.clone();
    let current_text_cell_3 = current_text_cell.clone();
    let view_cell = Rc::new(RefCell::new(&mut self.view));
    // let select_state = &mut self.select;

    let input_props: input::InputProps<'a> = input::InputProps {
      value: self_2.current_text,
      on_input: Box::new(move |e| {
        let mut current_text = current_text_cell.borrow_mut();
        let e: &Event = e.as_ref();

        let target: HtmlInputElement = unsafe {
          transmute::<EventTarget, HtmlInputElement>(e.target().unwrap())
        };

        **current_text = target.value();
      }),
      on_key_down: Box::new(move |e| {
        let mut todo_items = todo_items_cell.borrow_mut();
        let mut current_text = current_text_cell_2.borrow_mut();
        if e.key_code() == 13 {
          AppState::create_todo_item(&mut todo_items, &mut current_text);
          **current_text = "".into();
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
        let rc2 = rc.clone();
        let rc3 = rc.clone();
        let todo_item = todo_item.clone();
        let todo_item_display_props = todo_item_display::TodoItemDisplayProps {
          todo_item,
          on_complete_item: Box::new(move || {
            let mut todo_items = rc.borrow_mut();
            let todo_item = todo_items.get_mut(i).unwrap();
            todo_item.is_done = !todo_item.is_done;
          }),
          on_hover_item: Box::new(move || {
            let mut todo_items = rc2.borrow_mut();
            let todo_item = todo_items.get_mut(i).unwrap();
            todo_item.is_hovered = true;
          }),
          on_unhover_item: Box::new(move || {
            let mut todo_items = rc3.borrow_mut();
            let todo_item = todo_items.get_mut(i).unwrap();
            todo_item.is_hovered = false;
          })
        };
        todo_item_display::TodoItemDisplay::render(todo_item_display_props)
      })
      .collect::<Vec<HtmlToken>>();

    // let select_state_html_token: HtmlToken<'b> = select_state.render(());
    jsx!(<div class="container"
      // on_pointerenter={Box::new(|_| console::log_1(&JsValue::from_str("enter")))}
      // on_pointerleave={Box::new(|_| console::log_1(&JsValue::from_str("leave")))}
      // on_scroll={Box::new(|_| console::log_1(&JsValue::from_str("scroll")))}
      // style="max-height: 100px;overflow-y:scroll"
    >
      <link
        href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css"
        rel="stylesheet"
      />
      <title>Smithy Todo List</title>
      <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
      <h1 class="mt-4">Smithy Todo List</h1>
      {view_picker::ViewPicker::render(view_picker_props)}
      {input::Input::render(input_props)}
      <ul class="mt-2 list-group list-group-flush">
        {todo_item_displays}
      </ul>
      <hr />
      { state_formatted }
      <hr />
      <div
        on_mouse_move={Box::new(move |_| {
          log_1(&JsValue::from_str("mouse over"));
        })}
      >
        here <span>not here</span>
      </div>
      { event_tester::EventTester::render(()) }

      {
        self.select.render(custom_select::CustomSelectProps {
          items,
          on_click: Box::new(move |s| {
            // log_1(&JsValue::from_str("in on click handler in app state"));
            // log_1(&JsValue::from_str(&s));
            let mut current_text = current_text_cell_3.borrow_mut();
            **current_text = s;
          })
        })
      }

    </div>)
  }
}
