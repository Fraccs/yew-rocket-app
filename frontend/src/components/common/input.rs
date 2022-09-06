use yew::{prelude::*};

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub itype: String,
    pub name: String,
    pub placeholder: String
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input
            type={props.itype.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            class="h-12 rounded-2xl text-center text-gray-600 focus:outline-none"
        />
    }
}
