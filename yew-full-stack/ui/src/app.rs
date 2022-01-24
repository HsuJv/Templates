use yew::prelude::*;

pub enum AppMsg {}

pub struct App {}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div class="horizontal-centre vertical-centre">
                <p> {"Hello World"} </p>
            </div>
        }
    }
}
