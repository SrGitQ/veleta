const { invoke } = window.__TAURI__.core;

const { once } = window.__TAURI__.event;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;
const tauri = window.__TAURI__;

async function greet() {
  let greetInputEl = document.querySelector("#greet-input");
  let greetMsgEl = document.querySelector("#greet-msg");
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}


once('download-started', (event) => {
  console.log('Download started:', event.payload);
})


window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector("#download-file").addEventListener("submit", async (e) => {
    e.preventDefault();
    let urlInputEl = document.querySelector("#url-input");
    let url = urlInputEl.value;
    let response = await invoke("download", { url: url });
    const view = getCurrentWebviewWindow();
    view.once('download-completed', (event) => {
      localStorage.setItem('downloaded', JSON.stringify(event.payload));
    })
    console.log(response);
    console.log(tauri);

  });

});
