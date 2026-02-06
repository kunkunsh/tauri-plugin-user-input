import * as v from "valibot";

export enum EventTypeEnum {
  KeyPress = "KeyPress",
  KeyRelease = "KeyRelease",
  ButtonPress = "ButtonPress",
  ButtonRelease = "ButtonRelease",
  MouseMove = "MouseMove",
  MouseDragged = "MouseDragged",
  Wheel = "Wheel",
}

export enum Button {
  Left = "Left",
  Right = "Right",
  Middle = "Middle",
}

export const EventType = v.enum(EventTypeEnum);
export type EventType = v.InferOutput<typeof EventType>;

export const InputEvent = v.object({
  eventType: EventType,
  time: v.pipe(
    v.number(),
    v.transform((n) => new Date(n))
  ),
  button: v.optional(v.enum(Button)),
  position: v.optional(
    v.object({
      x: v.number(),
      y: v.number(),
    })
  ),
  deltaPosition: v.optional(
    v.object({
      deltaX: v.number(),
      deltaY: v.number(),
    })
  ),
  key: v.optional(v.string()),
});
export type InputEvent = v.InferOutput<typeof InputEvent>;

export type Key =
  | { Unknown: number }
  | "KeyA"
  | "KeyB"
  | "KeyC"
  | "KeyD"
  | "KeyE"
  | "KeyF"
  | "KeyG"
  | "KeyH"
  | "KeyI"
  | "KeyJ"
  | "KeyK"
  | "KeyL"
  | "KeyM"
  | "KeyN"
  | "KeyO"
  | "KeyP"
  | "KeyQ"
  | "KeyR"
  | "KeyS"
  | "KeyT"
  | "KeyU"
  | "KeyV"
  | "KeyW"
  | "KeyX"
  | "KeyY"
  | "KeyZ"
  | "Num0"
  | "Num1"
  | "Num2"
  | "Num3"
  | "Num4"
  | "Num5"
  | "Num6"
  | "Num7"
  | "Num8"
  | "Num9"
  | "F1"
  | "F2"
  | "F3"
  | "F4"
  | "F5"
  | "F6"
  | "F7"
  | "F8"
  | "F9"
  | "F10"
  | "F11"
  | "F12"
  | "F13"
  | "F14"
  | "F15"
  | "F16"
  | "F17"
  | "F18"
  | "F19"
  | "F20"
  | "F21"
  | "F22"
  | "F23"
  | "F24"
  | "ShiftLeft"
  | "ShiftRight"
  | "ControlLeft"
  | "ControlRight"
  | "AltLeft"
  | "AltRight"
  | "MetaLeft"
  | "MetaRight"
  | "Escape"
  | "Tab"
  | "CapsLock"
  | "Space"
  | "Enter"
  | "Backspace"
  | "Insert"
  | "Delete"
  | "Home"
  | "End"
  | "PageUp"
  | "PageDown"
  | "ArrowUp"
  | "ArrowDown"
  | "ArrowLeft"
  | "ArrowRight"
  | "NumLock"
  | "ScrollLock"
  | "PrintScreen"
  | "Pause"
  | "Grave"
  | "Minus"
  | "Equal"
  | "BracketLeft"
  | "BracketRight"
  | "Backslash"
  | "Semicolon"
  | "Quote"
  | "Comma"
  | "Period"
  | "Slash"
  | "Numpad0"
  | "Numpad1"
  | "Numpad2"
  | "Numpad3"
  | "Numpad4"
  | "Numpad5"
  | "Numpad6"
  | "Numpad7"
  | "Numpad8"
  | "Numpad9"
  | "NumpadAdd"
  | "NumpadSubtract"
  | "NumpadMultiply"
  | "NumpadDivide"
  | "NumpadDecimal"
  | "NumpadEnter"
  | "NumpadEqual"
  | "VolumeUp"
  | "VolumeDown"
  | "VolumeMute"
  | "MediaPlayPause"
  | "MediaStop"
  | "MediaNext"
  | "MediaPrevious"
  | "BrowserBack"
  | "BrowserForward"
  | "BrowserRefresh"
  | "BrowserStop"
  | "BrowserSearch"
  | "BrowserFavorites"
  | "BrowserHome"
  | "LaunchMail"
  | "LaunchApp1"
  | "LaunchApp2"
  | "IntlBackslash"
  | "IntlYen"
  | "IntlRo"
  | "ContextMenu";
