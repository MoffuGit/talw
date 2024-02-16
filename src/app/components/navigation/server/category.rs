use crate::app::components::collapsible::*;
use crate::app::components::navigation::server::channel::Channel;
use leptos::*;
use uuid::Uuid;

use crate::{app::server::get_channels_with_category, entities::category::Category};

#[component]
pub fn Category(category: Category, server_id: Uuid) -> impl IntoView {
    let channels = create_resource(
        || (),
        move |_| get_channels_with_category(server_id, category.id),
    );

    view! {
        //NOTE: terminar este componente, agregar icono, es algo rapido y ya brincas a hacer el
        //menu para poder invitar gente a lo canalas, el menu ya tiene su nota pero agregare algo
        //aqui, va a tener distintas cosas segun el server y el tipo de usuari, si este es admin
        //va a tener mas cosas que si solo es normal, lo mas importante ahorita es pode rinvitar
        //gente, poder salirse del server y poder editar el miembro de uno mismo, tambien con el menu vas a poder agregar las cosas de
        //admin, agregar, editar y eliminar canales y servidores
        //
        //NOTE: tambien quiero empezar a ver como guardar fotos en sql para que el usuario tenga foto de perfil
        //solo eso por el momento
        //
        //NOTE: es probable que todo lo anterior se haga con un modal, ya tienes ese compoennte
        //hecho, es algo bastante rapido
        //
        //NOTE: tambien ya voy a poder trabajar en el chat, esos van a ser web sockets y cosas de
        //db
        <CollapsibleProvider>
            <CollapsibleTrigger class="relative pt-4 mb-0.5">
                <div class="cursor-pointer box-border pr-2 pl-4 flex items-center justify-between">
                    <div class="flex-auto overflow-hidden">
                        <div class="w-3 h-3"/>
                        <div class="box-border text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide">
                            {category.name}
                        </div>
                    </div>
                </div>
            </CollapsibleTrigger>
            <CollapsibleContent>
                {
                    move || {
                        channels.and_then(|channels| {
                            channels.iter().map(|channel| {
                                view! {<Channel channel=channel.clone()/>}
                            }).collect_view()
                        })
                    }
                }
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
// view! {
//     <div>
//         <div>{category.name}</div>
//             {
//                 move || {
//                     channels.and_then(|channels| {
//                         channels.iter().map(|channel| {
//                             view! {<Channel channel=channel.clone()/>}
//                         }).collect_view()
//                     })
//                 }
//             }
//     </div>
// }
