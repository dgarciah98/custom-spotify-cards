use base64::{engine::general_purpose, Engine};
use common::{
    cards::{CanvasAssets, TextAssets},
    model::{AccessToken, CardData},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::component::bg_buttons::BackgroundButtons;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct CardViewProps {
    pub id: String,
}

#[function_component(CardView)]
pub(crate) fn card_view(props: &CardViewProps) -> Html {
    let style =
        "margin-left: auto;margin-right: auto;margin-top: 2%;margin-bottom: 2%;width: 70vw;";
    let image = use_state_eq(|| "".to_owned());
    let prev_bg_type = use_state_eq(|| "".to_owned());
    let bg_type = use_state_eq(|| "gradient".to_owned());
    let track_id = props.id.to_owned();
    let card_data: UseStateHandle<Option<CardData>> = use_state(|| None);
    let canvas_assets: UseStateHandle<Option<CanvasAssets>> = use_state_eq(|| None);
    let text_assets: UseStateHandle<Option<TextAssets>> = use_state(|| None);
    let token: UseStateHandle<Option<AccessToken>> = use_state(|| None);

    let bg_types = vec![
        String::from("simple"),
        String::from("gradient"),
        String::from("reverted"),
        String::from("custom"),
    ];

    let onclick = {
        let bg_type = bg_type.clone();
        Callback::from(move |btn_type: String| {
            bg_type.set(btn_type);
        })
    };

    {
        let image = image.clone();
        let canvas_assets = canvas_assets.clone();
        let text_assets = text_assets.clone();
        let card_data = card_data.clone();
        let bg_type = bg_type.clone();

        use_effect_with((track_id.clone(), canvas_assets.clone(), bg_type.clone()), move |_| {
            spawn_local(async move {
                let mut new_card_data = (*card_data).clone();
                if new_card_data.clone().is_none()
                    || new_card_data.clone().unwrap().track_id != track_id
                {
                    log::info!("fetch_data");
                    new_card_data = crate::utils::fetch_data(track_id.clone(), token).await.ok();
                    log::info!("data fetched");
                }
                let t0 = web_sys::window().unwrap().performance().unwrap().now();
                let mut new_canvas_assets = (*canvas_assets).clone();
                let mut new_text_assets = (*text_assets).clone();
                if new_card_data.clone() != (*card_data).clone() || (*card_data).clone().is_none() {
                    card_data.set(Some(new_card_data.clone().unwrap()));
                    new_canvas_assets =
                        Some(common::cards::generate_canvas_assets(new_card_data.clone().unwrap()));
                    canvas_assets.set(new_canvas_assets.clone());

                    new_text_assets = Some(common::cards::generate_text_assets(
                        new_card_data.clone().unwrap(),
                        new_canvas_assets.clone().unwrap(),
                    ));
                    text_assets.set(new_text_assets.clone());
                }
                if (*bg_type).clone() != (*prev_bg_type).clone() {
                    prev_bg_type.set((*bg_type).clone());
                    let generated_image = common::cards::generate_card(
                        if !new_card_data.clone().is_none() {
                            new_card_data.clone().unwrap()
                        } else {
                            (*card_data).clone().unwrap()
                        },
                        if !new_canvas_assets.clone().is_none() {
                            new_canvas_assets.clone().unwrap()
                        } else {
                            (*canvas_assets).clone().unwrap()
                        },
                        if !new_text_assets.clone().is_none() {
                            new_text_assets.clone().unwrap()
                        } else {
                            (*text_assets).clone().unwrap()
                        },
                        (*bg_type).clone(),
                    );
                    let b64 = general_purpose::STANDARD.encode(&generated_image);
                    image.set(format!("data:image/png;base64,{}", b64));
                }
                let t1 = web_sys::window().unwrap().performance().unwrap().now();
                log::info!("time {:?}", t1 - t0);
            });
            || ()
        });
    };
    html! {
        <div>
          <div class="btn-group" role="group" aria-label="Background selector">
            <BackgroundButtons types={bg_types} {onclick} />
          </div>
          <div>
            <img src={(*image).clone()} style={style} />
          </div>
        </div>
    }
}
