use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub id: String,
    pub folder_id: String,
    pub from: String,
    pub subject: String,
    pub snippet: String,
    pub body: String,
}

pub fn sample_messages() -> Vec<Message> {
    vec![
        Message {
            id: "1".to_string(),
            folder_id: "inbox".to_string(),
            from: "alice@example.com".to_string(),
            subject: "Welcome to Coer Mail".to_string(),
            snippet: "Thanks for trying out the MVP...".to_string(),
            body: "Thanks for trying out the Coer Mail MVP! This is a placeholder message to show how content will appear.".to_string(),
        },
        Message {
            id: "2".to_string(),
            folder_id: "inbox".to_string(),
            from: "bob@example.com".to_string(),
            subject: "Your account setup".to_string(),
            snippet: "Here are the details we discussed...".to_string(),
            body: "Here are the details we discussed about setting up your account. We can customize folders, filters, and more.".to_string(),
        },
        Message {
            id: "3".to_string(),
            folder_id: "sent".to_string(),
            from: "you@example.com".to_string(),
            subject: "Draft outreach".to_string(),
            snippet: "Following up about tomorrow's meeting...".to_string(),
            body: "Following up about tomorrow's meeting. This is a sent message example so the UI has some variety.".to_string(),
        },
        Message {
            id: "4".to_string(),
            folder_id: "archive".to_string(),
            from: "news@example.com".to_string(),
            subject: "Weekly newsletter".to_string(),
            snippet: "A summary of this week's updates...".to_string(),
            body: "A summary of this week's updates from the newsletter you subscribe to. Archived so it's kept for reference.".to_string(),
        },
    ]
}

pub fn sample_messages_for_folder(folder_id: &str) -> Vec<Message> {
    sample_messages()
        .into_iter()
        .filter(|message| message.folder_id == folder_id)
        .collect()
}

pub fn sample_message_by_id(id: &str) -> Option<Message> {
    sample_messages()
        .into_iter()
        .find(|message| message.id == id)
}
