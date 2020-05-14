use std::rc::Rc;
use yew::prelude::*;
use yew::{agent::Bridged, services::reader::File};

use crate::agents::event_bus::*;

#[derive(Debug)]
pub struct BlobUrl(String);

impl BlobUrl {
    pub fn new(file: &File) -> Self {
        let url = yew::web_sys::Url::create_object_url_with_blob(&file).unwrap();
        log::trace!("Created new BlobUrl: {}", url);
        Self(url)
    }
}

impl Drop for BlobUrl {
    fn drop(&mut self) {
        log::trace!("Revoking BlobUrl: {}", self.0);
        yew::web_sys::Url::revoke_object_url(&self.0).unwrap();
    }
}

impl ToString for BlobUrl {
    fn to_string(&self) -> String {
        self.0.clone()
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
        log::debug!("ImagePreview got update: {:?}", msg);

        match msg {
            Msg::NewState(state) => self.blob_url = state.blob_url,
        }

        true
    }

    fn view(&self) -> Html {
        let url = match &self.blob_url {
            Some(url) => url,
            None => return html! { <h2>{ "No image selected" }</h2> },
        };

        html! {
            <figure class="image">
                <img src=url />
            </figure>
        }
    }
}
