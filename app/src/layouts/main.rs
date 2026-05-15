use leptos::{children::Children, *};

use crate::components::sections::footer::SiteFooter;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        {children()}
        <SiteFooter/>
    }
}
