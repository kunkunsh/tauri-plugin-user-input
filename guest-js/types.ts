import * as v from "valibot";

export enum EventTypeEnum {
  KeyPress = "KeyPress",
  KeyRelease = "KeyRelease",
  ButtonPress = "ButtonPress",
  ButtonRelease = "ButtonRelease",
  MouseMove = "MouseMove",
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
  button: v.nullable(v.enum(Button)),
  x: v.nullable(v.number()),
  y: v.nullable(v.number()),
});
export type InputEvent = v.InferOutput<typeof InputEvent>;

export type Key =
  | { Unicode: string }
  | "Alt"
  | "Backspace"
  | "Begin"
  | "Break"
  | "Cancel"
  | "CapsLock"
  | "Clear"
  | "Control"
  | "Delete"
  | "DownArrow"
  | "End"
  | "Escape"
  | "Execute"
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
  | "F25"
  | "F26"
  | "F27"
  | "F28"
  | "F29"
  | "F30"
  | "F31"
  | "F32"
  | "F33"
  | "F34"
  | "F35"
  | "Find"
  | "Hangul"
  | "Hanja"
  | "Help"
  | "Home"
  | "Insert"
  | "Kanji"
  | "LeftArrow"
  | "Linefeed"
  | "LMenu"
  | "ModeChange"
  | "MediaNextTrack"
  | "MediaPlayPause"
  | "MediaPrevTrack"
  | "MediaStop"
  | "Numlock"
  | "PageDown"
  | "PageUp"
  | "Pause"
  | "Print"
  | "PrintScr"
  | "RControl"
  | "Redo"
  | "Return"
  | "RightArrow"
  | "RShift"
  | "ScrollLock"
  | "Select"
  | "ScriptSwitch"
  | "Shift"
  | "ShiftLock"
  | "Space"
  | "SysReq"
  | "Tab"
  | "Undo"
  | "UpArrow"
  | "VolumeDown"
  | "VolumeUp"
  | "VolumeMute"
  | "MicMute"
  | "Command";
