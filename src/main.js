const { invoke } = window.__TAURI__.core;

const input = document.getElementById("input");
const button = document.getElementById("button");
const results = document.getElementById("results");

let latest = 0;
async function onEdit() {
  const ticket = ++latest;
  const lines = await invoke("on_edit", { userInput: input.value })
  if (ticket !== latest) return; // a newer keystroke already superseded this result

  results.replaceChildren(
      ...lines.map((text) => {
        const div = document.createElement("div");
        div.className = "result";
        div.textContent = text; // textContent, never innerHTML, so results can't inject markup
        return div;
      })
  );
}

async function onEnter() {
  await invoke("on_enter", { userInput: input.value });
}

async function onButton() {
  await invoke("hide_popup");
}

input.addEventListener("input", onEdit);
input.addEventListener("keydown", e => {
  if(e.key === 'Enter') onEnter();
});

button.addEventListener("click", onButton);

window.addEventListener("keydown", e => {
  if(e.key === "Escape") onButton();
})

window.__TAURI__.window.getCurrentWindow().onFocusChanged(({ payload: focused }) => {
    if (focused) {
        input.focus();
        input.select(); // optional: next keystroke replaces the old query
    }
});