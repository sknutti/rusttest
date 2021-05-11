use seed::{self, prelude::*, *};

#[derive(Default)]
struct Model {
    items: Vec<String>,
    new_value: String,
    error: Option<String>,
}

enum Msg {
    Save,
    Clear,
    Remove(String),
    TextEntered(String),
    FetchedItems(fetch::Result<Vec<String>>),
    SaveCompleted(fetch::Result<Vec<String>>),
    ClearCompleted(fetch::Result<Vec<String>>),
    RemoveCompleted(fetch::Result<Vec<String>>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        Msg::Save => {
            let value = model.new_value.clone();
            model.new_value = String::from("");
            orders
                .skip()
                .perform_cmd(async { Msg::SaveCompleted(add_todo_item(value).await) });
        }
        Clear => {
            orders
                .skip()
                .perform_cmd(async { Msg::ClearCompleted(remove_todo_items().await) });
        }
        Remove(resp) => {
            orders.perform_cmd(async { Msg::RemoveCompleted(remove_todo_item(resp).await) });
        }
        TextEntered(value) => model.new_value = value,
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        SaveCompleted(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        ClearCompleted(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        RemoveCompleted(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div![img![
            C!["comment-author-img"],
            attrs! {At::Src => "https://cameras.liveviewtech.com/img/LVLogo_small.png"}
        ]],
        div![input![
            C!["form-control", "form-control-lg"],
            input_ev(Ev::Input, Msg::TextEntered),
            attrs! {
                At::Type => "text",
                At::Placeholder => "Enter some text...",
                At::Value => model.new_value
            },
        ]],
        div![
            button![
                C!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                "Save",
                ev(Ev::Click, |_| Msg::Save)
            ],
            button![
                C!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                "Clear",
                ev(Ev::Click, |_| Msg::Clear)
            ],
        ],
        div![ul![model.items.iter().map(|item| {
            let it = item.clone();
            div![
                style! {
                    St::Display => "flex",
                    St::Margin => px(10) + " " + &px(0),
                    St::PaddingRight => px(20)
                },
                li![
                    style! {
                        St::MinWidth => unit!(10, %),
                        St::PaddingRight => px(20) + " " + &px(10)
                    },
                    item
                ],
                button![
                    C!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                    "Delete",
                    ev(Ev::Click, |_| Msg::Remove(it))
                ]
            ]
        })]]
    ]
}

async fn get_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn add_todo_item(value: String) -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(Method::Post)
        .json(&value)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn remove_todo_item(value: String) -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(Method::Delete)
        .json(&value)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn remove_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Patch)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
