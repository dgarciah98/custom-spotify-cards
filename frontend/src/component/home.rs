use yew::prelude::*;

#[function_component(Home)]
pub(crate) fn home() -> Html {
    let style = "font-size: 1.2vw;margin-top: 2%;";
    html! {
        <div style={style}>
           <p>
              {"Welcome to my Custom Spotify Cards app! Here's how to use it:"}
           </p>
           <p>
              {"You're going to need a Spotify URI for the track you want. How do you get one?"}
           </p>
           <ul>
              <li><b>{"Copy Song Link"}</b>{": just copy the link from your app like you normally would or from your browser"}</li>
              <li><b>{"Copy Spotify URI"}</b>{": on desktop, press ALT while going to the Copy Song Link button"}</li>
              <li><b>{"Use Track ID"}</b>{": taking only the ID itself for the track also works!"}</li>
           </ul>
           <p>
              {"Keep in mind that it has to be a "}<b><i>{"track"}</i></b>{", that is the resulting URI has to be something like "}<br />
              <tt>{"open.spotify.com/track/<track_id>"}</tt>{" or "}<tt>{"spotify:track:<track_id>"}</tt>
           </p>
        </div>
    }
}
