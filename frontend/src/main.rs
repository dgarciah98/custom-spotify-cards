mod api;
mod utils;
mod component;

use component::card_view::CardView;
use component::text_input::TextInput;
use component::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:id")]
    Card { id: String },
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Card { id } => html! {<CardView id={id} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
	let font_size = "font-size: 3vw;";
	let style = "display: flex; justify-content: center; align-items: center;";
    html! {
        <main class="container">
			<HashRouter>
              <div class="row">
                 <div class="col text-center">
                    <h1 style={format!("{} {}", style, font_size)}>{ "Custom Spotify Cards" }</h1>
                 </div>
              </div>
              <section class="row">
                 <div class="col">
                    <TextInput />
                 </div>
              </section>
            <div style={style}>
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
