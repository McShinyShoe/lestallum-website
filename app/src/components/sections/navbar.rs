use leptos::prelude::*;

const DISCORD_URL: &str = "https://lestallum.shinyshoe.net/discord";

#[derive(Clone, Copy)]
pub struct ProfileMenuOpen(pub RwSignal<bool>);

#[component]
pub fn Navbar() -> impl IntoView {
    let ProfileMenuOpen(open) =
        use_context::<ProfileMenuOpen>().expect("ProfileMenuOpen context must be provided");

    view! {
        <nav class="fixed inset-x-4 top-4 z-50">

            <div class="flex items-center justify-between rounded-2xl border border-white/10 bg-base-200/80 px-6 py-3 shadow-2xl backdrop-blur-md">

                <a href="/" class="flex items-center gap-2 text-lg font-bold tracking-tight">
                    <img src="/favicon-32x32.png" alt="✦" class="w-5 h-5" />
                    "Lestallum"
                </a>

                <ul class="flex items-center gap-1 text-sm font-medium">
                    <NavLink href="/">"Home"</NavLink>
                    <NavLink href="/areas">"Areas"</NavLink>
                    <NavLink href="/map">"Map"</NavLink>
                    <li>
                        <a
                            href=DISCORD_URL
                            target="_blank"
                            rel="noopener"
                            class="rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
                        >
                            "Discord"
                        </a>
                    </li>
                </ul>

                <button
                    on:click=move |_| open.update(|v| *v = !*v)
                    class="flex h-10 w-10 items-center justify-center rounded-full border border-white/10 bg-base-300 text-base-content/60 transition hover:border-emerald-500/40 hover:text-emerald-400"
                    aria-label="Profile options"
                >
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                        />
                    </svg>
                </button>

            </div>

            <Show when=move || open.get()>
                <div
                    on:click=|ev| ev.stop_propagation()
                    on:contextmenu=|ev| ev.prevent_default()
                    class="absolute right-0 top-full mt-2 w-52 overflow-hidden rounded-xl border border-white/10 bg-base-200/80 shadow-2xl backdrop-blur-md"
                >
                    <ProfileMenuSection label="Account">
                        <ProfileMenuItem icon="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1">
                            "Login"
                        </ProfileMenuItem>
                    </ProfileMenuSection>

                </div>
            </Show>

        </nav>
    }
}

#[component]
fn NavLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
            >
                {children()}
            </a>
        </li>
    }
}

#[component]
pub fn ProfileMenuSection(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="border-b border-white/5 last:border-b-0">
            <p class="px-4 pb-1 pt-3 text-xs font-semibold uppercase tracking-widest text-base-content/40">
                {label}
            </p>
            <ul class="p-1.5">
                {children()}
            </ul>
        </div>
    }
}

#[component]
pub fn ProfileMenuItem(icon: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <button class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-sm text-base-content/80 transition hover:bg-white/10 hover:text-base-content">
                <svg class="h-4 w-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon/>
                </svg>
                {children()}
            </button>
        </li>
    }
}
