use crate::app::api::auth::use_auth;
use leptos::prelude::*;
use leptos_router::components::A;

#[allow(non_snake_case)]
#[component]
pub fn Signup() -> impl IntoView {
    let signup = use_auth().signup;
    on_cleanup(move || signup.value().set(None));
    view! {
        <ActionForm action=signup /* class="w-full h-full flex flex-col items-center" */>
            <A href="/" {..} class="btn btn-ghost btn-sm m-1">
                "go back"
            </A>
            <h1 class="w-auto text-center font-bold text-5xl mt-[24vh] mb-2">"Sign Up"</h1>
            <div class="form-control w-full max-w-xs mb-2">
                <label class="label">
                    <span class="label-text">"User ID"</span>
                </label>
                <input
                    type="text"
                    placeholder="Enter your ID..."
                    maxlength="32"
                    name="username"
                    class="input input-bordered input-sm w-full max-w-xs"
                />
            </div>
            <div class="form-control w-full max-w-xs mb-2">
                <label class="label">
                    <span class="label-text">"Password"</span>
                </label>
                <input
                    type="password"
                    placeholder="Enter your password"
                    name="password"
                    class="input input-bordered input-sm w-full max-w-xs"
                />
            </div>
            <div class="form-control w-full max-w-xs mb-2">
                <label class="label">
                    <span class="label-text">"Confirmation password"</span>
                </label>
                <input
                    type="password"
                    placeholder="Password again"
                    name="confirmation_password"
                    class="input input-bordered input-sm w-full max-w-xs"
                />
            </div>
            <div class="form-control w-full max-w-xs mb-2">
                <label class="label cursor-pointer">
                    <span class="label-text">Remember me:</span>
                    <input type="checkbox" name="remember" class="checkbox" />
                </label>
            </div>
            <button
                type="submit"
                class="btn btn-sm btn-outline btn-error w-full max-w-xs mb-2"
                disabled=move || signup.pending().get()
            >
                "Sing Up"
            </button>
            {move || {
                signup
                    .value()
                    .get()
                    .map(|res| match res {
                        Err(ServerFnError::ServerError(err)) => {
                            view! { <p class="text-error w-full text-center">{err}</p> }.into_any()
                        }
                        _ => view! { <p class="text-error w-full text-center" /> }.into_any(),
                    })
            }}
        </ActionForm>
    }
}
