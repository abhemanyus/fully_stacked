use seed::{prelude::*, *};

pub enum Msg {}
pub struct Model {}

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

pub fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

pub fn view(_: &Model) -> Node<Msg> {
    div!["Hello from Not Found!"]
}
