use instant::Instant;
use serde::Deserialize;
use yew::{
    callback::Callback,
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Debug, Deserialize)]
pub struct SourceFile {
    pub id: i32,
    pub site_id: i64,
    pub url: String,
    pub filename: String,
    pub artists: Option<Vec<String>>,
    #[serde(flatten)]
    pub site_info: Option<SiteInfo>,
    pub hash: Option<i64>,
    pub distance: Option<u64>,
}

impl SourceFile {
    fn twitter_username(&self) -> &str {
        self.artists.as_ref().unwrap().iter().next().unwrap()
    }

    pub fn link(&self) -> String {
        match self.site_info.as_ref().unwrap() {
            SiteInfo::FurAffinity(_) => {
                format!("https://www.furaffinity.net/view/{}/", self.site_id)
            }
            SiteInfo::E621(_) => format!("https://e621.net/post/show/{}", self.site_id),
            SiteInfo::Twitter => format!(
                "https://twitter.com/{}/status/{}",
                self.twitter_username(),
                self.site_id
            ),
        }
    }

    pub fn pretty_link(&self) -> String {
        match self.site_info.as_ref().unwrap() {
            SiteInfo::FurAffinity(_) => format!("furaffinity.net/view/{}", self.site_id),
            SiteInfo::E621(_) => format!("e621.net/post/show/{}", self.site_id),
            SiteInfo::Twitter => format!(
                "twitter.com/{}/status/{}",
                self.twitter_username(),
                self.site_id
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "site", content = "site_info")]
pub enum SiteInfo {
    FurAffinity(FurAffinityFile),
    #[serde(rename = "e621")]
    E621(E621File),
    Twitter,
}

impl SiteInfo {
    pub fn name(&self) -> &'static str {
        match self {
            SiteInfo::FurAffinity(_) => "FurAffinity",
            SiteInfo::E621(_) => "e621",
            SiteInfo::Twitter => "Twitter",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FurAffinityFile {
    pub file_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct E621File {
    pub sources: Option<Vec<String>>,
}

#[derive(Default)]
pub struct FuzzySearchService {
    web: FetchService,
}

#[derive(Debug)]
pub struct HashResult {
    pub items: anyhow::Result<Vec<SourceFile>>,
    pub duration: u128,
}

impl FuzzySearchService {
    const API_ENDPOINT: &'static str = "https://api.fuzzysearch.net";
    const API_KEY: &'static str = "eluIOaOhIP1RXlgYetkcZCF8la7p3NoCPy8U0i8dKiT4xdIH";

    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn hashes(&mut self, hash: i64, callback: Callback<HashResult>) -> FetchTask {
        let url = format!("{}/hashes?hashes={}", Self::API_ENDPOINT, hash);
        let bytes = hash.to_be_bytes();

        let start = Instant::now();

        let handler = move |r: Response<Json<anyhow::Result<Vec<SourceFile>>>>| {
            let (meta, Json(data)) = r.into_parts();
            if !meta.status.is_success() {
                return callback.emit(HashResult {
                    items: Err(anyhow::anyhow!("bad request: {}", meta.status)),
                    duration: start.elapsed().as_millis(),
                });
            }

            let items = data.map(|files| {
                let mut files: Vec<_> = files
                    .into_iter()
                    .map(|file| {
                        let hash = match &file.hash {
                            Some(hash) => hash,
                            None => return file,
                        };

                        let hash_bytes = hash.to_be_bytes();

                        let distance = hamming::distance_fast(&bytes, &hash_bytes).ok();

                        SourceFile { distance, ..file }
                    })
                    .collect();

                files.sort_by(|a, b| {
                    a.distance
                        .unwrap_or(u64::max_value())
                        .partial_cmp(&b.distance.unwrap_or(u64::max_value()))
                        .unwrap()
                });

                files
            });

            let duration = start.elapsed().as_millis();

            callback.emit(HashResult { items, duration })
        };

        let request = Request::get(url.as_str())
            .header("x-api-key", Self::API_KEY)
            .body(Nothing)
            .unwrap();

        self.web.fetch(request, handler.into()).unwrap()
    }
}
