use serde::Deserialize;
use serde::Serialize;

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub type Tabs = Vec<Tab>;

impl Tab {
    pub fn new() -> Self {
        Tab {
            ..Default::default()
        }
    }
}

// TODO Parse docs
/// https://developer.chrome.com/docs/extensions/reference/tabs/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    /// Whether the tab is active in its window. Does not necessarily mean the window is focused.
    #[serde(default, skip_serializing_if = "is_default")]
    pub active: bool,

    /// Whether the tab has produced sound over the past couple of seconds (but it might not be
    /// heard if also muted). Equivalent to whether the 'speaker audio' indicator is showing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audible: Option<bool>,

    /// Whether the tab can be discarded automatically by the browser when resources are low.
    #[serde(default, skip_serializing_if = "is_default")]
    pub auto_discardable: bool,

    /// Whether the tab is discarded. A discarded tab is one whose content has been unloaded from
    /// memory, but is still visible in the tab strip. Its content is reloaded the next time it is
    /// activated.
    #[serde(default, skip_serializing_if = "is_default")]
    //#[serde(default)]
    pub discarded: bool,

    /// The URL of the tab's favicon. This property is only present if the extension's manifest
    /// includes the "tabs" permission. It may also be an empty string if the tab is loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fav_icon_url: Option<String>,

    /// The ID of the group that the tab belongs to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub group_id: i64,

    /// The height of the tab in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Whether the tab is highlighted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub highlighted: bool,

    /// The ID of the tab. Tab IDs are unique within a browser session. Under some circumstances a
    /// tab may not be assigned an ID; for example, when querying foreign tabs using the sessions
    /// API, in which case a session ID may be present. Tab ID can also be set to
    /// chrome.tabs.TAB_ID_NONE for apps and devtools windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Whether the tab is in an incognito window.
    //#[serde(default, skip_serializing_if = "is_default")]
    #[serde(default)]
    pub incognito: bool,

    /// The zero-based index of the tab within its window.
    //#[serde(default, skip_serializing_if = "is_default")]
    #[serde(default)]
    pub index: i64,

    /// The tab's muted state and the reason for the last state change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_info: Option<MutedInfo>,

    /// The ID of the tab that opened this tab, if any. This property is only present if the opener
    /// tab still exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opener_tab_id: Option<i64>,

    /// The URL the tab is navigating to, before it has committed. This property is only present if
    /// the extension's manifest includes the "tabs" permission and there is a pending navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_url: Option<String>,

    /// Whether the tab is pinned.
    //#[serde(default, skip_serializing_if = "is_default")]
    #[serde(default)]
    pub pinned: bool,

    /// Whether the tab is selected. (Deprecated! Please use tabs.Tab.highlighted.)
    /// Session Buddy still uses this.
    #[serde(default)]
    pub selected: bool,

    /// The session ID used to uniquely identify a tab obtained from the sessions API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// The tab's loading status.
    #[serde(skip_serializing_if = "Option::is_none")]
    //status: Option<TabStatus>,
    pub status: Option<String>,

    /// The title of the tab. This property is only present if the extension's manifest includes
    /// the "tabs" permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The last committed URL of the main frame of the tab. This property is only present if the
    /// extension's manifest includes the "tabs" permission and may be an empty string if the tab
    /// has not yet committed. See also Tab.pendingUrl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The width of the tab in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// The ID of the window that contains the tab.
    #[serde(default, skip_serializing_if = "is_default")]
    pub window_id: i64,
}

// TODO
///// The tab's loading status.
//#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//enum TabStatus {
//    Unloaded(String),
//    Loading(String),
//    Complete(String),
//}

//#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct MutedInfo {
//    pub muted: bool,
//}


/// The tab's muted state and the reason for the last state change.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutedInfo {
    /// The ID of the extension that changed the muted state. Not set if an extension was not the
    /// reason the muted state last changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,

    /// Whether the tab is muted (prevented from playing sound). The tab may be muted even if it
    /// has not played or is not currently playing sound. Equivalent to whether the 'muted' audio
    /// indicator is showing.
    pub muted: bool,

    /// The reason the tab was muted or unmuted. Not set if the tab's mute state has never been
    /// changed.
    //reason: Option<MutedInfoReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

// TODO
///// An event that caused a muted state change.
//#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//enum MutedInfoReason {
//    User,
//    Capture,
//    Extension,
//}

/// Defines how zoom changes in a tab are handled and at what scope.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZoomSettings {
    /// Used to return the default zoom level for the current tab in calls to tabs.getZoomSettings.
    pub default_zoom_factor: Option<i64>,

    /// Defines how zoom changes are handled, i.e., which entity is responsible for the actual
    /// scaling of the page; defaults to automatic.
    //mode: Option<ZoomSettingsMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Defines whether zoom changes persist for the page's origin, or only take effect in this
    /// tab; defaults to per-origin when in automatic mode, and per-tab otherwise.
    //scope: Option<ZoomSettingsScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// TODO
///// Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling
///// of the page; defaults to automatic.
//#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//enum ZoomSettingsMode {
//    Automatic,
//    Manual,
//    Disabled,
//}

// TODO
///// Defines whether zoom changes persist for the page's origin, or only take effect in this tab;
///// defaults to per-origin when in automatic mode, and per-tab otherwise.
//#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//enum ZoomSettingsScope {
//    PerOrigin,
//    PerTab,
//}
