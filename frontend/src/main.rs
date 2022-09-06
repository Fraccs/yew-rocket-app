use yew::prelude::*;
use yew_router::prelude::*;
use pages::home::Home;
use pages::not_found::NotFound;
use pages::register::Register;

pub mod components;
pub mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Register => html! { <Register/> },
        Route::NotFound => html! { <NotFound/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
