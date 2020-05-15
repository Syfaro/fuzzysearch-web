use yew::prelude::*;
use yew::services::fetch::FetchTask;

use crate::agents::event_bus::*;
use crate::components::{result, ImageHash, ImagePreview};
use crate::services::fuzzysearch::{FuzzySearchService, HashResult, SourceFile};

const DISTANCE_THRESHOLD: u64 = 3;

struct ResultItems {
    count: usize,

    good: Html,
    good_count: usize,

    bad: Html,
    bad_count: usize,
}

pub struct Results {
    link: ComponentLink<Self>,
    fuzzysearch: FuzzySearchService,
    _producer: Box<dyn Bridge<EventBus>>,

    hash: i64,
    latest_hash: Option<i64>,

    task: Option<FetchTask>,
    items: Option<anyhow::Result<ResultItems>>,
    duration: Option<u128>,

    show_alts: bool,
}

#[derive(Debug)]
pub enum Msg {
    Results(HashResult),
    ToggleAlts,
    NewState(State),
}

#[derive(Properties, Debug, Clone)]
pub struct Props {
    pub hash: i64,
}

impl Results {
    fn results(&self, results: &ResultItems) -> Html {
        let mut items = vec![results.good.clone()];

        if results.bad_count > 0 && results.good_count > 0 {
            let text = if self.show_alts {
                "Hide less relevant results"
            } else {
                "Show less relevant results"
            };

            items.push(html! {
                <div class="box">
                    <button
                        class="button is-light is-warning is-fullwidth"
                        onclick=self.link.callback(|_| Msg::ToggleAlts)
                    >
                        { text }
                    </button>
                </div>
            });
        }

        if self.show_alts || results.good_count == 0 {
            items.push(results.bad.clone());
        }

        let items = items.into_iter().collect::<Html>();

        html! {
            <div>
                { items }
            </div>
        }
    }

    fn load(&mut self, hash: i64) {
        self.items = None;
        self.duration = None;
        self.hash = hash;
        self.show_alts = false;

        let task = self
            .fuzzysearch
            .hashes(hash, self.link.callback(Msg::Results));
        self.task = Some(task);
    }

    fn process_results(results: &[SourceFile]) -> ResultItems {
        // Create Vecs with capacity of some reasonable result assumptions.
        let mut good = Vec::with_capacity(3);
        let mut bad = Vec::with_capacity(results.len());

        for item in results {
            let rendered = result(item);

            if item.distance.unwrap_or(u64::max_value()) <= DISTANCE_THRESHOLD {
                good.push(rendered);
            } else {
                bad.push(rendered);
            }
        }

        ResultItems {
            count: results.len(),

            good_count: good.len(),
            good: good.into_iter().collect::<Html>(),

            bad_count: bad.len(),
            bad: bad.into_iter().collect::<Html>(),
        }
    }

    fn stats(&self) -> Html {
        let items = match &self.items {
            Some(Ok(items)) if items.count > 0 => items,
            _ => return html! {},
        };

        html! {
            <div class="box">
                <nav class="level">
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{ "Results" }</p>
                            <p class="title">{ items.count }</p>
                        </div>
                    </div>

                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{ "Duration (ms) "}</p>
                            <p class="title">{ self.duration.unwrap_or(0) }</p>
                        </div>
                    </div>
                </nav>
            </div>
        }
    }
}

impl Component for Results {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fuzzysearch = FuzzySearchService::new();

        let callback = link.callback(Msg::NewState);
        let _producer = EventBus::bridge(callback);

        let mut results = Self {
            link,
            fuzzysearch,
            _producer,
            task: None,
            items: None,
            show_alts: false,
            duration: None,
            hash: props.hash,
            latest_hash: None,
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
            Msg::Results(results) => {
                self.duration = Some(results.duration);
                self.items = Some(results.items.map(|results| Self::process_results(&results)))
            }
            Msg::NewState(state) => {
                self.latest_hash = state.latest_hash;
            }
            Msg::ToggleAlts => self.show_alts = !self.show_alts,
        }

        true
    }

    fn view(&self) -> Html {
        let items = match &self.items {
            None => html! {
                <h2>{ "Loading results..." }</h2>
            },
            Some(Err(err)) => html! {
                <h2>{ format!("Error loading results: {}", err) }</h2>
            },
            Some(Ok(results)) if results.count == 0 => html! {
                <h2>{ "No results found "}</h2>
            },
            Some(Ok(results)) => self.results(results),
        };

        let show_preview = if let Some(latest_hash) = self.latest_hash {
            latest_hash == self.hash
        } else {
            false
        };

        let preview = if show_preview {
            html! { <ImagePreview /> }
        } else {
            html! {}
        };

        html! {
            <section class="section">
                <div class="container">
                    <div class="columns">
                        <div class="column is-one-third">
                            <ImageHash redirect=true />

                            { self.stats() }

                            { preview }
                        </div>

                        <div class="column">
                            { items }
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
