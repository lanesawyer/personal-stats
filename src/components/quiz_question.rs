use leptos::*;

use crate::pages::quiz::QuizQuestionData;

#[component]
pub fn QuizQuestion(question: QuizQuestionData) -> impl IntoView {
    view! {
        <div style="margin: var(--padding)">
            <p>{question.text}</p>
            <For
                each=move || question.answers.clone()
                key=move |answer| answer.clone()
                children=move | answer: String| {
                    let id = format!("{}-{}", question.id.clone(), answer.clone());

                    view! {
                        
                        <input type="radio" id=id.clone() name=question.id.clone() value=answer.clone() />
                        <label for=id>{answer.clone()}</label>
                    }
                }
            />
        </div>
    }
}
