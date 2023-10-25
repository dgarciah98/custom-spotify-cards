use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub(crate) struct DownloadButtonProps {
    pub(crate) image: String
}

#[function_component]
pub(crate) fn DownloadButton(DownloadButtonProps { image }: &DownloadButtonProps) -> Html {
	html! {
		<div class="text-center" style="margin-top: 0.5vw; margin-bottom:4%;">
			<a href={image.to_string()} download="spotify-custom-card.png">
			  <button style="min-width:50vw; font-size:1.8vw" class="btn btn-secondary"><i class="fa fa-download" />{" Download"}</button>
			</a>
		</div>
	}
}
