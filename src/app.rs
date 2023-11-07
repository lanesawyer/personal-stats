use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{home::HomePage, profile::ProfilePage, quiz::QuizPage, stats::StatsPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/personal_stats.css"/>

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
                    <Route path="/" view=|| view! {  <HomePage /> } />
                    <Route path="quiz" view=|| view! { <QuizPage />} />
                    <Route path="stats" view=|| view! { <StatsPage />} />
                    <Route path="profile" view=|| view! { <ProfilePage />} />
                </Routes>
            </main>
        </Router>
    }
}
