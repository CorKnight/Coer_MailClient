const { invoke } = window.__TAURI__.tauri;

const folderList = document.getElementById("folder-list");
const messageList = document.getElementById("message-list");
const messageSubject = document.getElementById("message-subject");
const messageFrom = document.getElementById("message-from");
const messageBody = document.getElementById("message-body");

let currentFolder = "inbox";

async function loadFolders() {
  const folders = await invoke("list_folders");
  folderList.innerHTML = "";

  folders.forEach((folder) => {
    const item = document.createElement("li");
    item.textContent = folder.name;
    item.dataset.id = folder.id;
    if (folder.id === currentFolder) {
      item.classList.add("active");
    }
    item.addEventListener("click", () => {
      currentFolder = folder.id;
      highlightActiveFolder();
      loadMessages();
    });
    folderList.appendChild(item);
  });
}

function highlightActiveFolder() {
  document.querySelectorAll("#folder-list li").forEach((li) => {
    li.classList.toggle("active", li.dataset.id === currentFolder);
  });
}

async function loadMessages() {
  const messages = await invoke("list_messages", { folder_id: currentFolder });
  messageList.innerHTML = "";

  if (!messages.length) {
    const empty = document.createElement("li");
    empty.textContent = "No messages in this folder yet.";
    messageList.appendChild(empty);
    return;
  }

  messages.forEach((message) => {
    const item = document.createElement("li");
    const subject = document.createElement("p");
    subject.className = "message-subject";
    subject.textContent = message.subject;
    const snippet = document.createElement("p");
    snippet.className = "message-snippet";
    snippet.textContent = message.snippet;

    item.appendChild(subject);
    item.appendChild(snippet);
    item.addEventListener("click", () => loadMessageDetail(message.id));
    messageList.appendChild(item);
  });
}

async function loadMessageDetail(id) {
  const message = await invoke("get_message", { id });
  if (!message) {
    messageSubject.textContent = "Message not found";
    messageFrom.textContent = "";
    messageBody.textContent = "";
    return;
  }

  messageSubject.textContent = message.subject;
  messageFrom.textContent = `From: ${message.from}`;
  messageBody.textContent = message.body;
}

async function bootstrap() {
  await loadFolders();
  await loadMessages();
  const firstMessage = await invoke("get_message", { id: "1" });
  if (firstMessage) {
    loadMessageDetail(firstMessage.id);
  }
}

document.addEventListener("DOMContentLoaded", bootstrap);
