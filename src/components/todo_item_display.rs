use jsx_macro::*;
use jsx_types::*;

use super::TodoItem;

#[derive(Clone, Debug)]
pub struct TodoItemDisplay {}

pub struct TodoItemDisplayProps<'a> {
  pub todo_item: TodoItem,
  pub on_complete_item: Box<FnMut() -> () + 'a>,
  pub on_hover_item: Box<FnMut() -> () + 'a>,
  pub on_unhover_item: Box<FnMut() -> () + 'a>,
}

impl<'a> StatelessComponent<'a, TodoItemDisplayProps<'a>> for TodoItemDisplay {
  fn render(props: TodoItemDisplayProps<'a>) -> HtmlToken<'a> {
    let mut on_complete_item = props.on_complete_item;
    let mut on_hover_item = props.on_hover_item;
    let mut on_unhover_item = props.on_unhover_item;
    let style = "cursor: pointer; user-select: none;";
    let text = props.todo_item.text;
    let is_hovered = props.todo_item.is_hovered;

    jsx!(<li
      class={format!(
        "list-group-item border-0 pl-0 {}",
        if props.todo_item.is_done { "font-italic" } else { "" }
      )}
      on_mouseover={Box::new(move |_| on_hover_item())}
      on_mouseout={Box::new(move |_| on_unhover_item())}
      on_click={Box::new(move |_| on_complete_item())}
      style={style}
    >
      {text} {if is_hovered { Some("*") } else { None }}
    </li>)
  }
}
