use crate::ui::components::text_input;
use crate::ui::i18n;
use gpui_kit::base::StyledExt as _;
use gpui_kit::component::{
    ActiveTheme as _, IconName, Sizable as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    menu::ContextMenuExt as _,
    scroll::Scrollbar,
    v_flex,
};
use gpui_kit::*;
use panda_core::Scope;

use super::state::ArticleList;

impl Render for ArticleList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let list_entity = cx.entity().downgrade();
        let menu_emit = super::super::context_menus::pane_menu_emitter(list_entity.clone());
        let language = self.language;
        let section_title = match &self.scope {
            Scope::All => i18n::text(language, "All Articles").to_string(),
            Scope::Unread => i18n::text(language, "Unread").to_string(),
            Scope::Starred => i18n::text(language, "Starred").to_string(),
            Scope::Later => i18n::text(language, "Read Later").to_string(),
            Scope::Folder(name) => name.clone(),
            Scope::Feed(id) => self
                .feed_titles
                .get(id)
                .cloned()
                .unwrap_or_else(|| i18n::text(language, "Feed").into()),
        };
        let selected_article = self.selected_article;
        let articles = self.articles.clone();
        let target_language = self.target_language.clone();
        let mut list = v_flex()
            .w(px(340.))
            .h_full()
            .bg(cx.theme().colors.list)
            .border_r_1()
            .border_color(cx.theme().border)
            .child(
                v_flex()
                    .gap_3()
                    .px_5()
                    .pt_5()
                    .pb_4()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_xl().font_semibold().child(section_title))
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        Button::new("mark-all-read")
                                            .small()
                                            .ghost()
                                            .tooltip(i18n::text(language, "Mark all as read"))
                                            .label(i18n::text(language, "Read all"))
                                            .on_click({
                                                let list_entity = list_entity.clone();
                                                move |_, _, cx| {
                                                    let _ = list_entity.update(cx, |_, cx| {
                                                        cx.emit(
                                                            super::ArticleListEvent::MarkAllRead,
                                                        );
                                                    });
                                                }
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(i18n::format(
                                                language,
                                                "{} articles",
                                                self.articles.len(),
                                            )),
                                    ),
                            ),
                    )
                    .child(text_input(&self.search_input)),
            );
        if self.is_loading {
            list = list.child(
                div()
                    .p_5()
                    .text_color(cx.theme().muted_foreground)
                    .child(i18n::text(language, "Loading articles…")),
            );
        } else if self.articles.is_empty() {
            list = list.child(
                v_flex()
                    .items_center()
                    .gap_2()
                    .px_5()
                    .py_12()
                    .text_color(cx.theme().muted_foreground)
                    .child(div().text_2xl().child(IconName::Inbox))
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .child(i18n::text(language, "No articles yet")),
                    )
                    .child(div().text_xs().child(i18n::text(
                        language,
                        "Connect the selected provider in Settings to see your articles here",
                    ))),
            );
        } else {
            let scroll = self.scroll.clone();
            let can_load_more = self.has_more;
            list = list.child(
                div()
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        uniform_list(
                            "article-list-scroll",
                            articles.len(),
                            move |range, window, cx| {
                                if can_load_more && range.end + 4 >= articles.len() {
                                    let list_entity = list_entity.clone();
                                    window
                                        .spawn(cx, async move |cx| {
                                            let _ = list_entity.update(cx, |_, cx| {
                                                cx.emit(super::ArticleListEvent::LoadMoreArticles);
                                            });
                                        })
                                        .detach();
                                }
                                range
                                    .map(|index| {
                                        let article = &articles[index];
                                        let id = article.id;
                                        let date = article
                                            .published_at
                                            .as_deref()
                                            .and_then(|value| {
                                                super::super::date::format_published_at(
                                                    value, language, false,
                                                )
                                            })
                                            .unwrap_or_default();
                                        let menu_emit = menu_emit.clone();
                                        let menu_article = article.clone();
                                        let list_entity_for_article = list_entity.clone();
                                        v_flex()
                                            .id(("article-row", id as u64))
                                            .h(px(80.))
                                            .w_full()
                                            .gap_1()
                                            .px_4()
                                            .py_2()
                                            .overflow_hidden()
                                            .cursor_pointer()
                                            .border_b_1()
                                            .border_color(cx.theme().border)
                                            .bg(if selected_article == Some(id) {
                                                cx.theme().list_active
                                            } else {
                                                cx.theme().colors.list
                                            })
                                            .hover(|style| style.bg(cx.theme().list_hover))
                                            .on_click(move |_, _, cx| {
                                                let _ =
                                                    list_entity_for_article.update(cx, |_, cx| {
                                                        cx.emit(
                                                            super::ArticleListEvent::SelectArticle(
                                                                id,
                                                            ),
                                                        );
                                                    });
                                            })
                                            .context_menu(move |menu, window, cx| {
                                                super::super::context_menus::article_context_menu(
                                                    menu,
                                                    window,
                                                    cx,
                                                    &menu_article,
                                                    &menu_emit,
                                                    language,
                                                    false,
                                                )
                                            })
                                            .child(
                                                h_flex()
                                                    .items_start()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .mt(px(6.))
                                                            .w(px(6.))
                                                            .h(px(6.))
                                                            .flex_shrink_0()
                                                            .rounded_full()
                                                            .bg(if article.is_read {
                                                                cx.theme().muted
                                                            } else {
                                                                cx.theme().primary
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .min_w_0()
                                                            .text_sm()
                                                            .font_semibold()
                                                            .text_color(if article.is_read {
                                                                cx.theme().foreground.opacity(0.78)
                                                            } else {
                                                                cx.theme().foreground
                                                            })
                                                            .line_clamp(2)
                                                            .child(display_title(
                                                                article,
                                                                &target_language,
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                h_flex()
                                                    .justify_between()
                                                    .gap_2()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .min_w_0()
                                                            .overflow_hidden()
                                                            .truncate()
                                                            .child(article.feed_title.clone()),
                                                    )
                                                    .child(div().flex_shrink_0().child(date)),
                                            )
                                    })
                                    .collect::<Vec<_>>()
                            },
                        )
                        .flex_1()
                        .size_full()
                        .track_scroll(&scroll),
                    )
                    .child(Scrollbar::vertical(&scroll)),
            );
        }
        list
    }
}

fn display_title(article: &panda_core::ArticleSummary, target: &str) -> String {
    let hash = panda_translate::title_source_hash(&article.title);
    if article.auto_translated_title_lang.as_deref() == Some(target)
        && article.auto_translated_title_source_hash.as_deref() == Some(hash.as_str())
    {
        if let Some(title) = article
            .auto_translated_title
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            return title.to_owned();
        }
    }
    article.title.clone()
}
