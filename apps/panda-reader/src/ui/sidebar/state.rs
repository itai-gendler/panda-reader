use crate::app::preferences::{Language, LibrarySource};
use crate::ui::context_menus::PaneMenuEvent;
use gpui_kit::base::VirtualListScrollHandle;
use gpui_kit::*;
use panda_core::Feed;
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

pub(in crate::ui) struct Sidebar {
    pub(in crate::ui) feeds: Vec<Feed>,
    pub(in crate::ui) scroll: VirtualListScrollHandle,
    pub(in crate::ui) pending_remove_feed: Option<i64>,
    pub(in crate::ui) favicon_requested: HashSet<String>,
    pub(in crate::ui) icons_dir: PathBuf,
    pub(in crate::ui) is_refreshing: bool,
    pub(in crate::ui) selected_scope: panda_core::Scope,
    pub(in crate::ui) language: Language,
    pub(in crate::ui) library_source: LibrarySource,
    pub(in crate::ui) folder_order: Vec<String>,
    pub(in crate::ui) collapsed_folders: Vec<String>,
    pub(in crate::ui) folder_icons: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub(in crate::ui) enum SidebarEvent {
    SelectScope(panda_core::Scope),
    Refresh { force: bool },
}

impl EventEmitter<SidebarEvent> for Sidebar {}
impl EventEmitter<PaneMenuEvent> for Sidebar {}

impl Sidebar {
    pub(in crate::ui) fn new(icons_dir: PathBuf, language: Language) -> Self {
        Self {
            feeds: Vec::new(),
            scroll: VirtualListScrollHandle::new(),
            pending_remove_feed: None,
            favicon_requested: HashSet::new(),
            icons_dir,
            is_refreshing: false,
            selected_scope: panda_core::Scope::All,
            language,
            library_source: LibrarySource::Local,
            folder_order: Vec::new(),
            collapsed_folders: Vec::new(),
            folder_icons: BTreeMap::new(),
        }
    }
}
