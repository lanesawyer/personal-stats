use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{home::HomePage, profile::ProfilePage, quiz::QuizPage, stats::StatsPage};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Personal Stats"/>

        // content for this welcome page
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="quiz">"Quiz"</A>
                <A href="stats">"Stats"</A>
                <A href="profile">"Profile"</A>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage /> } />
                    <Route path="quiz" view=|cx| view! {cx, <QuizPage />} />
                    <Route path="stats" view=|cx| view! {cx, <StatsPage />} />
                    <Route path="profile" view=|cx| view! {cx, <ProfilePage />} />
                </Routes>
            </main>
        </Router>
    }
}
