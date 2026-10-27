use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub exec: String,
    pub desktop_file: PathBuf,
}

pub fn discover_sessions() -> Vec<Session> {
    let directories = [
        Path::new("/usr/share/wayland-sessions"),
        Path::new("/usr/local/share/wayland-sessions"),
    ];

    let mut sessions = Vec::new();

    for directory in directories {
        let Ok(entries) = fs::read_dir(directory) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("desktop") {
                continue;
            }

            if let Some(session) = parse_desktop_file(&path) {
                sessions.push(session);
            }
        }
    }

    sessions.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    sessions.dedup_by(|a, b| a.desktop_file == b.desktop_file);

    sessions
}

fn parse_desktop_file(path: &Path) -> Option<Session> {
    let contents = fs::read_to_string(path).ok()?;

    let mut in_desktop_entry = false;
    let mut name = None;
    let mut exec = None;
    let mut hidden = false;
    let mut no_display = false;
    let mut try_exec = None;

    for raw_line in contents.lines() {
        let line = raw_line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }

        if !in_desktop_entry {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let value = value.trim();

        match key.trim() {
            // Lokalisierte Name[xx]-Einträge werden vorerst ignoriert.
            "Name" => name = Some(value.to_string()),
            "Exec" => exec = Some(value.to_string()),
            "Hidden" => hidden = value.eq_ignore_ascii_case("true"),
            "NoDisplay" => no_display = value.eq_ignore_ascii_case("true"),
            "TryExec" => try_exec = Some(value.to_string()),
            _ => {}
        }
    }

    if hidden || no_display {
        return None;
    }

    let name = name.filter(|s| !s.trim().is_empty())?;
    let exec = exec.filter(|s| !s.trim().is_empty())?;

    // Falls TryExec gesetzt ist, prüfen wir, ob das Programm
    // vorhanden ist. Das ist eine einfache Prüfung für den Prototyp.
    if let Some(program) = try_exec {
        if !program_exists(&program) {
            return None;
        }
    }

    Some(Session {
        name,
        exec,
        desktop_file: path.to_path_buf(),
    })
}

fn program_exists(program: &str) -> bool {
    let path = Path::new(program);

    if path.components().count() > 1 {
        return path.is_file();
    }

    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .any(|dir| dir.join(program).is_file())
}
