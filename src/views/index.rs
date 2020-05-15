use yew::prelude::*;

use crate::agents::event_bus::*;
use crate::components::ImageHash;

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        EventBus::dispatcher().send(Request::SetState(State { blob_url: None }));

        Self
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <ImageHash redirect=true />
                </div>
            </div>
        }
    }
}
