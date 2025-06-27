use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Model structs from the backend
#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Question {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct QuestionDetail {
    question_uuid: String,
    title: String,
    description: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct QuestionId {
    question_uuid: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Answer {
    question_uuid: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct AnswerDetail {
    answer_uuid: String,
    question_uuid: String,
    content: String,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
struct AnswerId {
    answer_uuid: String,
}

// Component states
enum Modal {
    None,
    NewQuestion,
    ViewQuestion(QuestionDetail),
}

struct App {
    questions: Vec<QuestionDetail>,
    answers: Vec<AnswerDetail>,
    modal: Modal,
    new_question_title: String,
    new_question_description: String,
    new_answer_content: String,
}

enum Msg {
    LoadQuestions,
    QuestionsLoaded(Vec<QuestionDetail>),
    LoadAnswers(String),
    AnswersLoaded(Vec<AnswerDetail>),
    ShowNewQuestionModal,
    CloseModal,
    UpdateNewQuestionTitle(String),
    UpdateNewQuestionDescription(String),
    SubmitNewQuestion,
    ViewQuestion(QuestionDetail),
    DeleteQuestion(String),
    DeleteAnswer(String),
    CreateAnswer(String),
    UpdateNewAnswerContent(String),
    SubmitAnswer(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadQuestions);
        Self {
            questions: vec![],
            answers: vec![],
            modal: Modal::None,
            new_question_title: String::new(),
            new_question_description: String::new(),
            new_answer_content: String::new(), // Add this
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadQuestions => {
                ctx.link().send_future(async {
                    let questions: Vec<QuestionDetail> =
                        Request::get("http://localhost:8000/questions")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    Msg::QuestionsLoaded(questions)
                });
                false
            }
            Msg::QuestionsLoaded(questions) => {
                self.questions = questions;
                true
            }
            Msg::LoadAnswers(question_uuid) => {
                ctx.link().send_future(async move {
                    let answers: Vec<AnswerDetail> =
                        Request::post("http://localhost:8000/answers") // Changed from GET to POST
                            .json(&QuestionId { question_uuid })
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    Msg::AnswersLoaded(answers)
                });
                false
            }

            Msg::AnswersLoaded(answers) => {
                self.answers = answers;
                true
            }
            Msg::ShowNewQuestionModal => {
                self.modal = Modal::NewQuestion;
                true
            }
            Msg::CloseModal => {
                self.modal = Modal::None;
                self.new_question_title.clear();
                self.new_question_description.clear();
                self.new_answer_content.clear(); // Clear answer content too
                true
            }
            Msg::UpdateNewQuestionTitle(title) => {
                self.new_question_title = title;
                true
            }
            Msg::UpdateNewQuestionDescription(description) => {
                self.new_question_description = description;
                true
            }
            Msg::SubmitNewQuestion => {
                let question = Question {
                    title: self.new_question_title.clone(),
                    description: self.new_question_description.clone(),
                };
                ctx.link().send_future(async move {
                    Request::post("http://localhost:8000/question")
                        .json(&question)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    Msg::LoadQuestions
                });
                self.modal = Modal::None;
                false
            }
            Msg::ViewQuestion(question) => {
                self.modal = Modal::ViewQuestion(question.clone());
                ctx.link()
                    .send_message(Msg::LoadAnswers(question.question_uuid));
                true
            }
            Msg::DeleteQuestion(question_uuid) => {
                ctx.link().send_future(async move {
                    Request::delete("http://localhost:8000/question")
                        .json(&QuestionId { question_uuid })
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    Msg::LoadQuestions
                });
                false
            }
            Msg::DeleteAnswer(answer_uuid) => {
                let question_uuid = match &self.modal {
                    Modal::ViewQuestion(q) => Some(q.question_uuid.clone()),
                    _ => None,
                };

                ctx.link().send_future(async move {
                    Request::delete("http://localhost:8000/answer")
                        .json(&AnswerId { answer_uuid })
                        .unwrap()
                        .send()
                        .await
                        .unwrap();

                    if let Some(qid) = question_uuid {
                        Msg::LoadAnswers(qid)
                    } else {
                        Msg::LoadQuestions
                    }
                });
                false
            }
            Msg::UpdateNewAnswerContent(content) => {
                self.new_answer_content = content;
                true
            }
            Msg::SubmitAnswer(question_uuid) => {
                let answer = Answer {
                    question_uuid: question_uuid.clone(),
                    content: self.new_answer_content.clone(),
                };

                ctx.link().send_future(async move {
                    Request::post("http://localhost:8000/answer")
                        .json(&answer)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    Msg::LoadAnswers(question_uuid)
                });

                self.new_answer_content.clear();
                false
            }
            Msg::CreateAnswer(_) => {
                // This is handled by UpdateNewAnswerContent and SubmitAnswer
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <header>
                    <h1>{ "Stack Overflow Clone" }</h1>
                    <button onclick={ctx.link().callback(|_| Msg::ShowNewQuestionModal)}>
                        { "New Question" }
                    </button>
                </header>
                { self.view_questions(ctx) }
                { self.view_modal(ctx) }
            </div>
        }
    }
}

impl App {
    fn view_questions(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="questions">
                { for self.questions.iter().map(|question| self.view_question_card(ctx, question)) }
            </div>
        }
    }

    fn view_question_card(&self, ctx: &Context<Self>, question: &QuestionDetail) -> Html {
        let question_clone = question.clone();
        let question_uuid = question.question_uuid.clone();

        html! {
            <div class="question-card">
                <h3>{ &question.title }</h3>
                <p>{ &question.description }</p>
                <div class="button-group">
                    <button onclick={ctx.link().callback(move |_| Msg::ViewQuestion(question_clone.clone()))}>
                        { "View" }
                    </button>
                    <button onclick={ctx.link().callback(move |_| Msg::DeleteQuestion(question_uuid.clone()))}>
                        { "Delete" }
                    </button>
                </div>
            </div>
        }
    }

    fn view_modal(&self, ctx: &Context<Self>) -> Html {
        match &self.modal {
            Modal::None => html! {},
            Modal::NewQuestion => self.view_new_question_modal(ctx),
            Modal::ViewQuestion(question) => self.view_question_detail_modal(ctx, question),
        }
    }

    fn view_new_question_modal(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="modal">
                <div class="modal-content">
                    <h2>{ "New Question" }</h2>
                    <input
                        type="text"
                        placeholder="Title"
                        value={self.new_question_title.clone()}
                        onchange={ctx.link().callback(|e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateNewQuestionTitle(input.value())
                        })}
                    />
                    <textarea
                        placeholder="Description"
                        value={self.new_question_description.clone()}
                        onchange={ctx.link().callback(|e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateNewQuestionDescription(input.value())
                        })}
                    />
                    <div class="button-group">
                        <button onclick={ctx.link().callback(|_| Msg::CloseModal)}>
                            { "Cancel" }
                        </button>
                        <button onclick={ctx.link().callback(|_| Msg::SubmitNewQuestion)}>
                            { "Submit" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn view_question_detail_modal(&self, ctx: &Context<Self>, question: &QuestionDetail) -> Html {
        let question_uuid = question.question_uuid.clone();

        html! {
            <div class="modal">
                <div class="modal-content">
                    <h2>{ &question.title }</h2>
                    <p>{ &question.description }</p>
                    <div class="answers">
                        <h3>{ "Answers" }</h3>
                        <div class="new-answer">
                            <textarea
                                placeholder="Your answer"
                                value={self.new_answer_content.clone()}
                                onchange={ctx.link().callback(|e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateNewAnswerContent(input.value())
                                })}
                            />
                            <button onclick={
                                let question_uuid = question_uuid.clone();
                                ctx.link().callback(move |_| Msg::SubmitAnswer(question_uuid.clone()))
                            }>
                                { "Submit Answer" }
                            </button>
                        </div>
                        { for self.answers.iter().map(|answer| {
                            let answer_uuid = answer.answer_uuid.clone();
                            html! {
                                <div class="answer">
                                    <p>{ &answer.content }</p>
                                    <button onclick={ctx.link().callback(move |_| Msg::DeleteAnswer(answer_uuid.clone()))}>
                                        { "Delete" }
                                    </button>
                                </div>
                            }
                        })}
                    </div>
                    <button onclick={ctx.link().callback(|_| Msg::CloseModal)}>
                        { "Close" }
                    </button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
