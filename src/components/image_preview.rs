use std::rc::Rc;
use yew::prelude::*;
use yew::{agent::Bridged, services::reader::File};

use crate::agents::event_bus::*;

#[derive(Debug)]
pub struct BlobUrl {
    pub blob_url: String,
    pub name: String,
}

impl BlobUrl {
    pub fn new(file: &File) -> Self {
        let name = file.name();
        let blob_url = yew::web_sys::Url::create_object_url_with_blob(&file).unwrap();
        log::debug!("Created new BlobUrl: {}", blob_url);

        Self { blob_url, name }
    }
}

impl Drop for BlobUrl {
    fn drop(&mut self) {
        log::debug!("Revoking BlobUrl: {}", self.blob_url);
        yew::web_sys::Url::revoke_object_url(&self.blob_url).unwrap();
    }
}

impl ToString for BlobUrl {
    fn to_string(&self) -> String {
        self.blob_url.clone()
    }
}

pub struct ImagePreview {
    blob_url: Option<Rc<BlobUrl>>,
    _producer: Box<dyn Bridge<EventBus>>,
}

#[derive(Debug)]
pub enum Msg {
    NewState(State),
}

impl Component for ImagePreview {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::NewState);
        let _producer = EventBus::bridge(callback);

        Self {
            blob_url: None,
            _producer,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewState(state) => self.blob_url = state.blob_url,
        }

        true
    }

    fn view(&self) -> Html {
        let url = match &self.blob_url {
            Some(url) => url,
            None => return html! {},
        };

        html! {
            <figure class="image">
                <img src=url />
            </figure>
        }
    }
}
