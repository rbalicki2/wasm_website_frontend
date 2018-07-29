use jsx_macro::*;
use jsx_types::*;

use super::TodoItem;

#[derive(Clone)]
pub struct TodoItemDisplay {}

pub struct TodoItemDisplayProps<'a> {
  pub todo_item: TodoItem,
  pub on_complete_item: Box<FnMut() -> () + 'a>,
}

impl<'a> StatelessComponent<'a, TodoItemDisplayProps<'a>> for TodoItemDisplay {
  fn render(props: TodoItemDisplayProps<'a>) -> HtmlToken<'a> {
    let mut on_complete_item = props.on_complete_item;
    let style = "cursor: pointer; user-select: none;";
    jsx!(<li
      class={format!(
        "list-group-item border-0 pl-0 {}",
        if props.todo_item.is_done { "font-italic" } else { "" }
      )}
      on_click={Box::new(move |_| on_complete_item())}
      style={style}
    >
      {props.todo_item.text}
    </li>)
  }
}
