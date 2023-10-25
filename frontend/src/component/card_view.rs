use base64::{engine::general_purpose, Engine};
use common::{
    cards::{CanvasAssets, ColorSelectorEmit, TextAssets},
    model::{AccessToken, CardData},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::component::{bg_buttons::BackgroundButtons, color_buttons::ColorButtons, dl_button::DownloadButton};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct CardViewProps {
    pub id: String,
}

#[function_component]
pub(crate) fn CardView(props: &CardViewProps) -> Html {
    let style =
        "margin-left: auto;margin-right: auto;margin-top: 2%;margin-bottom: 2%;width: 70vw;";
    let bg_btn_style = "justify-content: center; align-items: center; margin-top: 1.3vw; margin-bottom: 2vw; margin-left: auto; margin-right: auto";
    let color_btn_style = "justify-content: center; align-items: center; margin-bottom: 1vw; margin-left: auto; margin-right: auto";
    let btn_class = "btn-toolbar mr-1";
    let image = use_state_eq(|| "".to_owned());
    let prev_bg_type = use_state_eq(|| "".to_owned());
    let bg_type = use_state_eq(|| "gradient".to_owned());
    let track_id = props.id.to_owned();
    let card_data: UseStateHandle<Option<CardData>> = use_state(|| None);
    let canvas_assets: UseStateHandle<Option<CanvasAssets>> = use_state_eq(|| None);
    let text_assets: UseStateHandle<Option<TextAssets>> = use_state(|| None);
    let token: UseStateHandle<Option<AccessToken>> = use_state(|| None);

    let bg_types = vec![
        String::from("plain"),
        String::from("gradient"),
        String::from("inverted"),
        String::from("custom"),
    ];

    let bg_type_onclick = {
        let bg_type = bg_type.clone();
        Callback::from(move |btn_type: String| {
            bg_type.set(btn_type);
        })
    };

    let color_onclick = {
        let canvas_assets = canvas_assets.clone();
        Callback::from(move |data: ColorSelectorEmit| {
            let mut new_canvas_assets = (*canvas_assets).clone();
            if new_canvas_assets.is_some() {
                match data.row.as_str() {
                    "top" => {
                        let keep_color =
                            new_canvas_assets.clone().unwrap().colors.gradient_end(true);
                        new_canvas_assets.as_mut().unwrap().colors.custom_gradient =
                            Some((data.new_color, keep_color));
                    }
                    _ => {
                        let keep_color =
                            new_canvas_assets.clone().unwrap().colors.gradient_start(true);
                        new_canvas_assets.as_mut().unwrap().colors.custom_gradient =
                            Some((keep_color, data.new_color));
                    }
                }
                canvas_assets.set(new_canvas_assets);
            }
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
                    new_card_data = crate::utils::fetch_data(track_id.clone(), token).await.ok();
                }
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
                if (*bg_type).clone() != (*prev_bg_type).clone()
                    || new_card_data.clone() != (*card_data).clone()
                    || new_canvas_assets.clone() == (*canvas_assets).clone()
                {
                    prev_bg_type.set((*bg_type).clone());
                    let generated_image = common::cards::generate_card(
                        (!new_card_data.clone().is_none())
                            .then_some(new_card_data.clone().unwrap())
                            .unwrap_or((*card_data).clone().unwrap()),
                        (!new_canvas_assets.clone().is_none())
                            .then_some(new_canvas_assets.clone().unwrap())
                            .unwrap_or((*canvas_assets).clone().unwrap()),
                        (!new_text_assets.clone().is_none())
                            .then_some(new_text_assets.clone().unwrap())
                            .unwrap_or((*text_assets).clone().unwrap()),
                        (*bg_type).clone(),
                    );
                    let b64 = general_purpose::STANDARD.encode(&generated_image);
                    image.set(format!("data:image/png;base64,{}", b64));
                }
            });
            || ()
        });
    };

    html! {
        <div>
          <div class="row">
             <div class={btn_class} role="toolbar" style={bg_btn_style} aria-label="Background selector">
               <BackgroundButtons types={bg_types} onclick={bg_type_onclick} />
             </div>
          </div>
          if (*bg_type).clone() == "custom".to_string() {
           <div class="row">
             <div class={btn_class} role="toolbar" style={color_btn_style} aria-label="Color selector 1">
              <p style="text-align:center; width: 12vw; font-size: 1.8vw; margin-top: auto; margin-bottom: auto;">{"Start Color:"}</p>
              <ColorButtons colors={(*canvas_assets).clone().unwrap().colors.all_colors} onclick={color_onclick.clone()} row={"top"} />
             </div>
             <div class={btn_class} role="toolbar" style={color_btn_style} aria-label="Color selector 2">
              <p style="text-align:center; width: 12vw; padding-left: 0.8vw; font-size: 1.8vw;margin-top: auto; margin-bottom: auto;">{"End Color:"}</p>
              <ColorButtons colors={(*canvas_assets).clone().unwrap().colors.all_colors} onclick={color_onclick.clone()} row={"bottom"} />
             </div>
           </div>
          }
          <div class="row">
            <img src={(*image).clone()} style={style} />
			if !(*image).clone().is_empty() {
				<DownloadButton image={(*image).clone()} />
			}
		  </div>
        </div>
    }
}
