# Wasm website frontend

* this is where you'd write your app
* this app will be initialized using create-smithy-app
* it relies on external libraries
  * jsx_macro
  * smithy

# TODO

* builder pattern for dom element
* on click event with a few random properties
* learn about std web
  * maybe std web's js! macro can be used to not have to include the js files in the core crate
* CSS?
* reconciliation
  * maybe the HtmlToken needs a version without event handlers that implements Clone

# Blog topics to cover

* integration with other projects, e.g. std web
* instability of API
* amount of times I had to change things
* lifetimes, "compiler knows more than I do"
* goal: close to idiomatic rust, add macros on top of that to make it more ergonomic
* CSS
* builder pattern
* ability to put JS in another crate
* unsafe JSON.parse and serde_json::from_str/to_str calls
* inability to pass more complicated structs across the wasm abi boundary
* awkwardness of thread_local
* organization
* inconsistent definition of paths
* multiple apps
* should've used typescript
* reconciliation
* let foo: () = foo;
