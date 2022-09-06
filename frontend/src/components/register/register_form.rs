use yew::{prelude::*};
use crate::components::common::input::Input;

#[function_component(RegisterForm)]
pub fn register_form() -> Html {
    html! {
        <div class="flex flex-col items-center gap-12">
            <form class="bg-gradient-to-tr from-slate-800 to-cyan-900 flex flex-col gap-6 p-8 shadow-xl rounded-2xl">
                <div class="text-center text-4xl font-black text-white">{ "Register" }</div>
                <Input itype="text" name="username" placeholder="Username"/>
                <Input itype="email" name="email" placeholder="Email"/>
                <Input itype="password" name="password" placeholder="Password"/>
                <Input itype="password" name="confirm" placeholder="Confirm password"/>
                <input class="text-white font-black h-10 bg-yellow-600 rounded-full hover:cursor-pointer hover:opacity-90" type="submit" value="Register"/>
                <div class="text-white underline">{ "Already registered? Login." }</div>
            </form>
        </div>
    }
}
