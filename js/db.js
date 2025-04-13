const DB_NAME = "MarkdownContentCache";
const DB_VERSION = 1;
const STORE_NAME = "renderedHTML";

function openDatabase() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onerror = () => reject(request.error);

    request.onupgradeneeded = () => {
      const db = request.result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME);
      }
    };

    request.onsuccess = () => resolve(request.result);
  });
}

async function saveRenderedHTML(html) {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, "readwrite");
    const store = tx.objectStore(STORE_NAME);
    store.put(html, "markdown");

    tx.oncomplete = () => {
      console.log("saved");
      resolve();
    };
    tx.onerror = () => reject(tx.error);
  });
}

async function loadRenderedHTML() {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, "readonly");
    const store = tx.objectStore(STORE_NAME);
    const request = store.get("markdown");

    request.onsuccess = () => {
      console.log("loaded");
      resolve(request.result);
    };
    request.onerror = () => reject(request.error);
  });
}

async function clearRenderedHTML() {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, "readwrite");
    const store = tx.objectStore(STORE_NAME);
    store.delete("markdown");

    tx.oncomplete = () => {
      console.log("cleared");
      resolve();
    };
    tx.onerror = () => reject(tx.error);
  });
}

export { saveRenderedHTML, loadRenderedHTML, clearRenderedHTML };
