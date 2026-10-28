use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Frontmatter {
    title: String,
    #[serde(default)]
    description: String,
    posted: toml::Value,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default = "default_author")]
    author: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    problem: Option<String>,
    #[serde(default)]
    order: i32,
    #[serde(default)]
    draft: bool,
}

fn default_author() -> String {
    "Big Mo".to_string()
}

#[derive(Serialize, Clone, Debug)]
pub struct TagRef {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TagCount {
    pub name: String,
    pub slug: String,
    pub count: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub posted: String,
    pub posted_pretty: String,
    pub tags: Vec<TagRef>,
    pub author: String,
    pub status: Option<String>,
    pub sims: Vec<String>,
    pub has_math: bool,
    pub has_code: bool,
    pub draft: bool,
    pub order: i32,
    #[serde(skip)]
    pub date: NaiveDate,
    #[serde(skip)]
    pub content: String,
    #[serde(skip)]
    pub problem: Option<String>,
}

#[derive(Default)]
struct RenderInfo {
    sims: Vec<String>,
    has_math: bool,
    has_code: bool,
}

pub fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in s.trim().chars() {
        if c.is_alphanumeric() {
            out.extend(c.to_lowercase());
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    out.trim_end_matches('-').to_string()
}

pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn pretty_date(d: NaiveDate) -> String {
    d.format("%b %-d, %Y").to_string()
}

fn parse_date(v: &toml::Value) -> Result<NaiveDate, String> {
    match v {
        toml::Value::String(s) => NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
            .map_err(|e| format!("invalid date '{s}': {e}")),
        toml::Value::Datetime(dt) => {
            let d = dt.date.ok_or("datetime has no date part")?;
            NaiveDate::from_ymd_opt(d.year as i32, d.month as u32, d.day as u32)
                .ok_or_else(|| "date out of range".to_string())
        }
        _ => Err("'posted' must be a date like 2026-10-25".to_string()),
    }
}

fn split_frontmatter(src: &str) -> Result<(&str, &str), String> {
    let src = src.trim_start_matches('\u{feff}').trim_start();
    let rest = src
        .strip_prefix("+++")
        .ok_or("post must start with a +++ frontmatter block")?;
    let end = rest
        .find("\n+++")
        .ok_or("frontmatter block is never closed with +++")?;
    let body = &rest[end + 4..];
    let body = body.split_once('\n').map(|(_, b)| b).unwrap_or("");
    Ok((&rest[..end], body))
}

fn valid_sim_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn sim_html(body: &str, info: &mut RenderInfo) -> String {
    let trimmed = body.trim();
    let mut table = if trimmed.contains('=') {
        match toml::from_str::<toml::Table>(trimmed) {
            Ok(t) => t,
            Err(e) => {
                return format!(
                    "<div class=\"sim-error\">Invalid simulation config: {}</div>",
                    escape(&e.to_string())
                );
            }
        }
    } else {
        let mut t = toml::Table::new();
        t.insert("name".into(), toml::Value::String(trimmed.to_string()));
        t
    };

    let name = match table.remove("name") {
        Some(toml::Value::String(s)) if valid_sim_name(&s) => s,
        _ => {
            return "<div class=\"sim-error\">Simulation block needs a valid name</div>".to_string();
        }
    };
    let caption = match table.remove("caption") {
        Some(toml::Value::String(s)) => s,
        _ => String::new(),
    };
    let height = match table.remove("height") {
        Some(toml::Value::Integer(h)) => h.clamp(120, 1200),
        _ => 380,
    };
    let config = serde_json::to_string(&table).unwrap_or_else(|_| "{}".to_string());

    if !info.sims.contains(&name) {
        info.sims.push(name.clone());
    }

    let caption_html = if caption.is_empty() {
        String::new()
    } else {
        format!("<figcaption>{}</figcaption>", escape(&caption))
    };

    format!(
        "<figure class=\"sim\" data-sim=\"{}\" data-config=\"{}\" style=\"--sim-h:{}px\"><div class=\"sim-head\"><span class=\"sim-dot\"></span><span class=\"sim-name\">{}</span></div><div class=\"sim-stage\"><div class=\"sim-loading\">loading simulation…</div></div>{}</figure>",
        escape(&name),
        escape(&config),
        height,
        escape(&name.replace(['_', '-'], " ")),
        caption_html
    )
}

fn render_markdown(src: &str, info: &mut RenderInfo) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    enum Capture {
        Sim(String),
        Math(String),
    }

    let mut events = Vec::new();
    let mut capture: Option<Capture> = None;

    for ev in Parser::new_ext(src, opts) {
        match ev {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) if lang.as_ref() == "sim" => {
                capture = Some(Capture::Sim(String::new()));
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang)))
                if matches!(lang.as_ref(), "math" | "latex" | "tex") =>
            {
                capture = Some(Capture::Math(String::new()));
            }
            Event::Text(t) if capture.is_some() => match capture.as_mut() {
                Some(Capture::Sim(buf)) | Some(Capture::Math(buf)) => buf.push_str(&t),
                None => {}
            },
            Event::End(TagEnd::CodeBlock) if capture.is_some() => match capture.take() {
                Some(Capture::Sim(buf)) => events.push(Event::Html(sim_html(&buf, info).into())),
                Some(Capture::Math(buf)) => {
                    info.has_math = true;
                    events.push(Event::Html(
                        format!(
                            "<div class=\"math math-display\">{}</div>",
                            escape(buf.trim())
                        )
                        .into(),
                    ));
                }
                None => {}
            },
            Event::Start(Tag::CodeBlock(_)) => {
                info.has_code = true;
                events.push(ev);
            }
            Event::InlineMath(_) | Event::DisplayMath(_) => {
                info.has_math = true;
                events.push(ev);
            }
            other => events.push(other),
        }
    }

    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

fn parse_post(slug: &str, src: &str) -> Result<Post, String> {
    let src = src.replace("\r\n", "\n");
    let (fm_src, body) = split_frontmatter(&src)?;
    let fm: Frontmatter = toml::from_str(fm_src).map_err(|e| e.to_string())?;
    let date = parse_date(&fm.posted)?;

    let mut info = RenderInfo::default();
    let content = render_markdown(body, &mut info);
    let problem = fm.problem.as_deref().map(|p| render_markdown(p, &mut info));
    
    Ok(Post {
        slug: slug.to_string(),
        title: fm.title,
        description: fm.description,
        posted: date.format("%Y-%m-%d").to_string(),
        posted_pretty: pretty_date(date),
        tags: fm
            .tags
            .iter()
            .map(|t| TagRef {
                name: t.trim().to_string(),
                slug: slugify(t),
            })
            .filter(|t| !t.slug.is_empty())
            .collect(),
        author: fm.author,
        status: fm.status.map(|s| s.to_lowercase()),
        sims: info.sims,
        has_math: info.has_math,
        has_code: info.has_code,
        draft: fm.draft,
        order: fm.order,
        date,
        content,
        problem,
    })
}

pub fn load_posts(root: &Path, include_drafts: bool) -> Vec<Post> {
    let dir = root.join("content").join("posts");
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(e) => {
            log::error!("cannot read {}: {e}", dir.display());
            return Vec::new();
        }
    };

    let mut posts = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let slug = slugify(stem);
        let src = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                log::error!("cannot read {}: {e}", path.display());
                continue;
            }
        };
        match parse_post(&slug, &src) {
            Ok(post) => {
                if post.draft && !include_drafts {
                    continue;
                }
                for sim in &post.sims {
                    let file = root.join("static").join("sims").join(format!("{sim}.js"));
                    if !file.exists() {
                        log::warn!("post '{}' uses missing simulation '{}'", slug, sim);
                    }
                }
                posts.push(post);
            }
            Err(e) => log::error!("skipping {}: {e}", path.display()),
        }
    }

    posts.sort_by_key(|p| (Reverse(p.date), Reverse(p.order), p.title.clone()));
    posts
}

pub fn tag_counts(posts: &[Post]) -> Vec<TagCount> {
    let mut map: BTreeMap<String, TagCount> = BTreeMap::new();
    for post in posts {
        for tag in &post.tags {
            map.entry(tag.slug.clone())
                .or_insert_with(|| TagCount {
                    name: tag.name.clone(),
                    slug: tag.slug.clone(),
                    count: 0,
                })
                .count += 1;
        }
    }
    let mut tags: Vec<TagCount> = map.into_values().collect();
    tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));
    tags
}
