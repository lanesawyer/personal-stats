use leptos::*;

use crate::components::quiz_question::QuizQuestion;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct QuizQuestionData {
    pub id: String,
    pub text: String,
    pub answers: Vec<String>,
}

#[component]
pub fn QuizPage(cx: Scope) -> impl IntoView {
    // TODO: From config file
    let answers: Vec<String> = vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
    ];

    let questions: Vec<QuizQuestionData> = vec![
        QuizQuestionData {
            id: "question-1".to_string(),
            text: "Sadness: Have you been feeling sad or down in the dumps?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-2".to_string(),
            text: "Discouragement: Does the future look hopeless?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-3".to_string(),
            text: "Low self-esteem: Do you feel worthless or think of yourself as a failure?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-4".to_string(),
            text: "Inferiority: Do you feel inadequate or inferior to others?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-5".to_string(),
            text: "Guilt: Do you get self-critical and blame yourself for everything?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-6".to_string(),
            text: "Indecisiveness: Do you have trouble making up your mind about things? ".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-7".to_string(),
            text: "Irritability and frustration: Have you been feeling resentful and angry a good deal of the time?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-8".to_string(),
            text: "Loss of interest in life: Have you lost interest in your career, your hobbies, your family, or your friends?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-9".to_string(),
            text: "Loss of motivation: Do you feel overwhelmed and have to push yourself hard to do things?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-10".to_string(),
            text: "Poor self-image: Do you think you’re looking old or unattractive?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-11".to_string(),
            text: "Appetite changes: Have you lost your appetite? Or do you overeat or binge compulsively?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-12".to_string(),
            text: "Sleep changes: Do you suffer from insomnia and find it hard to get a good night’s sleep? Or are you excessively tired and sleeping too much?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-13".to_string(),
            text: "Loss of libido: Have you lost your interest in sex?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-14".to_string(),
            text: "Hypochondriasis: Do you worry a great deal about your health?".to_string(),
            answers: answers.clone(),
        },
        QuizQuestionData {
            id: "question-15".to_string(),
            text: "Suicidal impulses: Do you have thoughts that life is not worth living or think that you might be better off dead?".to_string(),
            answers,
        },
    ];

    view! { cx,
        <h1>"Quiz"</h1>
        <form action="/api/quiz" method="post">
            <For
                each=move || questions.clone()
                key=move |question| question.clone()
                view=move |cx, question: QuizQuestionData| {
                    view! {
                        cx,
                        <QuizQuestion question />
                    }
                }
            />
            <input type="submit" value="Submit Quiz" />
        </form>
    }
}
