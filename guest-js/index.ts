import { invoke } from "@tauri-apps/api/core";

const pluginPrefix = "plugin:user-input|";
const constructCommandKey = (command: string) => `${pluginPrefix}${command}`;

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:user-input|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function startListening(windowLabels?: string[]) {
  return await invoke(constructCommandKey("start_listening"), {
    windowLabels: windowLabels ?? [],
  });
}

export async function stopListening() {
  return await invoke(constructCommandKey("stop_listening"));
}
