use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::ImageHash;

pub struct Index {
    link: ComponentLink<Self>,

    hash: Option<anyhow::Result<i64>>,
}

pub enum Msg {
    Hash(anyhow::Result<i64>),
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, hash: None }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hash(hash) => {
                if let Ok(hash) = hash {
                    let route = Route::<()>::from(AppRoute::Results(hash));
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(route));
                }

                self.hash = Some(hash);
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <ImageHash
                        onhash=self.link.callback(move |hash| Msg::Hash(hash))
                    />
                </div>

                <div>
                    { format!("{:?}", self.hash) }
                </div>
            </div>
        }
    }
}
