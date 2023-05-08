# typestacean-learn-rust-wasm-from-typescript
Typestaceans - Rust's Web Assembly Concepts for TypeScript Developers (It doesn't need to be that hard)

How you can get to speed with WASM as fast as possible as a TypeScript Developer.


## Logging in WASM
How to Log in Web Assembly:
Yew version (Using Gloo, and wasm_bindgen)
```
use wasm_bindgen::JsValue;
use gloo::{console::log};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChangedBy {
    Navigation,
    Manually,
}

impl From<ChangedBy> for JsValue {
    fn from(value: ChangedBy) -> Self {
        match value {
            ChangedBy::Navigation => JsValue::from("navigation"),
            ChangedBy::Manually => JsValue::from("manually"),
        }
    }
}

#[function_component(MyComponent)]
fn my_component() -> Html {
    let changed_by  = ChangedBy::Navigation;
    // Then you can, in your component:
    log!("Navigation is {}", changed_by);
    html! {
        <p>Hello world</p>
    }
}
```

How you'd do it in TypeScript:
```ts
import React from 'react';

enum ChangedBy {
    Navigation
    Manually
}

function MyComponent() {
    const navigation = ChangedBy.Navigation;
    console.log("Navigation is ${navigation}")
    return <p>Hello World</p>
}
```



### Follow me for more:
On how to understand Rust's Web Assembly Concepts as a TypeScript developer. 

I cover:
1. React to Yew
2. Explanation of TypeScript's unknown concepts like Heap, Stack, Boxing, etc. so 
3. Comparisons