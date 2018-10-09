use jsx_macro::*;
use jsx_types::*;
use std::mem::transmute;

use web_sys::{console, Event, InputEvent};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct EventTester {}

impl<'a> StatelessComponent<'a, ()> for EventTester {
  fn render(_props: ()) -> HtmlToken<'a> {
    jsx!(<form
      class="tester"
      on_submit={Box::new(|e| {
        console::log_1(&JsValue::from_str("submit event"));
        let e: &Event = unsafe { transmute::<&InputEvent, &Event>(e) };
        e.prevent_default();
      })}
    >
      <h1
        // on_dblclick={Box::new(|_| console::log_1(&JsValue::from_str("dbl click event")))}
        // on_drag={Box::new(|_| console::log_1(&JsValue::from_str("drag event")))}
        // on_mousedown={Box::new(|_| console::log_1(&JsValue::from_str("mouse down event")))}
      >
        foo tester
      </h1>
      <input
        // on_input={Box::new(|_| console::log_1(&JsValue::from_str("input event")))}

        // on_keydown={Box::new(|_| console::log_1(&JsValue::from_str("keydown event")))}
        // on_keyup={Box::new(|_| console::log_1(&JsValue::from_str("keyup event")))}
        // on_keypress={Box::new(|_| console::log_1(&JsValue::from_str("keypress event")))}

        // on_change={Box::new(|_| console::log_1(&JsValue::from_str("change event")))}
        // on_focus={Box::new(|_| console::log_1(&JsValue::from_str("focus event")))}
        // on_blur={Box::new(|_| console::log_1(&JsValue::from_str("blur event")))}
        // on_mouseenter={Box::new(|_| console::log_1(&JsValue::from_str("mouse enter event")))}
        // on_mouseleave={Box::new(|_| console::log_1(&JsValue::from_str("mouse leave event")))}
        autofocus={""}

        // on_contextmenu={Box::new(|_| console::log_1(&JsValue::from_str("context menu event")))}
        // on_mousemove={Box::new(|_| console::log_1(&JsValue::from_str("move your mouse")))}
        on_mouseup={Box::new(|_| console::log_1(&JsValue::from_str("mouse up")))}
      />
    </form>)
  }
}
