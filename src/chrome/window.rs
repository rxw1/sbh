use serde::Deserialize;
use serde::Serialize;

use crate::chrome::tab::Tabs;

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub type Windows = Vec<Window>;

// TODO Parse docs
/// https://developer.chrome.com/docs/extensions/reference/windows
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Window {
    // Session Buddy Specific and not part of the official Chrome API.
    #[serde(default, rename = "nx_title")]
    nx_title: String,

    /// Whether the window is set to be always on top.
    #[serde(default, skip_serializing_if = "is_default")]
    always_on_top: bool,

    /// Whether the window is currently the focused window.
    #[serde(default, skip_serializing_if = "is_default")]
    focused: bool,

    /// The height of the window, including the frame, in pixels. In some circumstances a window
    /// may not be assigned a height property; for example, when querying closed windows from the
    /// sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,

    /// The ID of the window. Window IDs are unique within a browser session. In some circumstances
    /// a window may not be assigned an ID property; for example, when querying windows using the
    /// sessions API, in which case a session ID may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,

    /// Whether the window is incognito.
    #[serde(default, skip_serializing_if = "is_default")]
    incognito: bool,

    /// The offset of the window from the left edge of the screen in pixels. In some circumstances
    /// a window may not be assigned a left property; for example, when querying closed windows
    /// from the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,

    /// The session ID used to uniquely identify a window, obtained from the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,

    /// The state of this browser window.
    #[serde(skip_serializing_if = "Option::is_none")]
    //state: Option<WindowState>,
    state: Option<String>,

    /// Array of tabs.Tab objects representing the current tabs in the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    tabs: Option<Tabs>,

    /// The offset of the window from the top edge of the screen in pixels. In some circumstances a
    /// window may not be assigned a top property; for example, when querying closed windows from
    /// the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<i64>,

    /// The type of browser window this is.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    //type_: Option<WindowType>,
    type_: Option<String>,

    /// The width of the window, including the frame, in pixels. In some circumstances a window may
    /// not be assigned a width property; for example, when querying closed windows from the
    /// sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
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
