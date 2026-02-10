use std::rc::Rc;

use js_sys::Date;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

use crate::application::{AskQuestionUseCase, CheckHealthUseCase, UseCaseError};
use crate::config::AppConfig;
use crate::domain::HealthStatus;
use crate::infrastructure::ApiClient;

#[derive(Clone, Debug, PartialEq)]
enum LoadState<T> {
    Idle,
    Loading,
    Ready(T),
    Error(String),
}

impl<T> LoadState<T> {
    fn is_loading(&self) -> bool {
        matches!(self, LoadState::Loading)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HealthViewState {
    state: LoadState<HealthStatus>,
    last_checked: Option<String>,
}

impl HealthViewState {
    fn idle() -> Self {
        Self {
            state: LoadState::Idle,
            last_checked: None,
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let config = AppConfig::load();
    let initial_url = config.api_base_url.as_str().to_string();

    let question = use_state(|| String::new());
    let ask_state = use_state(|| LoadState::Idle);

    let api_base_url = use_state(|| initial_url.clone());
    let api_base_url_input = use_state(|| initial_url);
    let api_base_url_error = use_state(|| Option::<String>::None);
    let api_base_url_notice = use_state(|| Option::<String>::None);

    let health_state = use_state(HealthViewState::idle);

    let run_health_check: Rc<dyn Fn()> = {
        let api_base_url = api_base_url.clone();
        let health_state = health_state.clone();
        Rc::new(move || {
            let api_base_url = (*api_base_url).clone();
            let health_state = health_state.clone();
            spawn_local(async move {
                health_state.set(HealthViewState {
                    state: LoadState::Loading,
                    last_checked: None,
                });

                let client = match AppConfig::parse_base_url(&api_base_url) {
                    Ok(base_url) => ApiClient::new(base_url),
                    Err(error) => {
                        health_state.set(HealthViewState {
                            state: LoadState::Error(error.to_string()),
                            last_checked: None,
                        });
                        return;
                    }
                };

                let usecase = CheckHealthUseCase::new(client);
                match usecase.execute().await {
                    Ok(status) => {
                        health_state.set(HealthViewState {
                            state: LoadState::Ready(status),
                            last_checked: Some(now_label()),
                        });
                    }
                    Err(error) => {
                        health_state.set(HealthViewState {
                            state: LoadState::Error(error_message(error)),
                            last_checked: Some(now_label()),
                        });
                    }
                }
            });
        })
    };

    {
        let run_health_check = run_health_check.clone();
        let api_base_url = api_base_url.clone();
        use_effect_with((*api_base_url).clone(), move |_| {
            run_health_check();
            || ()
        });
    }

    let on_question_input = {
        let question = question.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            question.set(input.value());
        })
    };

    let on_submit = {
        let question = question.clone();
        let ask_state = ask_state.clone();
        let api_base_url = api_base_url.clone();
        Callback::from(move |_| {
            let question_value = (*question).clone();
            let ask_state = ask_state.clone();
            let api_base_url = (*api_base_url).clone();
            spawn_local(async move {
                ask_state.set(LoadState::Loading);

                let client = match AppConfig::parse_base_url(&api_base_url) {
                    Ok(base_url) => ApiClient::new(base_url),
                    Err(error) => {
                        ask_state.set(LoadState::Error(error.to_string()));
                        return;
                    }
                };

                let usecase = AskQuestionUseCase::new(client);
                match usecase.execute(question_value).await {
                    Ok(result) => {
                        ask_state.set(LoadState::Ready(result));
                    }
                    Err(error) => {
                        ask_state.set(LoadState::Error(error_message(error)));
                    }
                }
            });
        })
    };

    let on_base_url_input = {
        let api_base_url_input = api_base_url_input.clone();
        let api_base_url_error = api_base_url_error.clone();
        let api_base_url_notice = api_base_url_notice.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            api_base_url_input.set(input.value());
            api_base_url_error.set(None);
            api_base_url_notice.set(None);
        })
    };

    let on_base_url_save = {
        let api_base_url_input = api_base_url_input.clone();
        let api_base_url = api_base_url.clone();
        let api_base_url_error = api_base_url_error.clone();
        let api_base_url_notice = api_base_url_notice.clone();
        Callback::from(move |_| {
            let value = (*api_base_url_input).clone();
            match AppConfig::parse_base_url(&value) {
                Ok(parsed) => {
                    if let Err(error) = AppConfig::save_base_url(&parsed) {
                        api_base_url_error.set(Some(error));
                        api_base_url_notice.set(None);
                        return;
                    }
                    api_base_url.set(parsed.as_str().to_string());
                    api_base_url_error.set(None);
                    api_base_url_notice.set(Some("Базовый URL сохранен".to_string()));
                }
                Err(error) => {
                    api_base_url_error.set(Some(error.to_string()));
                    api_base_url_notice.set(None);
                }
            }
        })
    };

    let on_base_url_reset = {
        let api_base_url_input = api_base_url_input.clone();
        let api_base_url = api_base_url.clone();
        let api_base_url_error = api_base_url_error.clone();
        let api_base_url_notice = api_base_url_notice.clone();
        Callback::from(move |_| {
            let value = AppConfig::default_base_url();
            api_base_url_input.set(value.clone());
            match AppConfig::parse_base_url(&value) {
                Ok(parsed) => {
                    let _ = AppConfig::save_base_url(&parsed);
                    api_base_url.set(parsed.as_str().to_string());
                    api_base_url_error.set(None);
                    api_base_url_notice.set(Some("URL сброшен к значению по умолчанию".to_string()));
                }
                Err(error) => {
                    api_base_url_error.set(Some(error.to_string()));
                    api_base_url_notice.set(None);
                }
            }
        })
    };

    let on_health_refresh = {
        let run_health_check = run_health_check.clone();
        Callback::from(move |_| {
            run_health_check();
        })
    };

    let question_is_empty = question.trim().is_empty();

    html! {
        <div class="app">
            <header class="app__header">
                <div class="app__title">
                    <p class="app__eyebrow">{"Учебный UI / Rust + Yew"}</p>
                    <h1>{"GigaChat Webapp"}</h1>
                </div>
                <p class="app__subtitle">
                    {"Клиент к API "}
                    <span class="app__mono">{"rust-gigachat-app"}</span>
                    {". Используйте два процесса на разных портах."}
                </p>
            </header>

            <main class="app__main">
                <section class="panel panel--ask" aria-live="polite">
                    <div class="panel__header">
                        <h2>{"Задать вопрос"}</h2>
                        <p>{"Введите вопрос и отправьте его в API."}</p>
                    </div>

                    <label class="field" for="question">
                        <span class="field__label">{"Ваш вопрос"}</span>
                        <textarea
                            id="question"
                            class="field__input"
                            rows="4"
                            value={(*question).clone()}
                            placeholder="Например: Что такое Rocket?"
                            oninput={on_question_input}
                        />
                        <span class="field__hint">{"Пустые вопросы не отправляются."}</span>
                    </label>

                    <div class="actions">
                        <button
                            class="button"
                            disabled={question_is_empty || ask_state.is_loading()}
                            onclick={on_submit}
                        >
                            { if ask_state.is_loading() { "Отправка..." } else { "Отправить" } }
                        </button>
                        <span class="actions__note">{
                            format!("API: {}", api_base_url.as_str())
                        }</span>
                    </div>

                    <div class="response">
                        {match &*ask_state {
                            LoadState::Idle => html! {
                                <p class="muted">{"Ответ появится здесь."}</p>
                            },
                            LoadState::Loading => html! {
                                <div class="loading">
                                    <span class="spinner" aria-hidden="true"></span>
                                    <span>{"Ожидание ответа от сервера..."}</span>
                                </div>
                            },
                            LoadState::Ready(result) => html! {
                                <div class="answer fade-in">
                                    <p class="answer__text">{result.answer.clone()}</p>
                                    <div class="answer__meta">
                                        <span>{format!("Источник: {}", result.source)}</span>
                                        <span>{format!("Системный промпт применен: {}", yes_no(result.system_prompt_applied))}</span>
                                    </div>
                                </div>
                            },
                            LoadState::Error(error) => html! {
                                <div class="message message--error fade-in">
                                    <strong>{"Ошибка"}</strong>
                                    <span>{error.clone()}</span>
                                </div>
                            },
                        }}
                    </div>
                </section>

                <section class="panel panel--status" aria-live="polite">
                    <div class="panel__header">
                        <h2>{"Статус API"}</h2>
                        <p>{"Проверка доступности и режима работы сервера."}</p>
                    </div>

                    <label class="field" for="api-base-url">
                        <span class="field__label">{"Базовый URL API"}</span>
                        <input
                            id="api-base-url"
                            class="field__input"
                            type="text"
                            value={(*api_base_url_input).clone()}
                            oninput={on_base_url_input}
                        />
                        <span class="field__hint">{"Можно изменить без пересборки — значение хранится в браузере."}</span>
                    </label>

                    <div class="actions actions--compact">
                        <button class="button button--ghost" onclick={on_base_url_save}>{"Сохранить"}</button>
                        <button class="button button--ghost" onclick={on_base_url_reset}>{"Сбросить"}</button>
                        <button class="button" onclick={on_health_refresh}>{"Проверить"}</button>
                    </div>

                    {if let Some(message) = &*api_base_url_error {
                        html! { <div class="message message--error">{message.clone()}</div> }
                    } else {
                        html! {}
                    }}

                    {if let Some(message) = &*api_base_url_notice {
                        html! { <div class="message message--success">{message.clone()}</div> }
                    } else {
                        html! {}
                    }}

                    <div class="status">
                        {match &health_state.state {
                            LoadState::Idle => html! {
                                <p class="muted">{"Статус еще не запрошен."}</p>
                            },
                            LoadState::Loading => html! {
                                <div class="loading">
                                    <span class="spinner" aria-hidden="true"></span>
                                    <span>{"Проверяем..."}</span>
                                </div>
                            },
                            LoadState::Ready(status) => html! {
                                <div class="status__content fade-in">
                                    <div class="status__row">
                                        <span class={status_class(&status.status)}>{format!("{}", status.status)}</span>
                                        <span>{format!("Версия: {}", status.version)}</span>
                                    </div>
                                    <div class="status__row">
                                        <span>{format!("Режим: {}", mode_label(status.gigachat_enabled))}</span>
                                    </div>
                                </div>
                            },
                            LoadState::Error(error) => html! {
                                <div class="message message--error fade-in">
                                    <strong>{"Ошибка"}</strong>
                                    <span>{error.clone()}</span>
                                </div>
                            },
                        }}

                        {if let Some(checked) = &health_state.last_checked {
                            html! { <p class="muted">{format!("Последняя проверка: {}", checked)}</p> }
                        } else {
                            html! {}
                        }}
                    </div>
                </section>
            </main>
        </div>
    }
}

fn error_message(error: UseCaseError) -> String {
    error.to_string()
}

fn yes_no(value: bool) -> &'static str {
    if value { "да" } else { "нет" }
}

fn mode_label(gigachat_enabled: bool) -> &'static str {
    if gigachat_enabled { "GigaChat" } else { "mock" }
}

fn status_class(status: &str) -> &'static str {
    if status.eq_ignore_ascii_case("ok") {
        "pill pill--success"
    } else {
        "pill pill--warning"
    }
}

fn now_label() -> String {
    Date::new_0().to_string().into()
}
