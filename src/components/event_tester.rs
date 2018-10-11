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
        on_copy={Box::new(|_| console::log_1(&JsValue::from_str("copy")))}
        on_cut={Box::new(|_| console::log_1(&JsValue::from_str("cut")))}
        on_paste={Box::new(|_| console::log_1(&JsValue::from_str("paste")))}
        // on_mouse_enter={Box::new(|_| console::log_1(&JsValue::from_str("enter")))}
        // on_mouse_leave={Box::new(|_| console::log_1(&JsValue::from_str("leave")))}
      >
        foo tester
        <span>stuff</span>
      </h1>
      // <div
      //   required
      //   // on_invalid={Box::new(|_| {
      //   //   console::log_1(&JsValue::from_str("invalid input"));
      //   // })}
      //   // on_input={Box::new(|_| console::log_1(&JsValue::from_str("input event")))}

      //   // on_key_down={Box::new(|_| console::log_1(&JsValue::from_str("keydown event")))}
      //   // on_key_up={Box::new(|_| console::log_1(&JsValue::from_str("keyup event")))}
      //   // on_key_press={Box::new(|_| console::log_1(&JsValue::from_str("keypress event")))}

      //   // on_change={Box::new(|_| console::log_1(&JsValue::from_str("change event")))}
      //   // on_focus={Box::new(|_| console::log_1(&JsValue::from_str("focus event")))}
      //   // on_blur={Box::new(|_| console::log_1(&JsValue::from_str("blur event")))}
      //   // on_mouse_enter={Box::new(|_| console::log_1(&JsValue::from_str("mouse enter event")))}
      //   // on_mouse_leave={Box::new(|_| console::log_1(&JsValue::from_str("mouse leave event")))}
      //   autofocus={""}

      //   // on_contextmenu={Box::new(|_| console::log_1(&JsValue::from_str("context menu event")))}
      //   // on_mouse_move={Box::new(|_| console::log_1(&JsValue::from_str("move your mouse")))}
      //   // on_mouse_up={Box::new(|_| console::log_1(&JsValue::from_str("mouse up")))}

      //   // on_pointer_down={Box::new(|_| console::log_1(&JsValue::from_str("pointerdown")))}
      //   // on_pointer_move={Box::new(|_| console::log_1(&JsValue::from_str("p move")))}
      //   // on_pointer_up={Box::new(|_| console::log_1(&JsValue::from_str("p up")))}
      //   // on_pointer_enter={Box::new(|_| console::log_1(&JsValue::from_str("p enter")))}
      //   // on_pointer_leave={Box::new(|_| console::log_1(&JsValue::from_str("p leave")))}
      //   // on_pointer_over={Box::new(|_| console::log_1(&JsValue::from_str("p over")))}
      //   // on_pointer_out={Box::new(|_| console::log_1(&JsValue::from_str("p out")))}
      // >
      //   this is my div
      //   <div>this occludes it</div>
      //   or does it
      // </div>

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
        draggable
        // on_drag={Box::new(|_| console::log_1(&JsValue::from_str("drag")))}
        on_drag_start={Box::new(|_| console::log_1(&JsValue::from_str("img drag_start")))}
        on_drag_end={Box::new(|_| console::log_1(&JsValue::from_str("img drag end")))}
      />
      // <div
      //   on_mouse_enter={Box::new(|_| console::log_1(&JsValue::from_str("enter")))}
      //   on_mouse_over={Box::new(|_| console::log_1(&JsValue::from_str("over")))}
      //   on_mouse_out={Box::new(|_| console::log_1(&JsValue::from_str("out")))}
      //   on_mouse_down={Box::new(|_| console::log_1(&JsValue::from_str("down")))}
      // >
      //   enter me
      //   <div>or me</div>
      //   <div>or me</div>
      // </div>
      // <img
      //   src="http://localhost:8000/wheel.svg2"
      //   on_error={Box::new(|_| console::log_1(&JsValue::from_str("error")))}
      // />
      <div
        on_drag_over={Box::new(|_| console::log_1(&JsValue::from_str("drag over")))}
        on_drag_enter={Box::new(|_| console::log_1(&JsValue::from_str("div drag enter")))}
        on_drag_exit={Box::new(|_| console::log_1(&JsValue::from_str("div drag exit")))}
        on_drag_leave={Box::new(|_| console::log_1(&JsValue::from_str("div drag leave")))}
        on_drop={Box::new(|_| console::log_1(&JsValue::from_str("div drop queen")))}
      >
        drag and drop target!!!
        <h2>does this occlude it</h2>
        stuff on bottom
      </div>
    </form>)
  }
}
