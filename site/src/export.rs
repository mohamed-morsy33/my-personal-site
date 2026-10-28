use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use chrono::Local;

use crate::posts::pretty_date;
use crate::{Site, pages};

fn write_page(out: &Path, route: &str, html: String) -> Result<(), String> {
    let dir = out.join(route.trim_matches('/'));
    fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let file = dir.join("index.html");
    fs::write(&file, html).map_err(|e| format!("{}: {e}", file.display()))
}

fn copy_dir(from: &Path, to: &Path) -> Result<usize, String> {
    fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    let mut count = 0;
    for entry in fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            count += copy_dir(&src, &dst)?;
        } else {
            fs::copy(&src, &dst).map_err(|e| format!("{}: {e}", src.display()))?;
            count += 1;
        }
    }
    Ok(count)
}

pub fn export(site: &Site, out: &Path) -> Result<(), String> {
    if out.exists() {
        fs::remove_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    }
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;

    let posts = site.posts();
    let today = Local::now().date_naive();
    let mut pages_written = 0;

    write_page(out, "", site.render_string("home.html", &pages::home(&posts, today))?)?;
    pages_written += 1;

    for (idx, post) in posts.iter().enumerate() {
        let html = site.render_string("post.html", &pages::post(&posts, idx))?;
        write_page(out, &format!("posts/{}", post.slug), html)?;
        pages_written += 1;
    }

    let tag_slugs: BTreeSet<&str> = posts
        .iter()
        .flat_map(|p| p.tags.iter().map(|t| t.slug.as_str()))
        .collect();
    for slug in tag_slugs {
        if let Some((heading, matching)) = pages::tag_posts(&posts, slug) {
            let html = site.render_string("list.html", &pages::list("tagged", &heading, &matching))?;
            write_page(out, &format!("tags/{slug}"), html)?;
            pages_written += 1;
        }
    }

    let dates: BTreeSet<_> = posts.iter().map(|p| p.date).collect();
    for date in dates {
        let matching = pages::day_posts(&posts, date);
        let html = site.render_string(
            "list.html",
            &pages::list("posted on", &pretty_date(date), &matching),
        )?;
        write_page(out, &format!("on/{}", date.format("%Y-%m-%d")), html)?;
        pages_written += 1;
    }

    let not_found = site.render_string("404.html", &tera::Context::new())?;
    fs::write(out.join("404.html"), not_found).map_err(|e| e.to_string())?;

    let graph = crate::graph::build(&posts, today);
    let api = out.join("api");
    fs::create_dir_all(&api).map_err(|e| e.to_string())?;
    fs::write(
        api.join("graph.json"),
        serde_json::to_string(&graph).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    let assets = copy_dir(&site.root.join("static"), &out.join("static"))?;
    fs::write(out.join(".nojekyll"), "").map_err(|e| e.to_string())?;

    log::info!(
        "exported {pages_written} pages and {assets} static files to {}",
        out.display()
    );
    Ok(())
}
