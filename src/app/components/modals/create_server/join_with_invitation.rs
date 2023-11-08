use super::use_slides;
use crate::app::server::JoinServerWithInvitation;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Join_with_invitation() -> impl IntoView {
    let slides = use_slides();
    let join_with_invitation = create_server_action::<JoinServerWithInvitation>();
    view! {
        <ActionForm action=join_with_invitation>
            <div class="text-center p-[16px]">
                <h1 class="font-bold mb-2 mt-6 text-[24px] leading-[30px]">Join a Server</h1>
                <div class="text-[14px] leading-[18px]">Enter an invite below to join an existing server</div>
                <form method="dialog">
                    <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">"✕"</button>
                </form>
            </div>
            <div class="px-4">
                <div class="mb-4">
                    <div>
                        <h2 class="mb-2 text-[12px] leading-[16px] font-bold">INVITE LINK</h2>
                        <input type="text" name="invitation" id="valide_invitation" class="input rounded input-secondary bg-secondary h-[40px] text-[14px] w-full" placeholder="https://discord.gg/hTKzmak"/>
                    </div>
                </div>
                <div class="mb-4">
                    <h2 class="mb-2 text-[12px] leading-[16px] font-bold">INVITE SHOULD LOOK LIKE</h2>
                    <div class="text-[14px] leading-[18px]">hTKzmak</div>
                    <div class="text-[14px] leading-[18px]">"https://discord.gg/hTKzmak"</div>
                    <div class="text-[14px] leading-[18px]">"https://discord.gg/cool-people"</div>
                </div>
                <div class="rounded-lg bg-base-200 mb-4 flex items-center p-3 text-left">
                    <div class="mr-3 w-[40px] h-[40px]"/>
                    <div>
                        <h2 class="text-[16px] leading-[20px] font-bold">"Don't have an invite?"</h2>
                        <div class="text-[12px] leading-[16px] font-normal">Check out Discoverable communities in Server Discovery.</div>
                    </div>
                    <div class="ml-auto mr-3">">"</div>
                </div>
            </div>
            <div class="relative p-4 overflow-x-auto flex justify-between items-center bg-base-200">
                <button type="reset"
                    //NOTE: para agregar eventos hay que poner on:{event_name} y luego lo que
                    //quieras hacer, para usar el click derecho hay que hacer lo siguiente:
                    // on:mouseup=move |e| {
                    //     e.prevent_default();
                    //     match e.button() {
                    //         2 => log::info!("boton derecho clickeado"),
                    //         _ => log::info!("cualquier otro")
                    //     };
                    // }
                    // on:contextmenu=move |e| e.prevent_default()
                    //para agregar el menu especial voy a tener que conocer el valor de x & y donde
                    //se dio el click y luego abrir el menu en relacion a dichos valores
                    on:click=move |_| slides.update(|slides| {slides.pop();})
                    class="leading-[16px]  w-auto min-h-min hover:underline text-base-content h-[38px] text-[14px] py-0.5 px-1">Back</button>
                <button type="submit" class="bg-primary hover:bg-primary-focus text-neutral py-0.5 px-4 font-medium text-[14px] leading-[16px] align-middle rounded-[3px] h-[38px] no-animation">Join Server</button>
            </div>
        </ActionForm>
    }
}
