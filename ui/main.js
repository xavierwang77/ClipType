const tauri = window.__TAURI__;
const invoke = tauri?.core?.invoke;
const listen = tauri?.event?.listen;

const elements = {
  enabled: document.querySelector("#enabled"),
  permissionText: document.querySelector("#permissionText"),
  hotkeyText: document.querySelector("#hotkeyText"),
  lastResultText: document.querySelector("#lastResultText"),
  hotkey: document.querySelector("#hotkey"),
  captureHotkey: document.querySelector("#captureHotkey"),
  captureHint: document.querySelector("#captureHint"),
  delayRange: document.querySelector("#delayRange"),
  delayNumber: document.querySelector("#delayNumber"),
  delayValue: document.querySelector("#delayValue"),
  appendEnter: document.querySelector("#appendEnter"),
  save: document.querySelector("#save"),
  typeNow: document.querySelector("#typeNow"),
  permissions: document.querySelector("#permissions"),
};

let capturing = false;

function setBusy(button, busy) {
  button.disabled = busy;
}

function clampDelay(value) {
  const parsed = Number.parseInt(value, 10);
  if (Number.isNaN(parsed)) {
    return 0;
  }
  return Math.min(Math.max(parsed, 0), 1000);
}

function syncDelay(value) {
  const delay = clampDelay(value);
  elements.delayRange.value = Math.min(delay, 200).toString();
  elements.delayNumber.value = delay.toString();
  elements.delayValue.textContent = `${delay} ms`;
}

function renderLast(result) {
  if (!result) {
    elements.lastResultText.textContent = "暂无";
    return;
  }

  elements.lastResultText.textContent = result.ok
    ? `已输入 ${result.typed_chars} 个字符`
    : result.message;
}

function renderStatus(status) {
  const config = status.config;
  elements.enabled.checked = config.enabled;
  elements.hotkey.value = config.hotkey;
  elements.hotkeyText.textContent = config.hotkey || "-";
  elements.appendEnter.checked = config.append_enter;
  syncDelay(config.input_delay_ms);

  const permission = status.permission;
  elements.permissionText.textContent = permission.accessibility_required
    ? permission.accessibility_granted
      ? "已授权"
      : "需要辅助功能权限"
    : "无需额外权限";

  renderLast(status.last_result);
}

function readForm() {
  return {
    enabled: elements.enabled.checked,
    hotkey: elements.hotkey.value.trim(),
    input_delay_ms: clampDelay(elements.delayNumber.value),
    append_enter: elements.appendEnter.checked,
  };
}

function keyToHotkeyPart(event) {
  if (/^Key[A-Z]$/.test(event.code)) {
    return event.code.slice(3);
  }
  if (/^Digit[0-9]$/.test(event.code)) {
    return event.code.slice(5);
  }
  if (/^F([1-9]|1[0-9]|2[0-4])$/.test(event.code)) {
    return event.code;
  }

  const names = {
    Backquote: "`",
    BracketLeft: "[",
    BracketRight: "]",
    Comma: ",",
    Equal: "=",
    Escape: "Escape",
    Minus: "-",
    Period: ".",
    Semicolon: ";",
    Slash: "/",
    Space: "Space",
    Tab: "Tab",
  };

  return names[event.code] ?? "";
}

function buildHotkey(event) {
  const parts = [];

  if (event.metaKey || event.ctrlKey) {
    parts.push("CommandOrControl");
  }
  if (event.altKey) {
    parts.push("Alt");
  }
  if (event.shiftKey) {
    parts.push("Shift");
  }

  const key = keyToHotkeyPart(event);
  if (!key || ["ControlLeft", "ControlRight", "MetaLeft", "MetaRight", "AltLeft", "AltRight", "ShiftLeft", "ShiftRight"].includes(event.code)) {
    return "";
  }

  parts.push(key);
  return parts.join("+");
}

async function refresh() {
  if (!invoke) {
    elements.lastResultText.textContent = "Tauri API 不可用";
    return;
  }
  renderStatus(await invoke("get_status"));
}

async function saveSettings() {
  setBusy(elements.save, true);
  try {
    const status = await invoke("save_settings", { config: readForm() });
    renderStatus(status);
  } catch (error) {
    elements.lastResultText.textContent = String(error);
  } finally {
    setBusy(elements.save, false);
  }
}

elements.delayRange.addEventListener("input", (event) => {
  syncDelay(event.target.value);
});

elements.delayNumber.addEventListener("input", (event) => {
  syncDelay(event.target.value);
});

elements.enabled.addEventListener("change", async () => {
  if (!invoke) {
    return;
  }
  try {
    const status = await invoke("set_enabled", { enabled: elements.enabled.checked });
    renderStatus(status);
  } catch (error) {
    elements.lastResultText.textContent = String(error);
  }
});

elements.captureHotkey.addEventListener("click", () => {
  capturing = true;
  document.body.classList.add("is-capturing");
  elements.captureHint.textContent = "请按下新的快捷键，Esc 取消";
  elements.hotkey.focus();
});

window.addEventListener("keydown", (event) => {
  if (!capturing) {
    return;
  }

  event.preventDefault();
  if (event.key === "Escape") {
    capturing = false;
    document.body.classList.remove("is-capturing");
    elements.captureHint.textContent = "示例：CommandOrControl+Shift+V";
    return;
  }

  const hotkey = buildHotkey(event);
  if (hotkey) {
    elements.hotkey.value = hotkey;
    capturing = false;
    document.body.classList.remove("is-capturing");
    elements.captureHint.textContent = "录制完成，保存后生效";
  }
});

elements.save.addEventListener("click", saveSettings);

elements.typeNow.addEventListener("click", async () => {
  setBusy(elements.typeNow, true);
  try {
    renderLast(await invoke("trigger_type_clipboard"));
  } catch (error) {
    elements.lastResultText.textContent = String(error);
  } finally {
    setBusy(elements.typeNow, false);
  }
});

elements.permissions.addEventListener("click", async () => {
  if (!invoke) {
    return;
  }
  try {
    renderStatus(await invoke("open_permissions"));
  } catch (error) {
    elements.lastResultText.textContent = String(error);
  }
});

if (listen) {
  listen("cliptype-result", (event) => renderLast(event.payload));
  listen("cliptype-status", (event) => renderStatus(event.payload));
}

refresh();
