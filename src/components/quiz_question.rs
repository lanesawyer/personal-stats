use leptos::*;

use crate::pages::quiz::QuizQuestionData;

#[component]
pub fn QuizQuestion(cx: Scope, question: QuizQuestionData) -> impl IntoView {
    view! {
        cx,
        <p>{question.text}</p>
        <For
            each=move || question.answers.clone()
            key=move |answer| answer.clone()
            view=move |cx, answer: String| {
                view! {
                    cx,
                        <input type="radio" id=answer.clone() name=question.id.clone() value=answer.clone() />
                        <label for=answer.clone()>{answer.clone()}</label><br />
                }
            }
        />
    }
}
