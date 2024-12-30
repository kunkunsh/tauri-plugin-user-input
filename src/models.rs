use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

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
    Button(rdev::Button),
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
    Key(rdev::Key),
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
    Wheel,
}

impl From<rdev::Event> for InputEvent {
    fn from(event: rdev::Event) -> Self {
        let data = match event.event_type {
            rdev::EventType::ButtonPress(btn) | rdev::EventType::ButtonRelease(btn) => {
                InputEventData::Button(btn)
            }
            rdev::EventType::MouseMove { x, y } => InputEventData::Position { x, y },
            rdev::EventType::Wheel { delta_x, delta_y } => {
                InputEventData::DeltaPosition { delta_x, delta_y }
            }
            rdev::EventType::KeyPress(key) | rdev::EventType::KeyRelease(key) => {
                InputEventData::Key(key)
            }
        };

        let event_type = match event.event_type {
            rdev::EventType::KeyPress(_) => EventType::KeyPress,
            rdev::EventType::KeyRelease(_) => EventType::KeyRelease,
            rdev::EventType::ButtonPress(_) => EventType::ButtonPress,
            rdev::EventType::ButtonRelease(_) => EventType::ButtonRelease,
            rdev::EventType::MouseMove { .. } => EventType::MouseMove,
            rdev::EventType::Wheel { .. } => EventType::Wheel,
        };

        Self {
            event_type,
            time: event.time.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            data,
        }
    }
}

// #[derive(Debug)]
// pub struct Key(enigo::Key);

// impl FromStr for Key {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // Add proper string-to-key conversion logic here
//         let key: enigo::Key = serde_json::from_str(&format!("\"{}\"", s)).map_err(|_| ())?;
//         Ok(Key(key))
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct KeyComb {
//     keys: Vec<enigo::Key>,
// }

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
