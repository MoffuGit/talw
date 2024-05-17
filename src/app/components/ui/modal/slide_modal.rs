use leptos::html::Div;
use leptos::*;

#[derive(Debug, Clone)]
struct SlideProviderContext {
    slides: RwSignal<Vec<u8>>,
    height: RwSignal<i32>,
    width: RwSignal<i32>,
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
    let width = create_rw_signal::<i32>(0);
    provide_context(SlideProviderContext {
        slides,
        height,
        width,
    });
    children()
}

#[component]
pub fn SlideViewport(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<SlideProviderContext>().expect("have slide context");
    let height = context.height;
    let width = context.width;

    let style_format = move || format!("height:{}px; width:{}px;", height.get(), width.get());

    view! {
        <div class=class style=move || style_format()>
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
            let content_ref = content_ref.get().unwrap();
            context_provider.height.update(|height| {
                *height = content_ref.scroll_height();
            });
            context_provider.width.update(|width| {
                *width = content_ref.scroll_width();
            })
        }
    });

    create_effect(move |_| {
        let content_ref = content_ref.get().unwrap();
        log::info!("height: {}", content_ref.scroll_height())
    });

    let content_width = move || {
        if let Some(content) = content_ref.get() {
            content.scroll_width()
        } else {
            0
        }
    };

    let position = move || {
        format!(
            "transform: translateX({})",
            if slides.get().last() == Some(&value) {
                "".to_string()
            } else if slides.get().iter().any(|val| val == &value) {
                format!("-{}px", content_width())
            } else {
                format!("{}px", content_width())
            }
        )
    };

    // let class = move || format!("{} {}", class, position());

    view! {
        <div _ref=content_ref class=class style= move || position()>
            {children()}
        </div>
    }
}
