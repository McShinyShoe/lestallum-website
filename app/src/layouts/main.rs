use leptos::prelude::*;

use crate::components::sections::footer::SiteFooter;
use crate::components::sections::navbar::{InfoOpen, MobileMenuOpen, Navbar, ProfileMenuOpen};

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let profile_open: RwSignal<bool> = RwSignal::new(false);
    let mobile_open: RwSignal<bool> = RwSignal::new(false);
    let info_open: RwSignal<bool> = RwSignal::new(false);
    provide_context(ProfileMenuOpen(profile_open));
    provide_context(MobileMenuOpen(mobile_open));
    provide_context(InfoOpen(info_open));

    view! {
        <Show when=move || profile_open.get() || mobile_open.get() || info_open.get()>
            <div
                class="fixed inset-0 z-40"
                on:click=move |_| {
                    profile_open.set(false);
                    mobile_open.set(false);
                    info_open.set(false);
                }
            />
        </Show>

        <Navbar/>
        {children()}
        <SiteFooter/>
    }
}
