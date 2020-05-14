use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive, Switch};

use crate::components::Navbar;
use crate::views::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,

    #[to = "/docs"]
    ApiDocs,

    #[to = "/results/{hash}"]
    Results(i64),

    #[to = "/404"]
    PageNotFound(Permissive<String>),

    #[to = "/"]
    Index,
}

pub struct Model;

impl Model {
    fn render(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Index => html! { <Index /> },
            AppRoute::About => html! { <h1>{ "About" }</h1> },
            AppRoute::ApiDocs => html! { <h1>{ "API Docs" }</h1> },
            AppRoute::Results(hash) => html! { <Results hash=hash /> },
            AppRoute::PageNotFound(_) => html! { <h1>{ "Page Not Found" }</h1> },
        }
    }

    fn redirect(route: Route) -> AppRoute {
        AppRoute::PageNotFound(Permissive(Some(route.route)))
    }
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        log::debug!("Rendering Model");

        html! {
            <div>
                <Navbar />

                <Router<AppRoute>
                    render = Router::render(Self::render)
                    redirect = Router::redirect(Self::redirect)
                />
            </div>
        }
    }
}
