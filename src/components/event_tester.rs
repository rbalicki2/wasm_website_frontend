use jsx_macro::*;
use jsx_types::*;
// use std::mem::transmute;

use web_sys::{
  console,
  // Event,
  // InputEvent
};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct EventTester {}

impl<'a> StatelessComponent<'a, ()> for EventTester {
  fn render(_props: ()) -> HtmlToken<'a> {
    jsx!(<form
      class="tester"
      // on_submit={Box::new(|e| {
      //   console::log_1(&JsValue::from_str("submit event"));
      //   let e: &Event = unsafe { transmute::<&InputEvent, &Event>(e) };
      //   e.prevent_default();
      // })}
    >
      <h1
        // on_dbl_click={Box::new(|_| console::log_1(&JsValue::from_str("dbl click event")))}
        // on_mouse_down={Box::new(|_| console::log_1(&JsValue::from_str("mouse down event")))}
        // on_copy={Box::new(|_| console::log_1(&JsValue::from_str("copy")))}
        // on_cut={Box::new(|_| console::log_1(&JsValue::from_str("cut")))}
        // on_paste={Box::new(|_| console::log_1(&JsValue::from_str("paste")))}
        // on_mouse_enter={Box::new(|_| console::log_1(&JsValue::from_str("enter")))}
        // on_mouse_leave={Box::new(|_| console::log_1(&JsValue::from_str("leave")))}
      >
        foo tester
      </h1>
      <input
        required
        // on_invalid={Box::new(|_| {
        //   console::log_1(&JsValue::from_str("invalid input"));
        // })}
        // on_input={Box::new(|_| console::log_1(&JsValue::from_str("input event")))}

        // on_key_down={Box::new(|_| console::log_1(&JsValue::from_str("keydown event")))}
        // on_key_up={Box::new(|_| console::log_1(&JsValue::from_str("keyup event")))}
        // on_key_press={Box::new(|_| console::log_1(&JsValue::from_str("keypress event")))}

        // on_change={Box::new(|_| console::log_1(&JsValue::from_str("change event")))}
        // on_focus={Box::new(|_| console::log_1(&JsValue::from_str("focus event")))}
        // on_blur={Box::new(|_| console::log_1(&JsValue::from_str("blur event")))}
        // on_mouse_enter={Box::new(|_| console::log_1(&JsValue::from_str("mouse enter event")))}
        // on_mouse_leave={Box::new(|_| console::log_1(&JsValue::from_str("mouse leave event")))}
        autofocus={""}

        // on_contextmenu={Box::new(|_| console::log_1(&JsValue::from_str("context menu event")))}
        // on_mouse_move={Box::new(|_| console::log_1(&JsValue::from_str("move your mouse")))}
        // on_mouse_up={Box::new(|_| console::log_1(&JsValue::from_str("mouse up")))}

        // on_pointerdown={Box::new(|_| console::log_1(&JsValue::from_str("pointerdown")))}
        // on_pointermove={Box::new(|_| console::log_1(&JsValue::from_str("move")))}
        // on_pointerup={Box::new(|_| console::log_1(&JsValue::from_str("up")))}
        // on_pointerenter={Box::new(|_| console::log_1(&JsValue::from_str("enter")))}
        // on_pointerleave={Box::new(|_| console::log_1(&JsValue::from_str("leave")))}
        // on_pointerover={Box::new(|_| console::log_1(&JsValue::from_str("over")))}
        on_select={Box::new(|_| console::log_1(&JsValue::from_str("select some text yo")))}
      />

      // <details
      //   class="details"
      //   on_toggle={Box::new(|_| console::log_1(&JsValue::from_str("toggle")))}
      //   on_animation_start={Box::new(|_| console::log_1(&JsValue::from_str("animation start")))}
      //   on_animation_end={Box::new(|_| console::log_1(&JsValue::from_str("animation end")))}
      //   on_animation_iteration={Box::new(|_| console::log_1(&JsValue::from_str("animation iteration")))}
      // >
      //   stfu
      // </details>
      // <div
      //   style="max-height: 100px; overflow-y: scroll" 
      //   class="overflow"
      //   on_scroll={Box::new(|_| console::log_1(&JsValue::from_str("scroll")))}
      // >
      //   <h1>asdfasdf</h1>
      //   <h1>asdfasdf</h1>
      //   <h1>asdfasdf</h1>
      //   <h1>asdfasdf</h1>
      //   <h1>asdfasdf</h1>
      // </div>
      <img
        src="http://localhost:8000/wheel.svg"
        // on_load={Box::new(|_| console::log_1(&JsValue::from_str("load")))}
        // draggable
        // // on_drag={Box::new(|_| console::log_1(&JsValue::from_str("drag")))}
        // on_drag_start={Box::new(|_| console::log_1(&JsValue::from_str("drag_start")))}
        // on_drag_end={Box::new(|_| console::log_1(&JsValue::from_str("drag end")))}
        // on_drop={Box::new(|_| console::log_1(&JsValue::from_str("drop")))}
      />
      // <img
      //   src="http://localhost:8000/wheel.svg2"
      //   on_error={Box::new(|_| console::log_1(&JsValue::from_str("error")))}
      // />
      // <h1
      //   // on_drag_over={Box::new(|_| console::log_1(&JsValue::from_str("drag over")))}
      //   on_drag_enter={Box::new(|_| console::log_1(&JsValue::from_str("drag enter")))}
      //   on_drag_exit={Box::new(|_| console::log_1(&JsValue::from_str("drag exit")))}
      //   on_drag_leave={Box::new(|_| console::log_1(&JsValue::from_str("drag leave")))}
      //   on_drop={Box::new(|_| console::log_1(&JsValue::from_str("drop queen")))}
      //   droppable
      // >
      //   drag and drop target!!!
      // </h1>
    </form>)
  }
}
