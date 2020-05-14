use std::rc::Rc;
use yew::prelude::*;
use yew::{agent::Dispatcher, services::reader::*};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

use crate::agents::{event_bus::*, ImageHashWorker};
use crate::app::AppRoute;
use crate::components::BlobUrl;

pub type HashCallback = Callback<anyhow::Result<i64>>;
pub type ImageCallback = Callback<()>;

pub struct ImageHash {
    link: ComponentLink<Self>,
    reader: ReaderService,
    hasher: Box<dyn Bridge<ImageHashWorker>>,
    event_bus: Dispatcher<EventBus>,

    task: Option<ReaderTask>,

    redirect: bool,
    onhash: Option<HashCallback>,
    onimage: Option<ImageCallback>,
}

#[derive(Debug)]
pub enum Msg {
    FileSelected(File),
    FileBytes(FileData),
    Hash(anyhow::Result<i64>),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub redirect: bool,
    #[prop_or_default]
    pub onhash: Option<HashCallback>,
    #[prop_or_default]
    pub onimage: Option<ImageCallback>,
}

impl Component for ImageHash {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::Hash);

        Self {
            link,
            reader: ReaderService::new(),
            hasher: ImageHashWorker::bridge(callback),
            event_bus: EventBus::dispatcher(),
            task: None,
            redirect: props.redirect,
            onhash: props.onhash,
            onimage: props.onimage,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onhash = props.onhash;
        self.onimage = props.onimage;

        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::trace!("ImageHash had update: {:?}", msg);

        match msg {
            Msg::Hash(hash) => {
                if self.redirect {
                    if let Ok(hash) = hash {
                        let route = Route::<()>::from(AppRoute::Results(hash));
                        RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(route));
                    }
                }

                if let Some(onhash) = &self.onhash {
                    onhash.emit(hash)
                }
            }
            Msg::FileBytes(bytes) => self.hasher.send(bytes.content),
            Msg::FileSelected(file) => {
                let blob_url = Rc::new(BlobUrl::new(&file));

                let callback = self.link.callback(Msg::FileBytes);
                let task = self.reader.read_file(file, callback).unwrap();
                self.task = Some(task);

                if let Some(onimage) = &self.onimage {
                    onimage.emit(())
                }

                self.event_bus.send(Request::SetState(State {
                    blob_url: Some(blob_url),
                }));
            }
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="file">
                <label class="file-label has-background-light">
                    <input class="file-input" type="file" accept="image/*" onchange=self.link.callback(change) />
                    <span class="file-cta">
                        <span class="file-icon">
                            <i class="fas fa-upload"></i>
                        </span>
                        <span class="file-label">
                            { "Browse" }
                        </span>
                    </span>
                </label>
            </div>
        }
    }
}

fn change(value: ChangeData) -> Msg {
    let files = match value {
        ChangeData::Files(files) => files,
        _ => return Msg::Hash(Err(anyhow::anyhow!("Invalid data was provided"))),
    };

    let files: Vec<_> = js_sys::try_iter(&files)
        .unwrap()
        .unwrap()
        .map(|f| File::from(f.unwrap()))
        .collect();

    if files.len() == 1 {
        Msg::FileSelected(files.into_iter().next().unwrap())
    } else {
        Msg::Hash(Err(anyhow::anyhow!("Incorrect file count")))
    }
}
