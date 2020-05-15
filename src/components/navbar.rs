use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

use crate::app::AppRoute;

pub struct Navbar {
    _router_agent: RouteAgentBridge<()>,
    route: Option<Route<()>>,
}

#[derive(Debug)]
pub enum Msg {
    RouteChanged(Route<()>),
}

impl Navbar {
    const DEFAULT_CLASSES: &'static str = "button is-outlined";
    const ACTIVE_CLASSES: &'static str = " is-active";
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::RouteChanged);
        let mut _router_agent = RouteAgentBridge::new(callback);
        _router_agent.send(RouteRequest::GetCurrentRoute);

        Self {
            _router_agent,
            route: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = Some(route),
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut about_classes = Self::DEFAULT_CLASSES.to_owned();
        let mut docs_classes = Self::DEFAULT_CLASSES.to_owned();

        if let Some(route) = &self.route {
            match route.route.as_ref() {
                "/about" => about_classes.push_str(Self::ACTIVE_CLASSES),
                "/docs" => docs_classes.push_str(Self::ACTIVE_CLASSES),
                _ => (),
            }
        }

        html! {
            <nav class="navbar is-primary">
                <div class="container">
                    <div class="navbar-brand">
                        <RouterAnchor<AppRoute> route=AppRoute::Index classes="navbar-item">
                            { "FuzzySearch Beta" }
                        </RouterAnchor<AppRoute>>
                    </div>

                    <div class="navbar-menu">
                        <div class="navbar-end">
                            <div class="navbar-item">
                                <div class="buttons">
                                    <RouterAnchor<AppRoute> route=AppRoute::About classes=about_classes>{ "About" }</RouterAnchor<AppRoute>>
                                    <RouterAnchor<AppRoute> route=AppRoute::ApiDocs classes=docs_classes>{ "API Docs" }</RouterAnchor<AppRoute>>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
