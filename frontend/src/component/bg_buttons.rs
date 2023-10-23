use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub(crate) struct BackgroundButtonsProps {
    pub(crate) types: Vec<String>,
    pub(crate) onclick: Callback<String>,
}

#[function_component]
pub(crate) fn BackgroundButtons(
    BackgroundButtonsProps { types, onclick }: &BackgroundButtonsProps,
) -> Html {
    let capitalize = |s: &str| -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    };
    let onclick = onclick.clone();
    types.iter().enumerate().map(|(i, bg_type)| {
		let on_bg_select = {
			let onclick = onclick.clone();
			let bg_type = bg_type.clone();
			Callback::from(move |_| {
				onclick.emit(bg_type.clone())
			})
		};
		html! {
			<div>
				<input type="radio" class="btn-check" name="btnradio" id={format!("btncheck{}",i)} autocomplete="off" />
				<label onclick={on_bg_select}
			           style="min-width:13vw; font-size:1.8vw"
				       type="button" class="btn btn-secondary mx-2"
				       for={format!("btncheck{}",i)}>{capitalize(bg_type)}</label>
			</div>
		}
	}).collect()
}
