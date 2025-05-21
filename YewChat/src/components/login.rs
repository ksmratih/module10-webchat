use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="w-screen h-screen" style="background: linear-gradient(to right, #fbc2eb, #a6c1ee);">
            <div class="container mx-auto flex flex-col justify-center items-center text-white h-full">
                <h1 class="text-3xl mb-4">{"🌸 Welcome to YewChat!"}</h1>
                <p class="mb-6">{"Enter your name and start chatting with people all around the world 🌍"}</p>
                <form class="m-4 flex">
                    <input
                        {oninput}
                        class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white"
                        placeholder="Username"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r"
                        >
                            {"Start Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }

}