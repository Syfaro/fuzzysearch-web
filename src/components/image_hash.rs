use yew::prelude::*;
use yew::services::reader::*;

use crate::agents::ImageHashWorker;

pub type HashCallback = Callback<anyhow::Result<i64>>;
pub type ImageCallback = Callback<()>;

pub struct ImageHash {
    link: ComponentLink<Self>,
    reader: ReaderService,
    hasher: Box<dyn Bridge<ImageHashWorker>>,

    task: Option<ReaderTask>,

    onhash: HashCallback,
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
    pub onhash: HashCallback,
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
            task: None,
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
            Msg::Hash(hash) => self.onhash.emit(hash),
            Msg::FileBytes(bytes) => self.hasher.send(bytes.content),
            Msg::FileSelected(file) => {
                let callback = self.link.callback(Msg::FileBytes);
                let task = self.reader.read_file(file, callback).unwrap();
                self.task = Some(task);
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
