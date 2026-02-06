# Tauri Plugin: User Input

Cross-platform Tauri v2 plugin for monitoring and simulating keyboard and mouse events.

Uses [monio](https://github.com/HuakunShen/monio) for event listening and key simulation, and [enigo](https://github.com/enigo-rs/enigo) for text input, mouse buttons, mouse movement, and scroll.

## Platform Support

| Platform | Monitoring | Simulation |
|----------|-----------|------------|
| macOS    | Yes       | Yes        |
| Windows  | Yes       | Yes        |
| Linux    | Partial   | Partial    |
| iOS      | No        | No         |
| Android  | No        | No         |

> **macOS**: Requires Accessibility permissions (System Settings > Privacy & Security > Accessibility).

> **Linux**: X11 is supported. Wayland support is limited. See [monio docs](https://github.com/HuakunShen/monio) for details.

## Installation

### Rust

Add the plugin to your Tauri app's `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-user-input = "0.1"
```

Register the plugin in your Tauri app:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_user_input::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript

Install the npm package:

```bash
npm install tauri-plugin-user-input-api
# or
pnpm add tauri-plugin-user-input-api
```

### Permissions

Add the plugin permissions to your app's capabilities file (`src-tauri/capabilities/default.json`):

```json
{
  "permissions": [
    "core:default",
    "user-input:default"
  ]
}
```

## JavaScript API

```typescript
import {
  startListening,
  stopListening,
  setEventTypes,
  setWindowLabels,
  isListening,
  key,
  text,
  button,
  moveMouse,
  scroll,
} from "tauri-plugin-user-input-api";
```

### Event Monitoring

Before listening, set which event types you want to receive:

```typescript
import { setEventTypes, startListening, stopListening } from "tauri-plugin-user-input-api";

await setEventTypes(["KeyPress", "KeyRelease", "MouseMove", "ButtonPress", "Wheel"]);

await startListening((event) => {
  console.log("Event:", event.eventType, event);
});

// Later...
await stopListening();
```

Available event types: `KeyPress`, `KeyRelease`, `ButtonPress`, `ButtonRelease`, `MouseMove`, `MouseDragged`, `Wheel`.

### Key Simulation

```typescript
import { key } from "tauri-plugin-user-input-api";

// Press and release a key
await key("KeyClick", "KeyA");

// Hold a key
await key("KeyPress", "MetaLeft");
await key("KeyClick", "KeyC");
await key("KeyRelease", "MetaLeft");

// With delay before execution
await key("KeyClick", "Enter", { delayMs: 100 });
```

Key names follow the [monio Key enum](https://docs.rs/monio/latest/monio/keycode/enum.Key.html) (e.g., `KeyA`, `ShiftLeft`, `ControlLeft`, `MetaLeft`, `ArrowUp`, `Enter`, `Space`, `Backspace`, `F1`-`F24`).

### Text Input

```typescript
import { text } from "tauri-plugin-user-input-api";

await text("Hello, world!");
```

### Mouse Simulation

```typescript
import { button, moveMouse, scroll } from "tauri-plugin-user-input-api";

// Move mouse (absolute or relative coordinates)
await moveMouse(500, 300, "Abs");
await moveMouse(10, -5, "Rel");

// Click a mouse button
await button("Clicked", "Left");
await button("Pressed", "Right");
await button("Released", "Right");

// Scroll
await scroll(3, "Vertical");
await scroll(-2, "Horizontal");
```

Button values: `Left`, `Middle`, `Right`, `Back`, `Forward`, `ScrollUp`, `ScrollDown`, `ScrollLeft`, `ScrollRight`.

### Utility

```typescript
// Check if currently listening
const listening = await isListening();

// Set which windows receive events via Tauri's emit system
await setWindowLabels(["main"]);
```

## Rust API

Access the plugin API from Rust via the `UserInputExt` trait:

```rust
use tauri_plugin_user_input::UserInputExt;

// In a command or setup handler with access to AppHandle:
let user_input = app_handle.user_input();

// Key simulation
user_input.key(monio::Key::KeyA, tauri_plugin_user_input::EventType::KeyClick).unwrap();

// Text input
user_input.text("Hello from Rust!").unwrap();

// Mouse
user_input.move_mouse(100, 200, enigo::Coordinate::Abs).unwrap();
user_input.button(enigo::Button::Left, enigo::Direction::Click).unwrap();
user_input.scroll(3, enigo::Axis::Vertical).unwrap();

// Listening state
let is_active = user_input.is_listening();
```

## Event Data

Events received in the `startListening` callback have this shape:

```typescript
{
  eventType: "KeyPress" | "KeyRelease" | "ButtonPress" | "ButtonRelease" | "MouseMove" | "MouseDragged" | "Wheel",
  time: number,          // Unix timestamp in milliseconds
  key?: string,          // For key events (e.g., "KeyA", "ShiftLeft")
  button?: string,       // For button events ("Left", "Right", "Middle")
  position?: { x: number, y: number },           // For mouse move/drag events
  deltaPosition?: { deltaX: number, deltaY: number }  // For wheel events
}
```

## License

MIT OR Apache-2.0
