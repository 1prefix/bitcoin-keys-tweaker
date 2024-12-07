use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ResultProps {
    pub secret_key: String,
}

#[function_component]
pub fn ResultComponent(props: &ResultProps) -> Html {
    let secret_key = props.secret_key.clone();

    html!(<div id="result">
        <h2>{"Result Secret Key"}</h2>
        <p class="result_key">{secret_key.clone()}</p>
    </div>)
}
