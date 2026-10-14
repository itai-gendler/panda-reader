use crate::services::Command;
use crate::ui::window::ReaderWindow;
use gpui_kit::*;
use panda_core::{ArticleCursor, Scope};
use std::sync::Arc;
use tokio::sync::oneshot;

const ARTICLE_PAGE: i64 = 20;

impl ReaderWindow {
    pub(in crate::ui) fn select_scope(&mut self, scope: Scope, cx: &mut Context<Self>) {
        let article_id = self.update_article_reader(cx, |reader| {
            let article_id = reader.requested_article_id.take();
            reader.request_epoch.next();
            reader.is_extracting = false;
            reader.article = None;
            reader.body_html = SharedString::default();
            reader.body_markdown = SharedString::default();
            reader.showing_translation = false;
            article_id
        });
        if let Some(article_id) = article_id {
            self.services.cancel_article_work(article_id);
        }
        self.update_article_list(cx, |list, _| {
            list.scope = scope.clone();
            list.selected_article = None;
        });
        self.sidebar.update(cx, |sidebar, cx| {
            sidebar.selected_scope = scope;
            cx.notify();
        });
        self.update_article_list(cx, |list, _| {
            list.articles = Arc::new(Vec::new());
            list.has_more = false;
        });
        self.sync_status_bar(cx);
        self.load_snapshot(cx);
    }

    pub(in crate::ui) fn select_scope_preserving_article(
        &mut self,
        scope: Scope,
        cx: &mut Context<Self>,
    ) {
        self.update_article_list(cx, |list, _| {
            list.scope = scope.clone();
            list.articles = Arc::new(Vec::new());
            list.has_more = false;
        });
        self.sidebar.update(cx, |sidebar, cx| {
            sidebar.selected_scope = scope;
            cx.notify();
        });
        self.sync_status_bar(cx);
        self.load_snapshot(cx);
    }

    pub(in crate::ui) fn load_snapshot(&mut self, cx: &mut Context<Self>) {
        self.update_article_list(cx, |list, _| {
            list.pending_navigation = None;
            list.is_loading_more = false;
        });
        self.load_snapshot_page(false, cx);
    }

    pub(in crate::ui) fn load_snapshot_page(&mut self, append: bool, cx: &mut Context<Self>) {
        let (revision, after, scope, search, is_empty) = self.update_article_list(cx, |list, _| {
            if !append {
                list.pending_navigation = None;
            }
            list.snapshot_revision = list.snapshot_revision.wrapping_add(1);
            let revision = list.snapshot_revision;
            let after = if append {
                list.articles.last().map(ArticleCursor::from_summary)
            } else {
                None
            };
            (
                revision,
                after,
                list.scope.clone(),
                list.search.clone(),
                list.articles.is_empty(),
            )
        });
        let (reply, response) = oneshot::channel();
        self.services.send(Command::Snapshot {
            scope,
            search,
            limit: ARTICLE_PAGE,
            after,
            include_feeds: !append,
            reply,
        });
        if !append {
            self.update_article_list(cx, |list, _| list.is_loading = is_empty);
        }
        cx.spawn(async move |this, cx| {
            let result = response
                .await
                .unwrap_or_else(|_| Err("Background service stopped".to_string()));
            let _ = this.update(cx, |this, cx| {
                if revision != this.list.read(cx).snapshot_revision {
                    return;
                }
                this.update_article_list(cx, |list, _| {
                    list.is_loading = false;
                    list.is_loading_more = false;
                });
                let mut continue_navigation = None;
                match result {
                    Ok(snapshot) => {
                        let newly_loaded = snapshot.articles.clone();
                        if !append {
                            let feed_titles = snapshot
                                .feeds
                                .iter()
                                .map(|feed| (feed.id, feed.title.clone()))
                                .collect();
                            this.sidebar.update(cx, |sidebar, cx| {
                                sidebar.feeds = snapshot.feeds;
                                cx.notify();
                            });
                            this.update_article_list(cx, |list, _| {
                                list.feed_titles = feed_titles;
                            });
                            this.sync_folder_order_from_feeds(cx);
                            this.ensure_favicons(cx);
                            this.update_article_list(cx, |list, _| {
                                list.articles = Arc::new(snapshot.articles);
                            });
                        } else if !snapshot.articles.is_empty() {
                            this.update_article_list(cx, |list, _| {
                                Arc::make_mut(&mut list.articles).extend(snapshot.articles);
                            });
                        }
                        let articles = this.update_article_list(cx, |list, _| {
                            list.has_more = snapshot.has_more;
                            if append
                                && let Some((delta, unread_only, previous_len)) =
                                    list.pending_navigation
                            {
                                if list.articles.len() > previous_len {
                                    list.pending_navigation = None;
                                    continue_navigation = Some((delta, unread_only));
                                } else {
                                    list.pending_navigation = None;
                                }
                            }
                            list.articles.clone()
                        });
                        this.update_article_reader(cx, |reader| {
                            if let Some(article) = &mut reader.article {
                                article.summary = articles
                                    .iter()
                                    .find(|row| row.id == article.summary.id)
                                    .cloned()
                                    .unwrap_or_else(|| article.summary.clone());
                            }
                        });
                        this.queue_title_translations(&newly_loaded, cx);
                        if !append && this.startup_sync_pending {
                            this.startup_sync_pending = false;
                            this.refresh(cx);
                        }
                    }
                    Err(error) => {
                        this.update_article_list(cx, |list, _| {
                            list.pending_navigation = None;
                        });
                        this.set_error(error)
                    }
                }
                if let Some((delta, unread_only)) = continue_navigation {
                    this.move_article(delta, unread_only, cx);
                }
                cx.notify();
            });
        })
        .detach();
    }
}

impl ReaderWindow {
    pub(in crate::ui) fn queue_title_translations(
        &mut self,
        rows: &[panda_core::ArticleSummary],
        cx: &mut Context<Self>,
    ) {
        if !self.preferences.auto_translate_titles || rows.is_empty() {
            return;
        }
        let items: Vec<_> = rows
            .iter()
            .filter(|r| r.feed_auto_translate_titles)
            .filter(|r| !self.preferences.only_translate_future_titles || r.title_is_future)
            .map(|r| crate::services::TitleTranslationInput {
                id: r.id,
                title: r.title.clone(),
            })
            .collect();
        if items.is_empty() {
            return;
        }
        let target = self
            .preferences
            .translation_language
            .translator_code()
            .to_owned();
        let (reply, response) = oneshot::channel();
        self.services.send(Command::TranslateTitles {
            items,
            target_lang: target.clone(),
            force: false,
            reply,
        });
        cx.spawn(async move |this, cx| {
            let result = response
                .await
                .unwrap_or_else(|_| Err("Background service stopped".into()));
            let _ =
                this.update(cx, |this, cx| match result {
                    Ok(outcome) => {
                        this.title_translation_usage = outcome.usage;
                        let status = outcome.status.clone();
                        if this.preferences.auto_translate_titles
                            && this.preferences.translation_language.translator_code() == target
                        {
                            for (id, original, translated, lang) in outcome.translated {
                                this.update_article_list(cx, |list, _| {
                                    if let Some(row) = Arc::make_mut(&mut list.articles)
                                        .iter_mut()
                                        .find(|r| r.id == id && r.title == original)
                                    {
                                        row.auto_translated_title = Some(translated.clone());
                                        row.auto_translated_title_lang = Some(lang.clone());
                                        row.auto_translated_title_source_hash =
                                            Some(panda_translate::title_source_hash(&original));
                                    }
                                });
                                this.update_article_reader(cx, |reader| {
                                    if let Some(article) = reader.article.as_mut().filter(|a| {
                                        a.summary.id == id && a.summary.title == original
                                    }) {
                                        article.summary.auto_translated_title = Some(translated);
                                        article.summary.auto_translated_title_lang = Some(lang);
                                        article.summary.auto_translated_title_source_hash =
                                            Some(panda_translate::title_source_hash(&original));
                                    }
                                });
                            }
                        }
                        if let Some(status) = status {
                            match status {
                                crate::services::TitleTranslationStatus::Info(message) => {
                                    this.set_flash(message, cx);
                                }
                                crate::services::TitleTranslationStatus::Error(message) => {
                                    this.set_error(message);
                                }
                            }
                        }
                        cx.notify();
                    }
                    Err(error) => this.set_error(format!("Title translation failed: {error}")),
                });
        })
        .detach();
    }

    pub(in crate::ui) fn load_title_translation_usage(&mut self, cx: &mut Context<Self>) {
        let (reply, response) = oneshot::channel();
        self.services.send(Command::TranslationUsage { reply });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(usage)) = response.await {
                let _ = this.update(cx, |this, cx| {
                    this.title_translation_usage = usage;
                    cx.notify();
                });
            }
        })
        .detach();
    }

    pub(in crate::ui) fn translate_title_manually(&mut self, id: i64, cx: &mut Context<Self>) {
        let title = self
            .list
            .read(cx)
            .articles
            .iter()
            .find(|row| row.id == id)
            .map(|row| row.title.clone());
        let Some(title) = title else {
            if let Some(title) = self.read_article_reader(cx, |reader| {
                reader
                    .article
                    .as_ref()
                    .filter(|article| article.summary.id == id)
                    .map(|article| article.summary.title.clone())
            }) {
                self.queue_manual_title_translation(id, title, cx);
            }
            return;
        };
        self.queue_manual_title_translation(id, title, cx);
    }

    fn queue_manual_title_translation(&mut self, id: i64, title: String, cx: &mut Context<Self>) {
        let target = self
            .preferences
            .translation_language
            .translator_code()
            .to_owned();
        let (reply, response) = oneshot::channel();
        self.services.send(Command::TranslateTitles {
            items: vec![crate::services::TitleTranslationInput { id, title }],
            target_lang: target,
            force: true,
            reply,
        });
        cx.spawn(async move |this, cx| {
            let result = response
                .await
                .unwrap_or_else(|_| Err("Background service stopped".into()));
            let _ = this.update(cx, |this, cx| match result {
                Ok(outcome) => {
                    this.title_translation_usage = outcome.usage;
                    for (id, original, translated, lang) in outcome.translated {
                        this.update_article_list(cx, |list, _| {
                            if let Some(row) = Arc::make_mut(&mut list.articles)
                                .iter_mut()
                                .find(|row| row.id == id && row.title == original)
                            {
                                row.auto_translated_title = Some(translated.clone());
                                row.auto_translated_title_lang = Some(lang.clone());
                                row.auto_translated_title_source_hash =
                                    Some(panda_translate::title_source_hash(&original));
                            }
                        });
                        this.update_article_reader(cx, |reader| {
                            if let Some(article) = reader.article.as_mut().filter(|article| {
                                article.summary.id == id && article.summary.title == original
                            }) {
                                article.summary.auto_translated_title = Some(translated);
                                article.summary.auto_translated_title_lang = Some(lang);
                                article.summary.auto_translated_title_source_hash =
                                    Some(panda_translate::title_source_hash(&original));
                            }
                        });
                    }
                    if let Some(status) = outcome.status {
                        match status {
                            crate::services::TitleTranslationStatus::Info(message) => {
                                this.set_flash(message, cx)
                            }
                            crate::services::TitleTranslationStatus::Error(message) => {
                                this.set_error(message)
                            }
                        }
                    }
                    cx.notify();
                }
                Err(error) => this.set_error(format!("Title translation failed: {error}")),
            });
        })
        .detach();
    }
}

impl ReaderWindow {
    pub(in crate::ui) fn move_article(
        &mut self,
        delta: isize,
        unread_only: bool,
        cx: &mut Context<Self>,
    ) {
        let articles = self.list.read(cx).articles.clone();
        if articles.is_empty() {
            if delta > 0 && self.can_load_more(cx) {
                self.update_article_list(cx, |list, _| {
                    list.pending_navigation = Some((delta, unread_only, 0));
                });
                self.load_more_articles(cx);
            }
            return;
        }
        let current = self.read_article_reader(cx, |reader| {
            reader.article.as_ref().map(|article| article.summary.id)
        });
        let current_index = current.and_then(|id| articles.iter().position(|row| row.id == id));
        let next_id = if unread_only {
            match (delta > 0, current_index) {
                (true, Some(index)) => articles[index + 1..]
                    .iter()
                    .find(|row| !row.is_read)
                    .map(|row| row.id),
                (true, None) => articles.iter().find(|row| !row.is_read).map(|row| row.id),
                (false, Some(index)) => articles[..index]
                    .iter()
                    .rev()
                    .find(|row| !row.is_read)
                    .map(|row| row.id),
                (false, None) => articles
                    .iter()
                    .rev()
                    .find(|row| !row.is_read)
                    .map(|row| row.id),
            }
        } else {
            let next_index = match current_index {
                Some(index) => {
                    let next = index as isize + delta;
                    if next < 0 || next >= articles.len() as isize {
                        if next >= articles.len() as isize && delta > 0 {
                            self.update_article_list(cx, |list, _| {
                                list.pending_navigation =
                                    Some((delta, unread_only, articles.len()));
                            });
                            self.load_more_articles(cx);
                        }
                        return;
                    }
                    next as usize
                }
                None if delta > 0 => 0,
                None => articles.len().saturating_sub(1),
            };
            Some(articles[next_index].id)
        };
        if let Some(id) = next_id {
            if let Some(index) = articles.iter().position(|row| row.id == id) {
                self.list
                    .read(cx)
                    .scroll
                    .scroll_to_item(index, ScrollStrategy::Center);
            }
            self.open_article(id, cx);
        } else if delta > 0 && self.can_load_more(cx) {
            self.update_article_list(cx, |list, _| {
                list.pending_navigation = Some((delta, unread_only, articles.len()));
            });
            self.load_more_articles(cx);
        }
    }

    pub(in crate::ui) fn load_more_articles(&mut self, cx: &mut Context<Self>) {
        if !self.can_load_more(cx) || self.list.read(cx).is_loading_more {
            return;
        }
        self.update_article_list(cx, |list, _| list.is_loading_more = true);
        self.load_snapshot_page(true, cx);
    }

    pub(in crate::ui) fn can_load_more(&self, cx: &App) -> bool {
        self.list.read(cx).has_more
    }
}
