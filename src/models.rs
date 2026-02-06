use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputEvent {
    pub event_type: EventType,
    pub time: u128,
    #[serde(flatten)]
    pub data: InputEventData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InputEventData {
    Button(monio::Button),
    Position {
        x: f64,
        y: f64,
    },
    DeltaPosition {
        #[serde(rename = "deltaX")]
        delta_x: i64,
        #[serde(rename = "deltaY")]
        delta_y: i64,
    },
    Key(monio::Key),
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum EventType {
    KeyPress,
    KeyRelease,
    KeyClick,
    ButtonPress,
    ButtonRelease,
    ButtonClick,
    MouseMove,
    MouseDragged,
    Wheel,
}

impl From<monio::Event> for InputEvent {
    fn from(event: monio::Event) -> Self {
        let data = match event.event_type {
            monio::EventType::MousePressed
            | monio::EventType::MouseReleased
            | monio::EventType::MouseClicked => {
                if let Some(mouse) = &event.mouse {
                    InputEventData::Button(mouse.button.unwrap_or(monio::Button::Unknown(0)))
                } else {
                    InputEventData::Button(monio::Button::Unknown(0))
                }
            }
            monio::EventType::MouseMoved | monio::EventType::MouseDragged => {
                if let Some(mouse) = &event.mouse {
                    InputEventData::Position {
                        x: mouse.x,
                        y: mouse.y,
                    }
                } else {
                    InputEventData::Position { x: 0.0, y: 0.0 }
                }
            }
            monio::EventType::MouseWheel => {
                if let Some(wheel) = &event.wheel {
                    let (delta_x, delta_y) = match wheel.direction {
                        monio::ScrollDirection::Up => (0, wheel.delta as i64),
                        monio::ScrollDirection::Down => (0, -(wheel.delta as i64)),
                        monio::ScrollDirection::Left => (-(wheel.delta as i64), 0),
                        monio::ScrollDirection::Right => (wheel.delta as i64, 0),
                    };
                    InputEventData::DeltaPosition { delta_x, delta_y }
                } else {
                    InputEventData::DeltaPosition {
                        delta_x: 0,
                        delta_y: 0,
                    }
                }
            }
            monio::EventType::KeyPressed
            | monio::EventType::KeyReleased
            | monio::EventType::KeyTyped => {
                if let Some(kb) = &event.keyboard {
                    InputEventData::Key(kb.key)
                } else {
                    InputEventData::Key(monio::Key::Unknown(0))
                }
            }
            monio::EventType::HookEnabled | monio::EventType::HookDisabled => {
                InputEventData::Key(monio::Key::Unknown(0))
            }
        };

        let event_type = match event.event_type {
            monio::EventType::KeyPressed => EventType::KeyPress,
            monio::EventType::KeyReleased | monio::EventType::KeyTyped => EventType::KeyRelease,
            monio::EventType::MousePressed => EventType::ButtonPress,
            monio::EventType::MouseReleased | monio::EventType::MouseClicked => {
                EventType::ButtonRelease
            }
            monio::EventType::MouseMoved => EventType::MouseMove,
            monio::EventType::MouseDragged => EventType::MouseDragged,
            monio::EventType::MouseWheel => EventType::Wheel,
            monio::EventType::HookEnabled | monio::EventType::HookDisabled => EventType::KeyPress, // fallback, these won't typically be emitted
        };

        Self {
            event_type,
            time: event
                .time
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enigo_button() {
        let b = enigo::Button::Left;
        let b_str = format!("{:?}", b);
        assert_eq!(b_str, "Left");
        let btn: enigo::Button = serde_json::from_str(&format!("\"{}\"", "L")).unwrap();
        assert_eq!(btn, enigo::Button::Left);
        let btn: enigo::Button = serde_json::from_str(&format!("\"{}\"", "Left")).unwrap();
        assert_eq!(btn, enigo::Button::Left);
    }

    #[test]
    fn test_enigo_coordinate() {
        let c = enigo::Coordinate::Abs;
        let c_str = format!("{:?}", c);
        assert_eq!(c_str, "Abs");
        let coord: enigo::Coordinate = serde_json::from_str(&format!("\"{}\"", "Abs")).unwrap();
        assert_eq!(coord, enigo::Coordinate::Abs);
    }
}
