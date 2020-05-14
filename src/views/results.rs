use yew::prelude::*;
use yew::services::fetch::FetchTask;

use crate::components::{result, ImageHash, ImagePreview};
use crate::services::fuzzysearch::{FuzzySearchService, SourceFile};

pub struct Results {
    link: ComponentLink<Self>,
    fuzzysearch: FuzzySearchService,

    task: Option<FetchTask>,
    results: Option<anyhow::Result<Vec<SourceFile>>>,
}

#[derive(Debug)]
pub enum Msg {
    Results(anyhow::Result<Vec<SourceFile>>),
}

#[derive(Properties, Debug, Clone)]
pub struct Props {
    pub hash: i64,
}

impl Results {
    fn results(results: &[SourceFile]) -> Html {
        html! {
            <div>
                <p>{ format!("Found {} results", results.len()) }</p>

                <div>
                    <ImageHash redirect=true />
                    <ImagePreview />
                </div>

                <div>
                { results.iter().map(result).collect::<Html>() }
                </div>
            </div>
        }
    }

    fn load(&mut self, hash: i64) {
        self.results = None;

        let task = self
            .fuzzysearch
            .hashes(hash, self.link.callback(Msg::Results));
        self.task = Some(task);
    }
}

impl Component for Results {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fuzzysearch = FuzzySearchService::new();

        let mut results = Self {
            link,
            fuzzysearch,
            task: None,
            results: None,
        };

        results.load(props.hash);

        results
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.load(props.hash);

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Results(results) => self.results = Some(results),
        }

        true
    }

    fn view(&self) -> Html {
        match &self.results {
            None => html! {
                <h2>{ "Loading..." }</h2>
            },
            Some(Err(err)) => html! {
                <h2>{ format!("Error loading results: {}", err) }</h2>
            },
            Some(Ok(results)) => Self::results(results),
        }
    }
}
