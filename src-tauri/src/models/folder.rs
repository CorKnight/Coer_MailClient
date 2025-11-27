use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
}

pub fn sample_folders() -> Vec<Folder> {
    vec![
        Folder {
            id: "inbox".to_string(),
            name: "Inbox".to_string(),
        },
        Folder {
            id: "sent".to_string(),
            name: "Sent".to_string(),
        },
        Folder {
            id: "archive".to_string(),
            name: "Archive".to_string(),
        },
    ]
}
