use seed::{prelude::*, *};

pub enum Msg {
    SetMessage(Option<String>),
}
pub struct Model {
    message: Option<String>,
}

async fn get_message() -> Option<String> {
    let res = fetch("/api/hello").await.ok()?;
    let msg = res.text().await.ok()?;
    Some(msg)
}

pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::SetMessage(get_message().await) });
    Model { message: None }
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetMessage(message) => model.message = message,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![model.message.as_ref()]
}
