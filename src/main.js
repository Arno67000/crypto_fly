const { invoke } = window.__TAURI__.tauri;

let pass;
let decryptForm;
let encryptForm;
let error;
let retry;

const RESPONSE_STATUS = {
  0: "OK",
  4: "No corresponding files found in current location",
  5: "Cannot access directory, permission denied",
  6: "Failed to write or modify file",
  7: "Encryption failed",
  8: "Decryption failed",
};

function disable(elements) {
  elements.forEach((element) => {
    if (element) element.classList.add("disabled");
  });
}

function enable(elements) {
  elements.forEach((element) => {
    if (element) element.classList.remove("disabled");
  });
}

async function checkFiles() {
  error.innerHTML = "";
  disable([error, retry]);
  if (await invoke("get_vault")) {
    enable([decryptForm]);
    document.querySelector("#pass_decrypt").focus();
  } else if (await invoke("get_zip")) {
    enable([encryptForm]);
    document.querySelector("#pass_encrypt").focus();
  } else {
    error.innerHTML = RESPONSE_STATUS[4];
    enable([error, retry]);
  }
}

async function handleFile(pass) {
  disable([decryptForm, encryptForm, error, retry]);
  const result = await invoke("handle_file_encryption", { pass });
  return result;
}

async function run(input) {
  const result = await handleFile(input.value);
  if (result != 0) {
    error.innerHTML = RESPONSE_STATUS[result];
    enable([error, retry]);
  } else {
    input.value = "";
    await checkFiles();
  }
}

window.addEventListener("DOMContentLoaded", () => {
  // initialize variables
  decryptForm = document.querySelector("#decrypt");
  encryptForm = document.querySelector("#encrypt");
  error = document.querySelector("#error");
  retry = document.querySelector("#retry");

  checkFiles();
  retry.addEventListener(
    "click",
    (e) => {
      e.preventDefault();
      retry.innerHTML = "";
      document.querySelector("#pass_decrypt").value = "";
      document.querySelector("#pass_encrypt").value = "";
      checkFiles();
    },
    false
  );
  decryptForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    await run(document.querySelector("#pass_decrypt"));
  });
  encryptForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    await run(document.querySelector("#pass_encrypt"));
  });
});
