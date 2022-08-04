use seed::{prelude::*, *};

mod page;

const HOME: &str = "home";
const ABOUT: &str = "about";

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url().add_path_part(HOME)
    }
    fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
    fn not_found(self) -> Url {
        self.base_url().add_path_part("404")
    }
}

enum Page {
    Home(page::home::Model),
    About(page::about::Model),
    NotFound(page::not_found::Model),
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [HOME] => Self::Home(page::home::init(url, &mut orders.proxy(Msg::Home))),
            [ABOUT] => Self::About(page::about::init(url, &mut orders.proxy(Msg::About))),
            _ => Self::NotFound(page::not_found::init(url, &mut orders.proxy(Msg::NotFound))),
        }
    }
}

enum Msg {
    Home(page::home::Msg),
    About(page::about::Msg),
    NotFound(page::not_found::Msg),

    UrlChange(subs::UrlChanged),
}
struct Model {
    page: Page,
    base_url: Url,
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChange);
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Home(msg) => {
            if let Page::Home(model) = &mut model.page {
                page::home::update(msg, model, &mut orders.proxy(Msg::Home))
            }
        }
        Msg::About(msg) => {
            if let Page::About(model) = &mut model.page {
                page::about::update(msg, model, &mut orders.proxy(Msg::About))
            }
        }
        Msg::NotFound(msg) => {
            if let Page::NotFound(model) = &mut model.page {
                page::not_found::update(msg, model, &mut orders.proxy(Msg::NotFound))
            }
        }
        Msg::UrlChange(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    let base_url = &model.base_url;
    nodes![
        ol![[
            Urls::new(base_url).home(),
            Urls::new(base_url).about(),
            Urls::new(base_url).not_found()
        ]
        .into_iter()
        .map(|page| li![a![page.to_string(), attrs! {At::Href => page.to_string()}]])],
        match &model.page {
            Page::Home(model) => page::home::view(model).map_msg(Msg::Home),
            Page::About(model) => page::about::view(model).map_msg(Msg::About),
            Page::NotFound(model) => page::not_found::view(model).map_msg(Msg::NotFound),
        }
    ]
}

fn main() {
    App::start("app", init, update, view);
}
