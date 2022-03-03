use crate::language::Language;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Note {
    color: String,
    isTrashed: bool,
    isPinned: bool,
    isArchived: bool,
    annotations: Option<Vec<serde_json::Value>>,
    textContent: Option<String>,
    title: String,
    userEditedTimestampUsec: u64,
    labels: Option<Vec<serde_json::Value>>,
}

impl Note {
    pub fn is(&self, lang: &Language) -> bool {
        if self.labels == None {
            return false;
        }

        let labels = self.labels.as_ref().unwrap();

        labels.contains(&serde_json::json!({"name": "poem"}))
            && labels.contains(&serde_json::json!({"name": lang.to_note_label()}))
            && !labels.contains(&serde_json::json!({"name": lang.converse().to_note_label()}))
    }

    pub fn content(&self) -> &String {
        self.textContent.as_ref().unwrap()
    }
}
