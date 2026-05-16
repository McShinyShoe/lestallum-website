use leptos::prelude::*;

use crate::components::sections::footer::SiteFooter;
use crate::components::sections::navbar::{Navbar, ProfileMenuOpen};

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let open: RwSignal<bool> = RwSignal::new(false);
    provide_context(ProfileMenuOpen(open));

    view! {
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-40"
                on:click=move |_| open.set(false)
            />
        </Show>

        <Navbar/>
        {children()}
        <SiteFooter/>
    }
}
