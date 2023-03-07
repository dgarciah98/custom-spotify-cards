mod api;

use base64::{engine::general_purpose, Engine};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    Card { id: String },
}

#[derive(Properties, PartialEq, Clone)]
struct CardViewProps {
    id: String,
}

#[derive(Properties, PartialEq, Clone)]
struct Input {
    class: String,
}

#[rustfmt::skip(html)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {},
        Route::Card { id } => html! {<CardView id={id} /> },
    }
}

fn parse_uri(uri: String) -> Result<String, JsValue> {
    let get_id = |url: web_sys::Url| {
        url.pathname().split(|c| c == '/' || c == ':').last().unwrap().to_string()
    };
    let regex = regex::Regex::new(r"(^[a-zA-Z0-9]{22}$)").unwrap();
    if regex.is_match(&uri) {
        return Ok(uri);
    }

    let url = web_sys::Url::new(&uri);
    let url_res = match url {
        Ok(url) => Ok(get_id(url)),
        Err(err) => Err(err),
    };
    match url_res {
        Ok(id) => {
            if regex.is_match(&id) {
                Ok(id)
            } else {
                Err(JsValue::from_str("Invalid ID"))
            }
        }
        Err(err) => Err(err),
    }
}

#[function_component(TextInput)]
#[rustfmt::skip(html)]
fn text_input() -> Html {
    let navigator = use_navigator().unwrap();
    let style = "display: flex; justify-content: center; align-items: center;";
    let class = use_state(|| "form-control".to_owned());

    let onchange = {
        let class = class.clone();

        Callback::from(move |e: Event| {
            let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();

            let res = match parse_uri(value.clone()) {
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
                //id.set(Some(res));
                navigator.push(&Route::Card { id: res })
            }
        })
    };

    html! {
        <>
           <form id="inputForm" onSubmit="return false;" style={style}>
              <div class="col-8">
                 <label for="validationInput" class="form-label">{"Put your favorite song!"}</label>
                 <div class="input-group">
                    <input type="text" class={&*class} id="inputForm" onchange={onchange} placeholder="URI" required=true />
                 </div>
              </div>
           </form>
        </>
    }
}

#[function_component(CardView)]
fn card_view(CardViewProps { id }: &CardViewProps) -> Html {
    let image = use_state(|| "".to_owned());
    {
        let image = image.clone();
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let token = api::authorize().await;
                    let track = api::get_song(id, token.clone()).await;
					let common::model::Track { name, album, .. } = track.clone();
                    let artist_id = track.artists.first().unwrap().id.to_owned();
                    let artist = api::get_artist(artist_id, token).await;
					let image_data = album.images.first().unwrap();
                    let image_bytes = api::get(image_data.url.to_owned()).await.binary().await.unwrap();
                    let card_data = common::model::CardData {
                        name,
                        album: album.name,
                        album_type: album.album_type,
                        artists: track.artists(),
                        genres: artist.genres().unwrap(),
                        jacket_size: image_data.width,
                    };
					let t0 = web_sys::window().unwrap().performance().unwrap().now();
                    let generated_image = common::cards::generate_card(card_data, &image_bytes).await;
					let t1 = web_sys::window().unwrap().performance().unwrap().now();
					log::info!("time {:?}",t1-t0);
                    let b64 = general_purpose::STANDARD.encode(&generated_image);
                    image.set(format!("data:image/png;base64,{}", b64));
                });
                || ()
            },
            (),
        );
    }
    // let card_data = common::model::CardData {
    //     name: track.as_ref().unwrap().name.to_owned(),
    //     album: track.as_ref().unwrap().album.name.to_owned(),
    //     album_type: track.as_ref().unwrap().album.album_type.to_owned(),
    //     artists: track.as_ref().unwrap().artists().to_owned(),
    //     genres: genres.to_string(),
    //     jacket_size: *jacket_size,
    // };
    // log::info!("voy a generar la imagen");
    // let image = common::cards::generate_card(card_data, image_bytes.as_ref().unwrap());
    // let b64 = general_purpose::STANDARD.encode(&image);
    // log::info!("la imagen en b64 {:?}",b64);
    html! {
       <img src={(*image).clone()} />
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="container">
			<BrowserRouter>
              <div class="row">
                 <div class="col text-center">
                    <h1>{ "Spotify Custom Cards" }</h1>
                 </div>
              </div>
              <section class="row">
                 <div class="col">
                    <TextInput />
                 </div>
              </section>
              <div>
                 <Switch<Route> render={switch} />
              </div>
       </BrowserRouter>
    </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
