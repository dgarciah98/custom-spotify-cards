use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

#[function_component]
pub(crate) fn HomeBar() -> Html {
    html! {
        <div>
			<nav class="navbar navbar-expand-lg sticky-top" style="background-color: #1ed760">
			<div class="container-fluid">
			<Link<Route> classes={classes!("navbar-brand", "col", "text-center")} to={Route::Home}>
			<h1 style="font-size: 8vmin; font-family: Montserrat">{"Custom Spotify Cards"}</h1>
		    </Link<Route>>
			</div>
			</nav>
        </div>
    }
}
