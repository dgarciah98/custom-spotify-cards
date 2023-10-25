mod api;
mod component;
mod utils;

use component::card_view::CardView;
use component::home::Home;
use component::text_input::TextInput;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::component::home_bar::HomeBar;

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

#[function_component]
fn App() -> Html {
    html! {
        <main>
			<HashRouter>
			   <div>
			      <HomeBar />
               </div>
               <div class="container">
                  <section class="row">
                     <div class="col">
                        <TextInput />
                     </div>
                  </section>
                  <div style="display: flex; justify-content: center; align-items: center;">
                     <Switch<Route> render={switch} />
                  </div>
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
