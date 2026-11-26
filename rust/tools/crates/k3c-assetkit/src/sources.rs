//! Quellen der Web-Recherche (B-298/AC-04): freesound, Wikimedia Commons und OpenGameArt, parallel abgefragt.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use std::time::Duration;

use regex::Regex;
use serde::Deserialize;
use tokio::task::JoinSet;

use crate::error::{Error, Result};
use crate::gojson::null_default;
use crate::net::{encode_query, read_limited};
use crate::register::{KIND_GRAPHIC, KIND_SOUND};
use crate::research::Hit;

/// Begrenzt jede Quelle; eine langsame Quelle hält die anderen nicht auf.
const SOURCE_TIMEOUT: Duration = Duration::from_secs(15);

const USER_AGENT: &str = "k3c-workbench/0.1 (Ressourcen-Manager, B-298)";

/// Größte gelesene Antwort einer Quelle.
const MAX_ANSWER: usize = 4 << 20;

/// Fragt die Quellen der Web-Recherche ab. Basis-Adressen sind für Tests austauschbar.
#[derive(Clone)]
pub struct Searcher {
    pub client: reqwest::Client,
    /// API, braucht Token.
    pub freesound: String,
    /// MediaWiki-API von Wikimedia Commons.
    pub commons: String,
    /// OpenGameArt, Suchseite als HTML (ohne API).
    pub oga: String,
    /// freesound; nie im Repo.
    pub token: String,
}

/// Die echten Quellen und der freesound-Token aus `K3C_FREESOUND_TOKEN` oder `.work/freesound-token`.
pub fn new_searcher(root: &Path) -> Result<Searcher> {
    let mut token = std::env::var("K3C_FREESOUND_TOKEN")
        .unwrap_or_default()
        .trim()
        .to_string();
    if token.is_empty()
        && let Ok(text) = fs::read_to_string(root.join(".work").join("freesound-token"))
    {
        token = text.trim().to_string();
    }
    let client = reqwest::Client::builder()
        .timeout(SOURCE_TIMEOUT)
        .build()
        .map_err(|e| Error::Msg(e.to_string()))?;
    Ok(Searcher {
        client,
        token,
        freesound: "https://freesound.org".to_string(),
        commons: "https://commons.wikimedia.org".to_string(),
        oga: "https://opengameart.org".to_string(),
    })
}

#[derive(Clone, Copy)]
enum Source {
    Freesound,
    Commons,
    OpenGameArt,
}

impl Source {
    fn name(self) -> &'static str {
        match self {
            Source::Freesound => "freesound",
            Source::Commons => "Wikimedia Commons",
            Source::OpenGameArt => "OpenGameArt",
        }
    }
}

impl Searcher {
    /// Fragt alle passenden Quellen parallel; Fehler einer Quelle stehen als Hinweis in der zweiten Liste, die anderen
    /// liefern weiter.
    pub async fn search(&self, query: &str, kind: &str) -> (Vec<Hit>, Vec<String>) {
        let mut sources = vec![Source::Commons, Source::OpenGameArt];
        if kind == KIND_SOUND {
            sources.insert(0, Source::Freesound);
        }
        let mut tasks = JoinSet::new();
        for (i, source) in sources.iter().copied().enumerate() {
            let (searcher, query, kind) = (self.clone(), query.to_string(), kind.to_string());
            tasks.spawn(async move {
                let run = searcher.run(source, &query, &kind);
                let result = tokio::time::timeout(SOURCE_TIMEOUT, run)
                    .await
                    .unwrap_or_else(|_| Err(Error::Msg("context deadline exceeded".to_string())));
                (i, result)
            });
        }
        let mut results: Vec<Option<Result<Vec<Hit>>>> = sources.iter().map(|_| None).collect();
        while let Some(joined) = tasks.join_next().await {
            if let Ok((i, result)) = joined {
                results[i] = Some(result);
            }
        }
        let (mut hits, mut notes) = (Vec::new(), Vec::new());
        for (source, result) in sources.iter().zip(results) {
            let result = result.unwrap_or_else(|| Err(Error::Msg("abgebrochen".to_string())));
            match result {
                Ok(found) => hits.extend(found),
                Err(e) => notes.push(format!("{}: {e}", source.name())),
            }
        }
        (hits, notes)
    }

    async fn run(&self, source: Source, query: &str, kind: &str) -> Result<Vec<Hit>> {
        match source {
            Source::Freesound => self.freesound_hits(query).await,
            Source::Commons => self.commons_hits(query, kind).await,
            Source::OpenGameArt => self.open_game_art_hits(query, kind).await,
        }
    }

    async fn get(&self, url: &str, header: &[(&str, &str)]) -> Result<Vec<u8>> {
        let mut request = self.client.get(url).header(reqwest::header::USER_AGENT, USER_AGENT);
        for (name, value) in header {
            request = request.header(*name, *value);
        }
        let response = request.send().await.map_err(network_error)?;
        let status = response.status().as_u16();
        let body = read_limited(response, MAX_ANSWER).await.map_err(network_error)?;
        if status != 200 {
            return Err(Error::Msg(format!("HTTP {status}")));
        }
        Ok(body)
    }

    /// freesound: Such-API mit Token; Lizenz kommt als URL, Hörprobe als MP3-Vorschau. Download braucht OAuth: nur Link.
    async fn freesound_hits(&self, query: &str) -> Result<Vec<Hit>> {
        if self.token.is_empty() {
            return Err(Error::Msg(
                "token fehlt (K3C_FREESOUND_TOKEN oder .work/freesound-token)".to_string(),
            ));
        }
        // Token im Header, nie in der Adresse: so steht er in keiner Fehlermeldung und keinem Log.
        let q = encode_query(&[
            ("query", query),
            ("fields", "id,name,url,username,license,previews"),
            ("page_size", "15"),
        ]);
        let url = format!("{}/apiv2/search/text/?{q}", self.freesound);
        let auth = format!("Token {}", self.token);
        let body = self.get(&url, &[("Authorization", &auth)]).await?;
        let answer: Option<FreesoundAnswer> = decode(&body)?;
        let results = answer.unwrap_or_default().results;
        Ok(results
            .into_iter()
            .map(|mut r| Hit {
                title: r.name,
                link: r.url,
                source: "freesound".to_string(),
                artist: r.username,
                license: license_from_url(&r.license),
                preview: r.previews.remove("preview-hq-mp3").unwrap_or_default(),
                ..Hit::default()
            })
            .collect())
    }

    /// Commons: MediaWiki-API mit Lizenz und Urheber aus den Metadaten; die Originaldatei ist direkt ladbar.
    async fn commons_hits(&self, query: &str, kind: &str) -> Result<Vec<Hit>> {
        let filetype = if kind == KIND_SOUND { "audio" } else { "bitmap" };
        let search = format!("{query} filetype:{filetype}");
        let q = encode_query(&[
            ("action", "query"),
            ("format", "json"),
            ("generator", "search"),
            ("gsrnamespace", "6"),
            ("gsrlimit", "15"),
            ("gsrsearch", &search),
            ("prop", "imageinfo"),
            ("iiprop", "url|extmetadata"),
            ("iiurlwidth", "128"),
            ("iiextmetadatafilter", "LicenseShortName|Artist|LicenseUrl"),
        ]);
        let body = self.get(&format!("{}/w/api.php?{q}", self.commons), &[]).await?;
        let answer: Option<CommonsAnswer> = decode(&body)?;
        let pages = answer.unwrap_or_default().query.pages;
        let mut hits: Vec<Hit> = pages.into_values().filter_map(commons_hit).collect();
        hits.sort_by(|a, b| a.title.cmp(&b.title)); // Seiten kommen als Map
        Ok(hits)
    }

    async fn open_game_art_hits(&self, query: &str, kind: &str) -> Result<Vec<Hit>> {
        let mut pairs = vec![("keys", query)];
        pairs.extend(oga_types(kind).iter().map(|t| ("field_art_type_tid[]", *t)));
        let url = format!("{}/art-search-advanced?{}", self.oga, encode_query(&pairs));
        let body = self.get(&url, &[]).await?;
        Ok(String::from_utf8_lossy(&body)
            .split('\n')
            .filter_map(|line| self.oga_hit(line))
            .collect())
    }

    /// OpenGameArt-Suchseite: je Zeile ein Treffer mit Titel und Link, Vorschau als Bild oder MP3; Lizenz nur auf der
    /// Einzelseite.
    fn oga_hit(&self, line: &str) -> Option<Hit> {
        let title = OGA_TITLE.captures(line)?;
        let preview = OGA_AUDIO.captures(line).or_else(|| OGA_PICTURE.captures(line));
        Some(Hit {
            title: html_escape::decode_html_entities(&title[2]).trim().to_string(),
            link: format!("{}{}", self.oga, &title[1]),
            source: "OpenGameArt".to_string(),
            preview: preview.map(|p| p[1].to_string()).unwrap_or_default(),
            ..Hit::default()
        })
    }
}

/// Netzfehler ohne Adresse: so gelangt nichts aus der Anfrage in Hinweise und Logs.
fn network_error(e: reqwest::Error) -> Error {
    let e = e.without_url();
    let mut text = e.to_string();
    let mut cause = std::error::Error::source(&e);
    while let Some(c) = cause {
        text.push_str(&format!(": {c}"));
        cause = c.source();
    }
    Error::Msg(text)
}

fn decode<'a, T: Deserialize<'a>>(body: &'a [u8]) -> Result<T> {
    serde_json::from_slice(body).map_err(|e| Error::Msg(format!("antwort unlesbar: {e}")))
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct FreesoundAnswer {
    #[serde(deserialize_with = "null_default")]
    results: Vec<FreesoundResult>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct FreesoundResult {
    #[serde(deserialize_with = "null_default")]
    name: String,
    #[serde(deserialize_with = "null_default")]
    url: String,
    #[serde(deserialize_with = "null_default")]
    username: String,
    #[serde(deserialize_with = "null_default")]
    license: String,
    #[serde(deserialize_with = "null_default")]
    previews: BTreeMap<String, String>,
}

/// Macht aus einer Creative-Commons-Adresse den Kurznamen (CC0 1.0, CC BY-NC 3.0 …).
fn license_from_url(u: &str) -> String {
    if u.contains("publicdomain/zero") {
        return "CC0 1.0".to_string();
    }
    let Some((_, rest)) = u.split_once("/licenses/") else {
        return u.to_string();
    };
    let mut parts = rest.trim_matches('/').split('/');
    let mut name = format!("CC {}", parts.next().unwrap_or_default().to_uppercase());
    if let Some(version) = parts.next() {
        name.push(' ');
        name.push_str(version);
    }
    name
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct CommonsAnswer {
    #[serde(deserialize_with = "null_default")]
    query: CommonsQuery,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct CommonsQuery {
    #[serde(deserialize_with = "null_default")]
    pages: BTreeMap<String, CommonsPage>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct CommonsPage {
    #[serde(deserialize_with = "null_default")]
    title: String,
    #[serde(deserialize_with = "null_default")]
    imageinfo: Vec<ImageInfo>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct ImageInfo {
    #[serde(deserialize_with = "null_default")]
    thumburl: String,
    #[serde(deserialize_with = "null_default")]
    url: String,
    #[serde(deserialize_with = "null_default")]
    descriptionurl: String,
    #[serde(deserialize_with = "null_default")]
    extmetadata: BTreeMap<String, MetaValue>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MetaValue {
    #[serde(deserialize_with = "null_default")]
    value: String,
}

fn commons_hit(page: CommonsPage) -> Option<Hit> {
    let info = page.imageinfo.into_iter().next()?;
    let meta = |key: &str| info.extmetadata.get(key).map_or("", |m| m.value.as_str());
    Some(Hit {
        title: page.title.strip_prefix("File:").unwrap_or(&page.title).to_string(),
        source: "Wikimedia Commons".to_string(),
        artist: plain_text(meta("Artist")),
        license: meta("LicenseShortName").to_string(),
        link: info.descriptionurl.clone(),
        preview: info.thumburl.clone(),
        download: info.url.clone(),
        ..Hit::default()
    })
}

// Die Muster sind fest; ein Tippfehler fiele in jedem Quellen-Test auf.
#[allow(clippy::expect_used)]
fn pattern(source: &str) -> Regex {
    Regex::new(source).expect("festes Muster")
}

static TAGS: LazyLock<Regex> = LazyLock::new(|| pattern(r"<[^>]*>"));
static OGA_TITLE: LazyLock<Regex> =
    LazyLock::new(|| pattern(r#"class="art-preview-title"><a href="(/content/[^"]+)">([^<]+)</a>"#));
static OGA_AUDIO: LazyLock<Regex> = LazyLock::new(|| pattern(r"data-mp3-url='([^']+)'"));
static OGA_PICTURE: LazyLock<Regex> = LazyLock::new(|| pattern(r"<img src='([^']+)'"));

fn plain_text(s: &str) -> String {
    let bare = TAGS.replace_all(s, "");
    html_escape::decode_html_entities(&bare).trim().to_string()
}

/// Kunst-Arten der Suche: 9 = 2D Art, 13 = Sound Effect, 12 = Music.
fn oga_types(kind: &str) -> &'static [&'static str] {
    match kind {
        KIND_GRAPHIC => &["9"],
        KIND_SOUND => &["13", "12"],
        _ => &[],
    }
}

#[cfg(test)]
#[path = "sources_tests.rs"]
mod tests;
