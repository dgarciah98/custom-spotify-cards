//use gloo::history::{HashHistory, History};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::Route;

#[function_component]
pub(crate) fn TextInput() -> Html {
	let navigator = use_navigator().unwrap();
    let style = "display: flex; justify-content: center; align-items: center; font-size: 3.2vmin;";
    let class = use_state(|| "form-control".to_owned());

    let onkeypress = {
        let class = class.clone();
		
        Callback::from(move |e: KeyboardEvent| {
			let class = class.clone();
			let navigator = navigator.clone();
            if e.key() == "Enter" {
                let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
				
				let res = match crate::utils::parse_uri(value.clone()) {
					Ok(ok) => {
						class.set("form-control".to_string());
						ok
					}
					Err(err) => {
						class.set("form-control is-invalid".to_string());
						panic!("{:?}", err);
					}
				};
				if !res.is_empty() {
					navigator.push(&Route::Card { id: res })
				}
            }
        })
    };

    html! {
        <>
           <form id="inputForm" onSubmit="return false;" style={style}>
              <div class="col-8">
                 <label for="validationInput" class="form-label">{"Put your favorite song!"}</label>
                 <div class="input-group">
                    <input type="text" class={&*class} id="inputForm" {onkeypress} placeholder="URI" required=true />
                 </div>
              </div>
           </form>
        </>
    }
}
