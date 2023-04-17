use leptos::*;

#[component]
pub fn ProfilePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Profile"</h1>
        <p>"This page will let you manage your settings."</p>
    }
}
