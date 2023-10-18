use gloo::history::{HashHistory, History};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(TextInput)]
pub(crate) fn text_input() -> Html {
    let style = "display: flex; justify-content: center; align-items: center; font-size: 1.2vw;";
    let class = use_state(|| "form-control".to_owned());

    let onkeypress = {
        let class = class.clone();

        Callback::from(move |e: KeyboardEvent| {
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
					HashHistory::new().push(format!("/{res}"));
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
                    <input type="text" class={&*class} id="inputForm" onkeypress={onkeypress} placeholder="URI" required=true style="font-size: 1.2vw" />
                 </div>
              </div>
           </form>
        </>
    }
}
