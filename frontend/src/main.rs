use seed::{prelude::*, *};

enum Msg {}
struct Model {}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}
fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

fn view(_: &Model) -> Node<Msg> {
    div!["Hello, World!"]
}

fn main() {
    App::start("app", init, update, view);
}
