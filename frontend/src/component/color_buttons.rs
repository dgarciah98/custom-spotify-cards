use common::cards::ColorSelectorEmit;
use image::{Pixel, Rgba};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub(crate) struct ColorButtonsProps {
    pub(crate) colors: Vec<Rgba<u8>>,
    pub(crate) onclick: Callback<ColorSelectorEmit>,
    pub(crate) row: String,
}

#[function_component]
pub(crate) fn ColorButtons(ColorButtonsProps { colors, onclick, row }: &ColorButtonsProps) -> Html {
    let rgb_to_hex = |c: Rgba<u8>| -> String {
        let mut hex = "#".to_string();
        c.channels().iter().enumerate().for_each(|(i, ch)| {
            if i != 4 {
                let x = (*ch as f32 / 16.0).trunc();
                let y = (((*ch as f32 / 16.0) - x) * 16.0).trunc();
                hex.push_str(&format!("{:x}{:x}", x as u8, y as u8));
            }
        });
        hex.to_string()
    };
    let onclick = onclick.clone();
    colors.iter().map(|color| {
		let mut border = color.clone();
		border.blend(&Rgba([0,0,0,100]));
		let button_style = format!(
			"background-color:{}; border-color:{}; box-shadow: 0 0 0.8vw 0.1vw {}; border-width:0.2vw; min-width:5vw; min-height:5vw; aspect-ratio: 1;",
			rgb_to_hex(color.clone()),
			rgb_to_hex(border),
			rgb_to_hex(border)
		);
		let on_color_select = {
			let onclick = onclick.clone();
			let color = color.clone();
			let row = row.clone();
			Callback::from(move |_| {
				onclick.emit(ColorSelectorEmit{ new_color: color, row: row.to_string()})
			})
		};
		html! {
			<button style={button_style} onclick={on_color_select} type="button" class="btn btn-default rounded-circle mx-1" />
		}
	}).collect()
}
