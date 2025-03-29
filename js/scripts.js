document.addEventListener("DOMContentLoaded", function () {
  initializeModalHandlers();
  initializeExampleButton();
  initializeFeedbackForm();
  initializeFileSelection();
});

function initializeModalHandlers() {
  const infoButton = document.getElementById("info-button");
  const modal = document.getElementById("readme-modal");
  const closeModal = document.getElementById("close-modal");
  const langToggle = document.getElementById("lang-toggle");
  const readmeContent = document.getElementById("readme-content");
  let currentLang = "zh";

  infoButton.addEventListener("click", async function () {
    modal.classList.add("active");
    modal.setAttribute("aria-hidden", "false");
    await loadReadme(currentLang);
  });

  closeModal.addEventListener("click", function () {
    modal.classList.remove("active");
    modal.setAttribute("aria-hidden", "true");
  });

  modal.addEventListener("click", function (e) {
    if (e.target === modal) {
      modal.classList.remove("active");
      modal.setAttribute("aria-hidden", "true");
    }
  });

  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape" && modal.classList.contains("active")) {
      modal.classList.remove("active");
      modal.setAttribute("aria-hidden", "true");
    }
  });

  langToggle.addEventListener("click", async function () {
    if (currentLang === "en") {
      currentLang = "zh";
      langToggle.textContent = "English";
    } else {
      currentLang = "en";
      langToggle.textContent = "中文";
    }
    await loadReadme(currentLang);
  });

  async function loadReadme(lang) {
    const readmeUrl = lang === "zh" ? "./README-ZH.md" : "./README.md";

    try {
      readmeContent.innerHTML =
        '<div class="loader active"><div class="loader-spinner"></div><p>Loading README...</p></div>';

      const response = await fetch(readmeUrl);
      if (!response.ok) {
        throw new Error(`Failed to load README: ${response.statusText}`);
      }

      const markdown = await response.text();
      readmeContent.innerHTML = marked.parse(markdown);
    } catch (error) {
      readmeContent.innerHTML = `<div class="error-message active">Error loading README: ${error.message}</div>`;
    }
  }
}

function initializeExampleButton() {
  const useExampleButton = document.getElementById("use-example");
  useExampleButton.addEventListener("click", async function () {
    const fileSelected = document.getElementById("file-selected");
    const filenameDisplay = document.getElementById("filename-display");
    const label = document.getElementById("file2-label");

    try {
      label.textContent = "Loading example file...";

      const response = await fetch("./example.txt");

      if (!response.ok) {
        throw new Error(`Failed to load example file: ${response.status}`);
      }

      const text = await response.text();
      const blob = new Blob([text], { type: "text/plain" });

      const file = new File([blob], "example.txt", {
        type: "text/plain",
      });

      const dataTransfer = new DataTransfer();
      dataTransfer.items.add(file);

      const fileInput = document.getElementById("file2");
      fileInput.files = dataTransfer.files;

      filenameDisplay.textContent = "example.txt";
      fileSelected.classList.add("active");
      label.textContent = "Example file loaded";

      run();
    } catch (error) {
      const errorMessage = document.getElementById("error-message");
      const errorText = document.getElementById("error-text");
      errorText.textContent = error.message;
      errorMessage.classList.add("active");
      label.textContent = "Error loading example";
    }
  });
}

function initializeFeedbackForm() {
  const submitFeedbackBtn = document.getElementById("submit-feedback");
  const feedbackText = document.getElementById("feedback-text");
  const feedbackStatus = document.getElementById("feedback-status");

  submitFeedbackBtn.addEventListener("click", function () {
    if (!feedbackText.value.trim()) {
      feedbackStatus.textContent = "Please enter some feedback";
      feedbackStatus.className = "feedback-status error";
      return;
    }

    submitFeedbackBtn.disabled = true;
    feedbackStatus.textContent = "Sending... This may take up to a minute.";
    feedbackStatus.className = "feedback-status";

    Email.send({
      Host: "smtp.elasticemail.com",
      Username: "2581063732@qq.com",
      Password: "035BB2FB8E77484CF1A6D36FBFED6FBE1181",
      To: "somnia1337x@gmail.com",
      From: "somnia1337x@gmail.com",
      Subject: "csTimer-Analyzer-web Feedback",
      Body: `<h2>Feedback from csTimer Analyzer</h2>
         <p><strong>Date:</strong> ${new Date().toLocaleString()}</p>
         <p><strong>Message:</strong></p>
         <p>${feedbackText.value.replace(/\n/g, "<br>")}</p>`,
    }).then(function (message) {
      submitFeedbackBtn.disabled = false;

      if (message === "OK") {
        feedbackStatus.textContent = "Thank you for your feedback!";
        feedbackStatus.className = "feedback-status success";
        feedbackText.value = "";
      } else {
        feedbackStatus.textContent =
          "Error sending feedback. Please try again.";
        feedbackStatus.className = "feedback-status error";
        console.error("SMTP error:", message);
      }
    });
  });
}

function initializeFileSelection() {
  document.getElementById("file2").addEventListener("change", function (e) {
    const file = e.target.files[0];
    const fileSelected = document.getElementById("file-selected");
    const filenameDisplay = document.getElementById("filename-display");
    const label = document.getElementById("file2-label");

    if (file) {
      filenameDisplay.textContent = file.name;
      fileSelected.classList.add("active");
      label.textContent = "File selected";
      run();
    } else {
      fileSelected.classList.remove("active");
      label.textContent = "Select csTimer Data";
    }
  });
}

window.run = async function () {
  console.log("Waiting for WASM module to load...");
};
