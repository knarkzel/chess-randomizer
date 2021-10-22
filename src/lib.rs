use rand::Rng;
use reformation::Reformation;

use seed::{prelude::*, *};

#[derive(Debug, Default)]
struct Model {
    openings: Vec<Opening>,
}

#[derive(Clone)]
enum Msg {
    Fetch,
    Obtained(String),
}

#[derive(Debug, Reformation)]
#[reformation("{link},,,{name}")]
struct Opening {
    name: String,
    link: String,
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::Fetch });
    Model::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let mut rand = rand::thread_rng();
    match msg {
        Msg::Fetch => {
            let file = rand.gen_range(0, 1212);

            let request = Request::new(format!("openings/x{:0>4}", file))
                .method(Method::Get)
                .mode(web_sys::RequestMode::NoCors);

            orders.perform_cmd(async {
                let response = fetch(request)
                    .await
                    .expect("HTTP request failed")
                    .text()
                    .await
                    .unwrap();
                Msg::Obtained(response)
            });
        }

        Msg::Obtained(data) => {
            let amount = rand.gen_range(0, 90);

            let mut openings = data
                .lines()
                .skip(amount)
                .take(9)
                .flat_map(Opening::parse)
                .collect::<Vec<_>>();
            openings.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
            openings.dedup_by(|a, b| a.name.eq(&b.name));
            openings.sort_by(|a, b| a.name.len().partial_cmp(&b.name.len()).unwrap());

            model.openings = openings;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        button!["Fetch openings!", ev(Ev::Click, |_| Msg::Fetch),],
        ol![
            model.openings.iter().map(|opening| {
                li![
                    format!("{} [", &opening.name),
                    a![attrs!(At::Href => opening.link, At::Target => "blank"), "L"],
                    ",",
                    a![attrs!(At::Href => format!("https://en.wikipedia.org/w/index.php?search={}", &opening.name), At::Target => "blank"), "W"],
                    ",",
                    a![attrs!(At::Href => format!("https://www.google.com/search?q={}", &opening.name), At::Target => "blank"), "G"],
                    "]",
                ]
            })
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
