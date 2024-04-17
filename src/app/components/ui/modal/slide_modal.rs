use leptos::html::Div;
use leptos::*;

#[derive(Debug, Clone)]
struct SlideProviderContext {
    slides: RwSignal<Vec<u8>>,
    height: RwSignal<i32>,
}

#[component]
pub fn SlideProvider(
    children: Children,
    #[prop(optional, into)] slides: Option<RwSignal<Vec<u8>>>,
    initial_value: u8,
) -> impl IntoView {
    let slides = if let Some(slides) = slides {
        slides.update(move |slides| slides.push(initial_value));
        slides
    } else {
        create_rw_signal(vec![initial_value])
    };
    let height = create_rw_signal::<i32>(0);
    provide_context(SlideProviderContext { slides, height });
    children()
}

#[component]
pub fn SlideViewport(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let height = use_context::<SlideProviderContext>()
        .expect("have slide context")
        .height;

    let height_format = move || format!("height:{}px", height.get());

    view! {
        <div class=class style=move || height_format()>
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
    children: Children,
    #[prop(optional)] class: &'static str,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let slides = use_context::<SlideProviderContext>()
        .expect("hvae alsdjfk")
        .slides;

    view! {
        <button {..attrs} on:click=move |_| slides.update(|slides| {slides.pop();}) class=class>
            {children()}
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
        if slides.get().last().is_some_and(|last| last == &value) {
            context_provider.height.update(|height| {
                let new_height = content_ref.get().unwrap().scroll_height();
                *height = new_height
            })
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
        <div _ref=content_ref class=move || class()>
            {children()}
        </div>
    }
}
