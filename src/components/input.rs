use jsx_macro::*;
use jsx_types::*;

#[derive(Clone)]
pub struct Input {}

pub struct InputProps<'a> {
  pub value: String,
  pub on_input: Box<events::InputEventHandler<'a>>,
  pub on_key_down: Box<events::KeyboardEventHandler<'a>>,
}

impl<'a> StatelessComponent<'a, InputProps<'a>> for Input {
  fn render(props: InputProps<'a>) -> HtmlToken<'a> {
    jsx!(<input
      class="form-control mt-3"
      type="text"
      value={props.value}
      on_input={props.on_input}
      on_key_down={props.on_key_down}
    />)
  }
}
