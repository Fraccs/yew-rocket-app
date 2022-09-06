use yew::prelude::*;
use crate::components::register::register_form::RegisterForm;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <div class="absolute h-full w-full bg-gradient-to-tr from-slate-900 to-cyan-900 flex items-center justify-center">
                <RegisterForm/>
        </div>
    }
}
