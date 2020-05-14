use yew::prelude::*;
use yew::services::fetch::FetchTask;

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
    fn match_quality(distance: u64) -> &'static str {
        if distance == 0 {
            "perfect match"
        } else if distance < 4 {
            "good match"
        } else {
            "unlikely match"
        }
    }

    fn result(result: &SourceFile) -> Html {
        let distance = result.distance.unwrap_or(u64::max_value());
        let site_info = result.site_info.as_ref().unwrap();

        let artists = match result.artists.as_ref() {
            Some(artists) => artists.join(", "),
            None => "Unknown".to_string(),
        };

        html! {
            <div class="box">
                <div class="columns">
                    <div class="column is-one-fifth has-text-centered">
                        <h2 class="is-size-1">{ distance }</h2>
                        <p class="is-size-7 has-text-grey">{ Self::match_quality(distance) }</p>
                    </div>
                    <div class="column">
                        <p>
                            <strong>{ site_info.name() }</strong><br/>
                            { format!("Posted by {}", artists) }<br/>
                            <a target="_blank" href=result.link()>{ result.pretty_link() }</a>
                        </p>
                    </div>
                </div>
            </div>
        }
    }

    fn results(results: &[SourceFile]) -> Html {
        html! {
            <div>
                <p>{ format!("Found {} results", results.len()) }</p>

                <div>
                { results.iter().map(Self::result).collect::<Html>() }
                </div>
            </div>
        }
    }
}

impl Component for Results {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut fuzzysearch = FuzzySearchService::new();

        let task = fuzzysearch.hashes(props.hash, link.callback(Msg::Results));

        Self {
            link,
            fuzzysearch,
            task: Some(task),

            results: None,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.results = None;

        let task = self
            .fuzzysearch
            .hashes(props.hash, self.link.callback(Msg::Results));
        self.task = Some(task);

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
