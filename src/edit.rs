//! `capsule.toml` editing for `squeeze add` / `squeeze remove`, through
//! `toml_edit` so comments, ordering and formatting elsewhere in the file are
//! kept. Only the `[[dependencies]]` row being added, replaced or removed
//! changes.

use std::path::{Component, Path, PathBuf};

use anyhow::{Context, bail};
use crush_pkg::manifest::Dependency;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

const SOURCE_KEYS: [&str; 6] = ["version", "path", "git", "tag", "rev", "branch"];

fn read(path: &Path) -> anyhow::Result<DocumentMut> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    text.parse::<DocumentMut>()
        .with_context(|| format!("parsing {}", path.display()))
}

fn deps_mut<'a>(
    doc: &'a mut DocumentMut,
    path: &Path,
) -> anyhow::Result<Option<&'a mut ArrayOfTables>> {
    match doc.get_mut("dependencies") {
        None => Ok(None),
        Some(Item::ArrayOfTables(aot)) => Ok(Some(aot)),
        Some(_) => bail!(
            "{}: `dependencies` is not written as `[[dependencies]]` tables; \
             squeeze only edits that form",
            path.display()
        ),
    }
}

fn row_name(t: &Table) -> Option<&str> {
    t.get("name").and_then(Item::as_str)
}

/// Add `dep` as a `[[dependencies]]` row, or replace the source of the row
/// with its name (keeping that row's position and comments). `true` when a
/// row was replaced.
pub fn upsert_dependency(manifest: &Path, dep: &Dependency) -> anyhow::Result<bool> {
    let mut doc = read(manifest)?;
    if doc.get("dependencies").is_none() {
        doc.insert("dependencies", Item::ArrayOfTables(ArrayOfTables::new()));
    }
    let aot = deps_mut(&mut doc, manifest)?.expect("inserted above");
    let existing = aot
        .iter_mut()
        .find(|t| row_name(t) == Some(dep.name.as_str()));
    let replaced = existing.is_some();
    let row = match existing {
        Some(t) => t,
        None => {
            let mut t = Table::new();
            t.insert("name", value(dep.name.as_str()));
            aot.push(t);
            aot.iter_mut().last().expect("just pushed")
        }
    };
    for k in SOURCE_KEYS {
        row.remove(k);
    }
    let fields = [
        ("path", &dep.path),
        ("git", &dep.git),
        ("tag", &dep.tag),
        ("rev", &dep.rev),
        ("branch", &dep.branch),
    ];
    for (k, v) in fields {
        if let Some(v) = v {
            row.insert(k, value(v.as_str()));
        }
    }
    std::fs::write(manifest, doc.to_string())
        .with_context(|| format!("writing {}", manifest.display()))?;
    Ok(replaced)
}

/// Remove the `[[dependencies]]` row named `name`; `false` when there is none.
pub fn remove_dependency(manifest: &Path, name: &str) -> anyhow::Result<bool> {
    let mut doc = read(manifest)?;
    let Some(aot) = deps_mut(&mut doc, manifest)? else {
        return Ok(false);
    };
    let Some(idx) = aot.iter().position(|t| row_name(t) == Some(name)) else {
        return Ok(false);
    };
    aot.remove(idx);
    if aot.is_empty() {
        doc.remove("dependencies");
    }
    std::fs::write(manifest, doc.to_string())
        .with_context(|| format!("writing {}", manifest.display()))?;
    Ok(true)
}

/// How to write `--path DIR` in the manifest: relative to the package root
/// (`../lib`), the way hand-written manifests do, so the package can move
/// with its neighbours. `DIR` is taken relative to the current directory.
pub fn relative_path(root: &Path, dir: &Path) -> String {
    let abs = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|c| c.join(dir))
            .unwrap_or_else(|_| dir.to_path_buf())
    };
    let (Ok(target), Ok(base)) = (std::fs::canonicalize(&abs), std::fs::canonicalize(root)) else {
        return abs.display().to_string();
    };
    let t: Vec<Component> = target.components().collect();
    let b: Vec<Component> = base.components().collect();
    let common = t.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let mut out = PathBuf::new();
    for _ in common..b.len() {
        out.push("..");
    }
    for c in &t[common..] {
        out.push(c.as_os_str());
    }
    if out.as_os_str().is_empty() {
        ".".to_string()
    } else {
        out.display().to_string()
    }
}
