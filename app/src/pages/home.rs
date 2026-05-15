use leptos::prelude::*;

use crate::layouts::main::MainLayout;

const DISCORD_URL: &str = "https://lestallum.shinyshoe.net/discord";

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <MainLayout>
            <Hero/>
            <About/>
            <AreasCarousel/>
            <Stats/>
        </MainLayout>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="relative h-screen w-full overflow-hidden">
            <div
                class="absolute inset-0 bg-cover bg-center"
                style="background-image: linear-gradient(135deg, #62739c 0%, #8994a8 40%, #49ac91 100%), url('/hero.webp'); background-blend-mode: multiply;"
            ></div>
            <div class="absolute inset-0 bg-gradient-to-b from-black/40 via-black/20 to-black/80"></div>

            <div class="relative z-10 flex h-full flex-col items-center justify-center px-6 text-center">
                <p class="mb-4 text-sm uppercase tracking-[0.4em] text-emerald-300/80">
                    "Welcome to"
                </p>
                <h1 class="bg-gradient-to-br from-white via-slate-100 to-emerald-200 bg-clip-text text-6xl font-extrabold tracking-tight text-transparent drop-shadow-2xl sm:text-8xl md:text-9xl">
                    "Lestallum Town"
                </h1>
                <div class="mt-6 h-px w-32 bg-gradient-to-r from-transparent via-emerald-400 to-transparent"></div>
            </div>

            <div class="absolute bottom-16 left-1/2 z-10 w-full -translate-x-1/2 px-6 text-center">
                <blockquote class="mx-auto mb-8 max-w-2xl text-lg italic text-slate-300 sm:text-xl">
                    "\"A haven of builders and fun loving peoples.\""
                </blockquote>
                <a
                    href=DISCORD_URL
                    target="_blank"
                    rel="noopener"
                    class="btn btn-primary btn-lg gap-2 rounded-full px-10 shadow-xl shadow-emerald-500/20 transition hover:scale-105"
                >
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M20.317 4.37a19.79 19.79 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.74 19.74 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028c.462-.63.873-1.295 1.226-1.994a.076.076 0 0 0-.041-.106a13.1 13.1 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.3 12.3 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.84 19.84 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.331c-1.182 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.974 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"/>
                    </svg>
                    "Join Now"
                </a>
            </div>

            <div class="absolute bottom-6 left-1/2 z-10 -translate-x-1/2 animate-bounce text-slate-400">
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
                </svg>
            </div>
        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <section class="bg-base-200 px-6 py-24 md:px-12">
            <div class="mx-auto grid max-w-6xl items-center gap-12 md:grid-cols-2">
                <div>
                    <span class="mb-4 inline-block rounded-full bg-emerald-500/10 px-4 py-1 text-xs font-semibold uppercase tracking-widest text-emerald-300">
                        "Our Town"
                    </span>
                    <h2 class="mb-6 text-4xl font-bold leading-tight md:text-5xl">
                        "A town for all style."
                    </h2>
                    <p class="mb-4 text-lg leading-relaxed text-base-content/70">
                        "Unlike ordinary towns that bounds to a single style, Lestallum is divided into several
                        unique town areas, each with its own identity and atmosphere. From the cobbled streets 
                        and towering keeps of the Medieval quarter, to the glowing skylines of the Futuristic district, 
                        every corner of the town tells a different story."
                    </p>
                </div>
                <div class="group relative aspect-video">
                    <div
                        class="absolute inset-0 bg-contain bg-center bg-no-repeat transition-transform duration-700 group-hover:scale-105"
                        style="background-image: url('/town.webp');"
                    ></div>
                </div>
            </div>
        </section>
    }
}

const AREAS: &[(&str, &str, &str)] = &[
    ("Arboria", "Semi-Modern", "/areas/arboria.webp"),
    (
        "Evergreen Garden",
        "Medieval",
        "/areas/evergreen_garden.webp",
    ),
    ("TsunamiCity", "Semi-Futuristic", "/areas/tsunami_city.webp"),
    ("Tropicana", "Random", "/areas/tropicana.webp"),
    ("Leville", "Random", "/areas/leville.webp"),
    ("Clintwood", "Random", "/areas/clintwood.webp"),
    ("OFC", "Dark Fantasy", "/areas/ofc.webp"),
    ("Hive", "Medieval", "/areas/hive.webp"),
    ("Oasis", "Medieval", "/areas/oasis.webp"),
];

fn area_bg(img: &str) -> String {
    format!(
        "background-image: url('{}');
         background-size: cover;
         background-position: center;",
        img
    )
}

#[component]
fn AreasCarousel() -> impl IntoView {
    let len = AREAS.len();
    let current = RwSignal::new(0usize);
    let go_prev = move |_| current.update(|c| *c = (*c + len - 1) % len);
    let go_next = move |_| current.update(|c| *c = (*c + 1) % len);

    let left = move || AREAS[(current.get() + len - 1) % len];
    let center = move || AREAS[current.get()];
    let right = move || AREAS[(current.get() + 1) % len];

    let mask_style = "mask-image: linear-gradient(to right, transparent 0%, black 18%, black 82%, transparent 100%); \
                      -webkit-mask-image: linear-gradient(to right, transparent 0%, black 18%, black 82%, transparent 100%);";

    view! {
        <section class="bg-base-300 px-6 py-24">
            <div class="mx-auto max-w-6xl">
                <div class="mb-10 text-center">
                    <span class="mb-4 inline-block rounded-full bg-emerald-500/10 px-4 py-1 text-xs font-semibold uppercase tracking-widest text-emerald-300">
                        "Explore"
                    </span>
                    <h2 class="mb-3 text-4xl font-bold md:text-5xl">"Areas of Lestallum"</h2>
                    <p class="text-base-content/60">
                        "Each district has its own flavor."
                    </p>
                </div>

                <div class="relative h-[32rem]">
                    <div class="absolute inset-0 overflow-hidden" style=mask_style>
                        <div
                            class="absolute left-0 top-1/2 h-80 w-80 -translate-x-1/2 -translate-y-1/2 overflow-hidden rounded-2xl shadow-xl ring-1 ring-white/10 transition-all duration-500"
                            style=move || area_bg(left().2)
                        >
                            <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/30 to-transparent"></div>
                            <div class="absolute bottom-0 left-0 right-0 p-5">
                                <span class="mb-2 inline-block rounded-full bg-emerald-500/20 px-3 py-1 text-xs font-semibold uppercase tracking-widest text-emerald-300 backdrop-blur">
                                    {move || left().1}
                                </span>
                                <h3 class="text-xl font-bold text-white drop-shadow-lg">
                                    {move || left().0}
                                </h3>
                            </div>
                        </div>

                        <div
                            class="absolute left-1/2 top-1/2 h-[28rem] w-[32rem] -translate-x-1/2 -translate-y-1/2 overflow-hidden rounded-2xl shadow-2xl ring-1 ring-emerald-500/20 transition-all duration-500"
                            style=move || area_bg(center().2)
                        >
                            <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/20 to-transparent"></div>
                            <div class="absolute bottom-0 left-0 right-0 p-7">
                                <span class="mb-3 inline-block rounded-full bg-emerald-500/30 px-4 py-1.5 text-xs font-bold uppercase tracking-widest text-emerald-200 backdrop-blur">
                                    {move || center().1}
                                </span>
                                <h3 class="text-3xl font-extrabold text-white drop-shadow-2xl md:text-4xl">
                                    {move || center().0}
                                </h3>
                            </div>
                        </div>

                        <div
                            class="absolute right-0 top-1/2 h-80 w-80 -translate-y-1/2 translate-x-1/2 overflow-hidden rounded-2xl shadow-xl ring-1 ring-white/10 transition-all duration-500"
                            style=move || area_bg(right().2)
                        >
                            <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/30 to-transparent"></div>
                            <div class="absolute bottom-0 left-0 right-0 p-5">
                                <span class="mb-2 inline-block rounded-full bg-emerald-500/20 px-3 py-1 text-xs font-semibold uppercase tracking-widest text-emerald-300 backdrop-blur">
                                    {move || right().1}
                                </span>
                                <h3 class="text-xl font-bold text-white drop-shadow-lg">
                                    {move || right().0}
                                </h3>
                            </div>
                        </div>
                    </div>

                    <button
                        on:click=go_prev
                        aria-label="Previous area"
                        class="btn btn-circle btn-primary absolute left-2 top-1/2 z-20 -translate-y-1/2 shadow-xl sm:left-6"
                    >
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                    </button>
                    <button
                        on:click=go_next
                        aria-label="Next area"
                        class="btn btn-circle btn-primary absolute right-2 top-1/2 z-20 -translate-y-1/2 shadow-xl sm:right-6"
                    >
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                        </svg>
                    </button>
                </div>

                <div class="mt-8 flex justify-center gap-2">
                    {(0..len).map(|i| {
                        let dot_class = move || {
                            if current.get() == i {
                                "h-2 w-8 rounded-full bg-emerald-400 transition-all duration-300"
                            } else {
                                "h-2 w-2 rounded-full bg-white/20 transition-all duration-300"
                            }
                        };
                        view! {
                            <button
                                on:click=move |_| current.set(i)
                                aria-label=format!("Go to area {}", i + 1)
                                class=dot_class
                            ></button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn Stats() -> impl IntoView {
    view! {
        <section class="bg-base-100 px-6 py-24">
            <div class="mx-auto max-w-6xl text-center">
                <h2 class="mb-4 text-4xl font-bold md:text-5xl">"By the Numbers"</h2>
                <p class="mb-12 text-base-content/60">
                    "Years of building, growing, and exploring together."
                </p>
                <div class="grid gap-6 md:grid-cols-3">
                    <StatCard
                        label="Residents"
                        value="30"
                        sublabel="peoples"
                    />
                    <StatCard
                        label="Town Size"
                        value="2,108"
                        sublabel="chunks across"
                    />
                    <StatCard
                        label="Founded"
                        value="Oct 31"
                        sublabel="2021"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn StatCard(label: &'static str, value: &'static str, sublabel: &'static str) -> impl IntoView {
    view! {
        <div class="group rounded-2xl border border-white/5 bg-base-200 p-8 shadow-lg transition hover:border-emerald-500/30 hover:shadow-emerald-500/10">
            <div class="mb-2 text-sm font-semibold uppercase tracking-widest text-emerald-400">
                {label}
            </div>
            <div class="mb-1 bg-gradient-to-br from-white to-slate-400 bg-clip-text text-5xl font-extrabold text-transparent md:text-6xl">
                {value}
            </div>
            <div class="text-sm text-base-content/50">{sublabel}</div>
        </div>
    }
}
