import { CONFIG } from "./constants.js";
import { AppError, ERROR_TYPES } from "./error-handler.js";

const { NAME, VERSION, STORE_NAME } = CONFIG.DB;

const dbManager = (() => {
  let dbInstance = null;

  async function getConnection() {
    if (dbInstance) return dbInstance;

    try {
      dbInstance = await new Promise((resolve, reject) => {
        const request = indexedDB.open(NAME, VERSION);

        request.onerror = () =>
          reject(
            new AppError(ERROR_TYPES.DATABASE, "connection", request.error)
          );

        request.onupgradeneeded = () => {
          const db = request.result;
          if (!db.objectStoreNames.contains(STORE_NAME)) {
            db.createObjectStore(STORE_NAME);
          }
        };

        request.onsuccess = () => resolve(request.result);
      });

      return dbInstance;
    } catch (error) {
      console.error("Database connection error:", error);
      throw error;
    }
  }

  return { getConnection };
})();

async function createTransaction(mode = "readonly") {
  try {
    const db = await dbManager.getConnection();
    const tx = db.transaction(STORE_NAME, mode);
    const store = tx.objectStore(STORE_NAME);

    return { tx, store };
  } catch (error) {
    throw new AppError(ERROR_TYPES.DATABASE, "transaction", error);
  }
}

export async function saveRenderedHTML(html) {
  try {
    const { tx, store } = await createTransaction("readwrite");

    return new Promise((resolve, reject) => {
      const request = store.put(html, CONFIG.DB.KEY);

      request.onsuccess = () => resolve();

      request.onerror = () =>
        reject(new AppError(ERROR_TYPES.DATABASE, "save", request.error));

      tx.oncomplete = () => resolve();
      tx.onerror = () =>
        reject(new AppError(ERROR_TYPES.DATABASE, "save", tx.error));
    });
  } catch (error) {
    console.error("Save HTML error:", error);
    throw error;
  }
}

export async function loadRenderedHTML() {
  try {
    const { store } = await createTransaction("readonly");

    return new Promise((resolve, reject) => {
      const request = store.get(CONFIG.DB.KEY);

      request.onsuccess = () => resolve(request.result);

      request.onerror = () =>
        reject(new AppError(ERROR_TYPES.DATABASE, "load", request.error));
    });
  } catch (error) {
    console.error("Load HTML error:", error);
    throw error;
  }
}

export async function clearRenderedHTML() {
  try {
    const { tx, store } = await createTransaction("readwrite");

    return new Promise((resolve, reject) => {
      const request = store.delete(CONFIG.DB.KEY);

      request.onsuccess = () => resolve();

      request.onerror = () =>
        reject(new AppError(ERROR_TYPES.DATABASE, "clear", request.error));

      tx.oncomplete = () => resolve();
      tx.onerror = () =>
        reject(new AppError(ERROR_TYPES.DATABASE, "clear", tx.error));
    });
  } catch (error) {
    console.error("Clear HTML error:", error);
    throw error;
  }
}
