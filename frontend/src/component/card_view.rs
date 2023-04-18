use base64::{engine::general_purpose, Engine};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardViewProps {
    pub id: String,
}

#[function_component(CardView)]
pub(crate) fn card_view(props: &CardViewProps) -> Html {
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
					let card_data = crate::utils::fetch_data(id.clone()).await;
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
