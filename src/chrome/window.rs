use serde::Deserialize;
use serde::Serialize;

use crate::chrome::tab::Tabs;

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub type Windows = Vec<Window>;

impl Window {
    pub fn new() -> Self {
        Window {
            ..Default::default()
        }
    }
}

//#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct Window {
//    pub always_on_top: bool,
//    pub focused: bool,
//    pub height: i64,
//    pub id: i64,
//    pub incognito: bool,
//    pub left: i64,
//    pub state: String,
//    pub tabs: Vec<Tab>,
//    pub top: i64,
//    #[serde(rename = "type")]
//    pub type_: String,
//    pub width: i64,
//}

// TODO Parse docs
/// https://developer.chrome.com/docs/extensions/reference/windows
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Window {
    /// Session Buddy Specific and not part of the official Chrome API.
    #[serde(
        default,
        rename = "nx_title",
        skip_serializing_if = "Option::is_none"
    )]
    pub nx_title: Option<String>,

    /// Whether the window is set to be always on top.
    //#[serde(default, skip_serializing_if = "is_default")]
    #[serde(default)]
    pub always_on_top: bool,

    /// Whether the window is currently the focused window.
    #[serde(default, skip_serializing_if = "is_default")]
    pub focused: bool,

    /// The height of the window, including the frame, in pixels. In some circumstances a window
    /// may not be assigned a height property; for example, when querying closed windows from the
    /// sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// The ID of the window. Window IDs are unique within a browser session. In some circumstances
    /// a window may not be assigned an ID property; for example, when querying windows using the
    /// sessions API, in which case a session ID may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Whether the window is incognito.
    //#[serde(default, skip_serializing_if = "is_default")]
    #[serde(default)]
    pub incognito: bool,

    /// The offset of the window from the left edge of the screen in pixels. In some circumstances
    /// a window may not be assigned a left property; for example, when querying closed windows
    /// from the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,

    /// The session ID used to uniquely identify a window, obtained from the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// The state of this browser window.
    #[serde(skip_serializing_if = "Option::is_none")]
    //state: Option<WindowState>,
    pub state: Option<String>,

    /// Array of tabs.Tab objects representing the current tabs in the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Tabs>,

    /// The offset of the window from the top edge of the screen in pixels. In some circumstances a
    /// window may not be assigned a top property; for example, when querying closed windows from
    /// the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,

    /// The type of browser window this is.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    //type_: Option<WindowType>,
    pub type_: Option<String>,

    /// The width of the window, including the frame, in pixels. In some circumstances a window may
    /// not be assigned a width property; for example, when querying closed windows from the
    /// sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

// TODO
//#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//enum WindowState {
//    Normal,
//    Minimized,
//    Maximized,
//    FullScreen,
//    LockedFullScreen
//}

// TODO
//#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//enum WindowType {
//    Normal,
//    Popup,
//    Panel,
//    App,
//    DevTools
//}
