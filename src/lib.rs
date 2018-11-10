extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document : HTMLDocument;

    #[wasm_bindgen(method)]
    fn getElementById(this: &HTMLDocument, tagName:&str) -> Element;

    #[wasm_bindgen(method, setter=innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));

    document.getElementById("app").set_inner("<p>HI THIS IS GOOD</p>");
}
