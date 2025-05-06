use crate::app::components::ui::collapsible::*;
use leptos::prelude::*;

#[component]
pub fn Collapsible(#[prop(into)] msg: Signal<String>, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <CollapsibleProvider open=open>
            <div class="pt-6 px-2 select-none text-base cursor-pointer box-border flex items-center justify-between">
                <CollapsibleTrigger class="flex flex-auto overflow-hidden items-center p-2 rounded-md hover:bg-base-100">
                    // <Icon icon=icondata::RiArrowDownSArrowsLine />
                    // class=MaybeProp::derive(move || Some(
                    // TextProp::from(
                    // format!(
                    // "h-4 w-4 text-base-content/75 group-hover:text-base-content {}",
                    // {
                    // match open.get() {
                    // true => "",
                    // false => "-rotate-90",
                    // }
                    // },
                    // ),
                    // ),
                    // ))
                    <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content mr-auto">
                        <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-semibold tracking-wide mr-auto">
                            {move || msg.get()}
                        </div>
                    </div>
                </CollapsibleTrigger>
            </div>
            <CollapsibleContent>
                {children()}
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
