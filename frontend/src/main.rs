mod api;
mod utils;

use base64::{engine::general_purpose, Engine};
use gloo::history::{HashHistory, History};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:id")]
    Card { id: String },
    #[at("/")]
    Home,
}

#[derive(Properties, PartialEq, Clone)]
struct CardViewProps {
    id: String,
}

#[derive(Properties, PartialEq, Clone)]
struct Input {
    class: String,
}

//#[rustfmt::skip(html)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {},
        Route::Card { id } => html! {<CardView id={id} /> },
    }
}

#[function_component(TextInput)]
//#[rustfmt::skip(html)]
fn text_input() -> Html {
    let navigator = use_navigator().unwrap();
    let style = "display: flex; justify-content: center; align-items: center; font-size: 1.2vw;";
    let class = use_state(|| "form-control".to_owned());

    let onkeypress = {
        let class = class.clone();

        Callback::from(move |e: KeyboardEvent| {
			if e.key() == "Enter" {
				let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();

				let res = match utils::parse_uri(value.clone()) {
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
					log::info!("got id {:?}", res);
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

#[function_component(CardView)]
fn card_view(props: &CardViewProps) -> Html {
	let style = "margin-left: auto;margin-right: auto;margin-top: 2%;margin-bottom: 2%;width: 70vw;";
    let image = use_state_eq(|| "".to_owned());
	let track_id = props.id.to_owned();
    {
        let image = image.clone();
		let id = track_id.clone();
        use_effect_with(
            track_id,
            move |_| {
                spawn_local(async move {
					let card_data = utils::fetch_data(id.clone()).await;
					let t0 = web_sys::window().unwrap().performance().unwrap().now();
                    let generated_image = common::cards::generate_card(card_data.clone(), &card_data.jacket_bytes);
					let t1 = web_sys::window().unwrap().performance().unwrap().now();
					log::info!("time {:?}",t1-t0);
                    let b64 = general_purpose::STANDARD.encode(&generated_image);
                    image.set(format!("data:image/png;base64,{}", b64));					
                });
				|| ()
            }
        );
    };
    html! {
	   <img src={(*image).clone()} style={style} />
    }
}

#[function_component(App)]
fn app() -> Html {
	let style = "display: flex; justify-content: center; align-items: center; font-size: 3vw;";
    html! {
        <main class="container">
			<HashRouter>
              <div class="row">
                 <div class="col text-center">
                    <h1 style={style}>{ "Spotify Custom Cards" }</h1>
                 </div>
              </div>
              <section class="row">
                 <div class="col">
                    <TextInput />
                 </div>
              </section>
              <div class="col" style="display: flex;">
                 <Switch<Route> render={switch} />
              </div>
       </HashRouter>
    </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
