use leptos::*;

use crate::pages::quiz::QuizQuestionData;

#[component]
pub fn QuizQuestion(cx: Scope, question: QuizQuestionData) -> impl IntoView {
    view! {
        cx,
        <div style="margin: var(--padding)">
            <p>{question.text}</p>
            <For
                each=move || question.answers.clone()
                key=move |answer| answer.clone()
                view=move |cx, answer: String| {
                    let id = format!("{}-{}", question.id.clone(), answer.clone());

                    view! {
                        cx,
                        <input type="radio" id=id.clone() name=question.id.clone() value=answer.clone() />
                        <label for=id>{answer.clone()}</label>
                    }
                }
            />
        </div>
    }
}
