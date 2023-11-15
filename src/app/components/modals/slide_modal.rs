use leptos::html::Div;
use leptos::*;

#[derive(Debug, Clone)]
pub struct SlideProviderContext {
    slides: RwSignal<Vec<u8>>,
    height: RwSignal<i32>,
}

#[component]
pub fn SlideProvider(children: Children, initial_value: u8) -> impl IntoView {
    let slides = create_rw_signal::<Vec<u8>>(vec![initial_value]);
    let height = create_rw_signal::<i32>(0);
    provide_context(SlideProviderContext { slides, height });
    view! {
        {children()}
    }
}

#[component]
pub fn SlideViewport(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let height = use_context::<SlideProviderContext>()
        .expect("have slide context")
        .height;

    let height_format = move || format!("h-[{}px]", height.get());

    let class = move || format!("{} {}", class, height_format());

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SlideForward(
    #[prop(optional)] children: Option<Children>,
    value: u8,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let slides = use_context::<SlideProviderContext>()
        .expect("hvae alsdjfk")
        .slides;

    view! {
        <button on:click=move |_| slides.update(move |slides| slides.push(value))  class=class>
            {children.map(|children| children())}
        </button>
    }
}

#[component]
pub fn SlideBack(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let slides = use_context::<SlideProviderContext>()
        .expect("hvae alsdjfk")
        .slides;

    view! {
        <button on:click=move |_| slides.update(|slides| {slides.pop();}) class=class>
            {children.map(|children| children())}
        </button>
    }
}

#[component]
pub fn SlideContent(
    children: Children,
    value: u8,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let context_provider = use_context::<SlideProviderContext>().expect("have slide xontext");
    let content_ref = create_node_ref::<Div>();
    let slides = context_provider.slides;

    create_effect(move |_| {
        if slides.get().last().is_some_and(|height| height == &value) {
            context_provider
                .height
                .update(|height| *height = content_ref.get().unwrap().offset_height())
        }
    });

    let position = move || {
        if slides.get().last() == Some(&value) {
            ""
        } else if slides.get().iter().any(|val| val == &value) {
            "-translate-x-[440px]"
        } else {
            "translate-x-[440px]"
        }
    };

    let class = move || format!("{} {}", class, position());

    view! {
        <div _ref=content_ref class=class()>
            {children()}
        </div>
    }
}
