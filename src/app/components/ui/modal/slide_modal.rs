use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

#[derive(Debug, Clone)]
struct SlideProviderContext {
    slides: RwSignal<Vec<u8>>,
    height: RwSignal<f64>,
    width: RwSignal<f64>,
}

#[component]
pub fn SlideProvider(
    children: Children,
    #[prop(optional, into)] on_slide: Option<Signal<()>>,
    #[prop(optional, into)] slides: Option<RwSignal<Vec<u8>>>,
    initial_value: u8,
) -> impl IntoView {
    let slides = slides.unwrap_or_default();
    slides.update(|slides| slides.push(initial_value));
    let height = RwSignal::new(0.0);
    let width = RwSignal::new(0.0);
    Effect::new(move |_| {
        slides.with(|_| on_slide.map(|on_slide| on_slide.get()));
    });
    provide_context(SlideProviderContext {
        slides,
        height,
        width,
    });
    view! { {children()} }
}

#[component]
pub fn SlideViewport(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<SlideProviderContext>().expect("have slide context");
    let height = context.height;
    let width = context.width;

    let style_format = move || format!("height:{}px; width:{}px;", height.get(), width.get());

    view! {
        <div class=class style=style_format>
            {children()}
        </div>
    }
}

#[component]
pub fn SlideForward(
    children: Children,
    #[prop(optional)] class: &'static str,
    value: u8,
) -> impl IntoView {
    let slides = use_context::<SlideProviderContext>()
        .expect("have slide context")
        .slides;

    view! {
        <button on:click=move |_| slides.update(move |slides| slides.push(value)) class=class>
            {children()}
        </button>
    }
}

#[component]
pub fn SlideBack(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let slides = use_context::<SlideProviderContext>()
        .expect("hvae alsdjfk")
        .slides;

    view! {
        <button
            on:click=move |_| {
                slides
                    .update(|slides| {
                        slides.pop();
                    })
            }
            class=class
        >
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
    let context_provider = use_context::<SlideProviderContext>().expect("have slide context");
    let content_ref = NodeRef::<Div>::new();
    let slides = context_provider.slides;
    let UseElementSizeReturn {
        width: content_width,
        height: content_height,
    } = use_element_size(content_ref);

    Effect::new(move |_| {
        if slides.get().last().is_some_and(|last| last == &value) {
            context_provider.height.update(|height| {
                *height = content_height.get();
            });
            context_provider.width.update(|width| {
                *width = content_width.get();
            })
        }
    });

    let position = move || {
        format!(
            "transform: translateX({})",
            if slides.get().last() == Some(&value) {
                "".to_string()
            } else if slides.get().iter().any(|val| val == &value) {
                format!("-{}px", content_width.get())
            } else {
                format!("{}px", content_width.get())
            }
        )
    };

    view! {
        <div node_ref=content_ref class=class style=position>
            {children()}
        </div>
    }
}
