use crate::collection::prow::download_artifacts;
use crate::system::create_file_index;
use std::io::BufRead;
use std::path::Path;

pub struct Nouns {
    pub nouns: Vec<String>,
}

impl Nouns {
    pub fn new() -> Self {
        Nouns {
            nouns: vec![
                "error".to_string(),
                "failure".to_string(),
                "mistake".to_string(),
                "problem".to_string(),
                "timeout".to_string(),
                "warning".to_string(),
            ],
        }
    }

    pub fn add(&mut self, noun: String) {
        self.nouns.push(noun);
    }

    pub fn list(&self) -> Vec<String> {
        self.nouns.clone()
    }
}

pub struct Verbs {
    pub verbs: Vec<String>,
}

impl Verbs {
    pub fn new() -> Self {
        Verbs {
            verbs: vec![
                "abort".to_string(),
                "add".to_string(),
                "analyze".to_string(),
                "build".to_string(),
                "cancel".to_string(),
                "check".to_string(),
                "collect".to_string(),
                "compile".to_string(),
                "connect".to_string(),
                "create".to_string(),
                "delete".to_string(),
                "disconnect".to_string(),
                "download".to_string(),
                "edit".to_string(),
                "exceed".to_string(),
                "expire".to_string(),
                "execute".to_string(),
                "fail".to_string(),
                "find".to_string(),
                "get".to_string(),
                "install".to_string(),
                "list".to_string(),
                "load".to_string(),
                "log".to_string(),
                "modify".to_string(),
                "move".to_string(),
                "open".to_string(),
                "parse".to_string(),
                "print".to_string(),
                "pull".to_string(),
                "push".to_string(),
                "read".to_string(),
                "remove".to_string(),
                "run".to_string(),
                "save".to_string(),
                "search".to_string(),
                "show".to_string(),
                "start".to_string(),
                "stop".to_string(),
                "test".to_string(),
                "time out".to_string(),
                "upload".to_string(),
                "use".to_string(),
                "verify".to_string(),
                "watch".to_string(),
                "write".to_string(),
            ],
        }
    }

    pub fn add(&mut self, verb: String) {
        self.verbs.push(verb);
    }

    pub fn list(&self) -> Vec<String> {
        self.verbs.clone()
    }
}

pub struct Adjectives {
    pub adjectives: Vec<String>,
}

impl Adjectives {
    pub fn new() -> Self {
        Adjectives {
            adjectives: vec![
                "unreachable".to_string(),
                "unresponsive".to_string(),
                "unsigned".to_string(),
                "unstable".to_string(),
                "unsuccessful".to_string(),
            ],
        }
    }
}

#[derive(Debug)]
/// Individual log events including their build ID, metadata, and content
pub struct Event {
    pub build_id: String,
    pub metadata: Option<Vec<String>>,
    pub content: String,
}

/// Collects all log events for a given build ID
///
/// # Arguments
///
/// * `build_id` - The build ID for the requested job
/// * `path` - The root data directory
///
/// # Returns
///
/// A vector of log events containing elements from the failure-relevant corpora
pub async fn collect_events(build_id: String, path: String) -> Vec<String> {
    if !Path::new(&path).exists() {
        download_artifacts(&build_id, &path).await;
    }
    let file_index = create_file_index(path.clone()).await;
    let mut events: Vec<String> = Vec::new();

    for file in file_index {
        let file_open = std::fs::File::open(&file);
        if file_open.is_ok() {
            let file_contents = file_open.unwrap();
            let reader = std::io::BufReader::new(file_contents);
            for line in reader.lines() {
                if line.is_ok() {
                    let line_contents = line.unwrap();
                    if line_contents.contains(Nouns::new().nouns[0].as_str()) {
                        let event = line_contents;
                        events.push(event);
                        println!("Event: {}", events.last().unwrap());
                    }
                }
            }
            //}
        }
    }
    return events;
}

#[cfg(test)]
mod tests {}
