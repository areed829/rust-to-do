use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                ctx.link().send_message(Msg::AddOne);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        html! {
            <div class="container">
              <p>{ self.count }</p>
              <button onclick={link.callback(|_| Msg::AddOne)}>{ "Click me!" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<CounterComponent>::new().render();
}
