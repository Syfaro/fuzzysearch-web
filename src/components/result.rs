use yew::prelude::*;

use crate::services::fuzzysearch::{E621File, SiteInfo, SourceFile};

fn match_quality(distance: u64) -> &'static str {
    if distance == 0 {
        "perfect match"
    } else if distance < 4 {
        "good match"
    } else {
        "unlikely match"
    }
}

fn e621_sources(sources: &[String]) -> Html {
    if sources.is_empty() {
        return html! {};
    }

    let items = sources.iter().map(|source| {
        let source = source.to_owned();
        let pretty = source
            .replace("https://", "")
            .replace("http://", "")
            .replace("www.", "");

        html! {
            <li>
                <a target="_blank" href=source>{ pretty }</a>
            </li>
        }
    });

    html! {
        <div style="margin-top: 1em;">
            <p>{ "Linked sources" }</p>

            <ul>
                { items.collect::<Html>() }
            </ul>
        </div>
    }
}

pub fn result(result: &SourceFile) -> Html {
    let distance = result.distance.unwrap_or(u64::max_value());
    let site_info = result.site_info.as_ref().unwrap();

    let artists = match result.artists.as_ref() {
        Some(artists) => artists.join(", "),
        None => "Unknown".to_string(),
    };

    let sources = match &result.site_info {
        Some(SiteInfo::E621(E621File {
            sources: Some(sources),
            ..
        })) => e621_sources(&sources),
        _ => html! {},
    };

    html! {
        <div class="box">
            <div class="columns">
                <div class="column is-one-fifth has-text-centered">
                    <h2 class="is-size-1">{ distance }</h2>
                    <p class="is-size-7 has-text-grey">{ match_quality(distance) }</p>
                </div>
                <div class="column">
                    <p>
                        <strong>{ site_info.name() }</strong><br/>
                        { format!("Posted by {}", artists) }<br/>
                        <a target="_blank" href=result.link()>{ result.pretty_link() }</a>

                        { sources }
                    </p>
                </div>
            </div>
        </div>
    }
}
