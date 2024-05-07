//NOTE:
//agregar el componente menu
//puede ser dropdown menu o context menu
//va a ser practicamente igual al tooltip en muchos aspectos pero lo que si va a cambiar es
//que el contenido del menu no va a ser un componente especial, voy a tener que escribir a la
//fuerza el contenido del menu porque sino entonces las cosas van medio raro
//no se pueden pasar elementos clonados por un portal porque actuan raro, tengo que ir y escribir
//los elementos directamente al portal y no pasarlos como una copia
//creo que tengo que copiar el portal al componente content
//
//
//
//WARNING: no de esta format:
//view! {
//     <Show when=move || show.get()>
//         <Portal mount=document().get_element_by_id("float_container").unwrap() clone:tip>
//             <div _ref=content_ref style=move || format!("translate: {}px {}px; {}", position().0, position().1, visibility()) class=format!("absolute z-50 w-12 h-6 bg-red-500 left-0 top-0 animate-tooltip-open {}", class)>
//                 {tip.clone()}
//             </div>
//         </Portal>
//     </Show>
// }
//
//NOTE: de esta forma:
//
//view! {
//     <MenuContent>
//         <Portal mount=document().get_element_by_id("float_container").unwrap()>
//              <div>
//                  "content"
//              </div>
//         </Portal>
//     </MenuContent>
// }
