use yew::prelude::*;

#[function_component]
pub(crate) fn Home() -> Html {
    let style = "font-size: 3.2vmin;margin-top: 2%;";
    html! {
        <div style={style}>
           <p>
              {"Welcome to my Custom Spotify Cards app! Here's how to use it:"}
           </p>
           <p>
              {"You're going to need a Spotify URI for the track you want. How do you get one?"}
           </p>
           <ul>
              <li><b>{"Copy Song Link"}</b>{": depending of your device, you can do the following"}
		         <ul>
			        <li><b>{"Mobile"}</b>{": push on the 'Share' button, then 'More' and finally 'Copy link'"}</li>
                    <li><p><b>{"Desktop"}</b>{": click the three dots next to the song, then 'Share' and 'Copy Song Link',"}<br />{" or copy directly from the browser"}</p></li>
			     </ul>
			  </li>
              <li><b>{"Copy Spotify URI"}</b>{": on desktop, press ALT while going to the 'Copy Song Link' button"}</li>
              <li><b>{"Use Track ID"}</b>{": taking only the ID itself for the track also works!"}</li>
           </ul>
           <p>
              {"Keep in mind that it has to be a "}<b><i>{"track"}</i></b>{", that is the resulting URI has to be something like "}<br />
              <tt>{"open.spotify.com/track/<track_id>"}</tt>{" or "}<tt>{"spotify:track:<track_id>"}</tt>
		   </p>
        </div>
    }
}
