use leptos::prelude::*;

#[component]
pub fn AnimateIn(children: Children) -> impl IntoView {
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::JsCast;

                let Some(el) = wrapper_ref.get() else { return };

                let cb = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
                    let entry = entries.get(0).unchecked_into::<web_sys::IntersectionObserverEntry>();
                    if entry.is_intersecting() {
                        visible.set(true);
                    }
                }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

                if let Ok(observer) = web_sys::IntersectionObserver::new(cb.as_ref().unchecked_ref()) {
                    observer.observe(el.as_ref());
                    cb.forget();
                }
            }
        }
    });

    view! {
        <div
            node_ref=wrapper_ref
            class=move || if visible.get() {
                "animate-in fade-in zoom-in slide-in-from-bottom duration-500"
            } else {
                "opacity-0"
            }
        >
            {children()}
        </div>
    }
}
