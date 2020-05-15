use std::collections::HashSet;
use std::rc::Rc;
use yew::worker::*;

use crate::components::BlobUrl;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub blob_url: Option<Rc<BlobUrl>>,
}

#[derive(Debug)]
pub enum Request {
    SetState(State),
}

pub struct EventBus {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,

    state: State,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = State;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),

            state: State::default(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        log::debug!("Got input on EventBus: {:?}", msg);

        match msg {
            Request::SetState(state) => {
                for sub in &self.subscribers {
                    self.link.respond(*sub, state.clone());
                }

                self.state = state;
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link.respond(id, self.state.clone());

        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
