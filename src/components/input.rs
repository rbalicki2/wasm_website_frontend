use jsx_macro::*;
use jsx_types::*;

#[derive(Clone)]
pub struct Input {}

pub struct InputProps<'a> {
  pub value: String,
  pub on_input: Box<events::InputEventHandler<'a>>,
  pub on_keydown: Box<events::KeyboardEventHandler<'a>>,
}

impl<'a> StatelessComponent<'a, InputProps<'a>> for Input {
  fn render(props: InputProps<'a>) -> HtmlToken<'a> {
    jsx!(<input
      value={props.value}
      on_input={props.on_input}
      on_keydown={props.on_keydown}
    />)
  }
}
