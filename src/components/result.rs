use yew::prelude::*;

use crate::services::fuzzysearch::SourceFile;

fn match_quality(distance: u64) -> &'static str {
    if distance == 0 {
        "perfect match"
    } else if distance < 4 {
        "good match"
    } else {
        "unlikely match"
    }
}

pub fn result(result: &SourceFile) -> Html {
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
                    <p class="is-size-7 has-text-grey">{ match_quality(distance) }</p>
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
