#[macro_use]
extern crate yew;

use yew::html::*;


struct Model {
    count: i64
}


enum Message {
    Increment,
    Decrement,
}


fn update(_context: &mut Context<Message>, model: &mut Model, message: Message) {
    match message {
        Message::Increment => {
            model.count = model.count + 1;
        }
        Message::Decrement => {
            model.count = model.count - 1;
        }
    }
}


fn view(model: &Model) -> Html<Message> {
    html! {
        <div>
            <nav class="menu",>
                <button onclick=|_| Message::Increment,>{ "Up" }</button>
                <button onclick=|_| Message::Decrement,>{ "Down" }</button>
            </nav>
            <p>{ model.count }</p>
        </div>
    }
}


fn main() {
    let model = Model {count: 1};
    program(model, update, view);
}