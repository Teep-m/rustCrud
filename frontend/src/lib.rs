use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());
    let (users , set_users) = create_signal(Vec::<User>::new());

    // Fetch todos on load
    create_effect(move |_| {
        spawn_local(async move {
            let fetched_todos: Vec<Todo> = Request::get("/api/todos")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            set_todos.set(fetched_todos);
        });
    });

    let add_todo = move |title: String| {
        spawn_local(async move {
            let new_todo = serde_json::json!({
                "title": title,
                "completed": false
            });
            let res = Request::post("/api/todos")
                .json(&new_todo)
                .unwrap()
                .send()
                .await
                .unwrap();
            
            if res.ok() {
                let todo: Todo = res.json().await.unwrap();
                set_todos.update(|t| t.push(todo));
            }
        });
    };

    let toggle_todo = move |id: i32, completed: bool| {
        spawn_local(async move {
            let update = serde_json::json!({ "completed": !completed });
            let res = Request::put(&format!("/api/todos/{}", id))
                .json(&update)
                .unwrap()
                .send()
                .await
                .unwrap();

            if res.ok() {
                set_todos.update(|t| {
                    if let Some(todo) = t.iter_mut().find(|t| t.id == id) {
                        todo.completed = !completed;
                    }
                });
            }
        });
    };

    let delete_todo = move |id: i32| {
        spawn_local(async move {
            let res = Request::delete(&format!("/api/todos/{}", id))
                .send()
                .await
                .unwrap();

            if res.ok() {
                set_todos.update(|t| t.retain(|todo| todo.id != id));
            }
        });
    };

    view! {
        <div class="container">
            <h1>"Todo App"</h1>
            <div class="input-group">
                <textarea id="new-todo" placeholder="Add a new todo..."
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" && !ev.shift_key() {
                            ev.prevent_default();
                            let input = event_target::<web_sys::HtmlTextAreaElement>(&ev);
                            let value = input.value();
                            if !value.is_empty() {
                                add_todo(value);
                                input.set_value("");
                            }
                        }
                    }
                />
            </div>
            <ul class="todo-list">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    children=move |todo| {
                        let title = todo.title.clone();
                        view! {
                            <li class={if todo.completed { "completed" } else { "" }}>
                                <div class="todo-text" on:click=move |_| toggle_todo(todo.id, todo.completed)>
                                    {title}
                                </div>
                                <button on:click=move |_| delete_todo(todo.id)>"X"</button>
                                <p>{todo.id}</p>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
