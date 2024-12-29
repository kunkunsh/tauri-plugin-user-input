import { Channel, invoke } from "@tauri-apps/api/core";
import { EventType, Key } from "./types";
export * from "./types";

const pluginPrefix = "plugin:user-input|";
const constructCommandKey = (command: string) => `${pluginPrefix}${command}`;

/**
 * Remember to call `setEventTypes()`
 * @param onEvent - A callback function that will be called with the event data.
 * @returns A promise that resolves when the listening starts.
 */
export async function startListening(onEvent?: (event: InputEvent) => void) {
  const onEventChannel = new Channel<any>();
  if (onEvent) {
    onEventChannel.onmessage = onEvent;
  }
  return await invoke(constructCommandKey("start_listening"), {
    onEvent: onEventChannel,
  });
}

export async function stopListening() {
  return await invoke(constructCommandKey("stop_listening"));
}

export async function setWindowLabels(windowLabels: string[]) {
  return await invoke(constructCommandKey("set_window_labels"), {
    windowLabels,
  });
}

export async function setEventTypes(eventTypes: EventType[]) {
  return await invoke(constructCommandKey("set_event_types"), {
    eventTypes,
  });
}

export function isListening() {
  return invoke(constructCommandKey("is_listening"));
}

export async function key(
  direction: "Pressed" | "Released" | "Clicked",
  key: Key
) {
  return await invoke(constructCommandKey("key"), {
    key: JSON.stringify(key),
    direction,
  });
}

export function text(text: string) {
  return invoke(constructCommandKey("text"), {
    text,
  });
}

export function button(
  direction: "Pressed" | "Released" | "Clicked",
  button:
    | "Left"
    | "Middle"
    | "Right"
    | "Back"
    | "Forward"
    | "ScrollUp"
    | "ScrollDown"
    | "ScrollLeft"
    | "ScrollRight"
) {
  return invoke(constructCommandKey("button"), {
    button,
    direction,
  });
}

export function moveMouse(x: number, y: number, coordinate: "Abs" | "Rel") {
  return invoke(constructCommandKey("move_mouse"), {
    x,
    y,
    coordinate,
  });
}

export function scroll(length: number, axis: "Horizontal" | "Vertical") {
  return invoke(constructCommandKey("scroll"), {
    length,
    axis,
  });
}
