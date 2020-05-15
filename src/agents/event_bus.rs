use std::collections::HashSet;
use std::rc::Rc;
use yew::worker::*;

use crate::components::BlobUrl;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub blob_url: Option<Rc<BlobUrl>>,
    pub latest_hash: Option<i64>,
}

#[derive(Debug)]
pub enum Request {
    ClearState,
    SetBlobUrl(Option<Rc<BlobUrl>>),
    SetLatestHash(Option<i64>),
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
            Request::ClearState => self.state = State::default(),
            Request::SetBlobUrl(blob_url) => self.state.blob_url = blob_url,
            Request::SetLatestHash(hash) => self.state.latest_hash = hash,
        }

        for sub in &self.subscribers {
            self.link.respond(*sub, self.state.clone());
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
