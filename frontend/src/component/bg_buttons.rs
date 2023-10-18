use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub(crate) struct BackgroundButtonsProps {
    pub(crate) types: Vec<String>,
    pub(crate) onclick: Callback<String>,
}

#[function_component(BackgroundButtons)]
pub(crate) fn background_buttons(
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
    types.iter().map(|bg_type| {
		let on_bg_select = {
			let onclick = onclick.clone();
			let bg_type = bg_type.clone();
			Callback::from(move |_| {
				onclick.emit(bg_type.clone())
			})
		};
		html! {
			<button onclick={on_bg_select} type="button" class="btn btn-secondary">{capitalize(bg_type)}</button>
		}
	}).collect()
}
