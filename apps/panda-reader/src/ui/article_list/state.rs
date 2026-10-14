use crate::ui::context_menus::PaneMenuEvent;
use crate::ui::i18n;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use panda_core::{ArticleSummary, Scope};
use std::{collections::HashMap, sync::Arc};

pub(in crate::ui) struct ArticleList {
    _subscriptions: Vec<Subscription>,
    pub(in crate::ui) search_input: Entity<InputState>,
    pub(in crate::ui) articles: Arc<Vec<ArticleSummary>>,
    pub(in crate::ui) has_more: bool,
    pub(in crate::ui) scroll: UniformListScrollHandle,
    pub(in crate::ui) scope: Scope,
    pub(in crate::ui) search: String,
    pub(in crate::ui) search_revision: u64,
    pub(in crate::ui) snapshot_revision: u64,
    pub(in crate::ui) is_loading: bool,
    pub(in crate::ui) is_loading_more: bool,
    pub(in crate::ui) pending_navigation: Option<(isize, bool, usize)>,
    pub(in crate::ui) language: crate::app::preferences::Language,
    pub(in crate::ui) target_language: String,
    pub(in crate::ui) selected_article: Option<i64>,
    pub(in crate::ui) feed_titles: HashMap<i64, String>,
}

#[derive(Clone, Debug)]
pub(in crate::ui) enum ArticleListEvent {
    SearchChanged,
    SelectArticle(i64),
    MarkAllRead,
    LoadMoreArticles,
}

impl EventEmitter<ArticleListEvent> for ArticleList {}
impl EventEmitter<PaneMenuEvent> for ArticleList {}

impl ArticleList {
    pub(in crate::ui) fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        language: crate::app::preferences::Language,
    ) -> Self {
        let search_placeholder = i18n::text(language, "Search title, author and body");
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder(search_placeholder));
        let search_for_events = search_input.clone();
        let _subscriptions =
            vec![
                cx.subscribe_in(&search_input, window, move |this, _, event, _, cx| {
                    if matches!(event, InputEvent::Change) {
                        this.search = search_for_events.read(cx).value().to_string();
                        this.search_revision = this.search_revision.wrapping_add(1);
                        cx.emit(ArticleListEvent::SearchChanged);
                    }
                }),
            ];
        Self {
            search_input,
            articles: Arc::new(Vec::new()),
            has_more: false,
            scroll: UniformListScrollHandle::new(),
            scope: Scope::All,
            search: String::new(),
            search_revision: 0,
            snapshot_revision: 0,
            is_loading: true,
            is_loading_more: false,
            pending_navigation: None,
            language,
            target_language: "en".to_owned(),
            selected_article: None,
            feed_titles: HashMap::new(),
            _subscriptions,
        }
    }
}
