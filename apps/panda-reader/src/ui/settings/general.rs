use super::Settings;
use crate::app::preferences::LibrarySource;
use crate::ui::components::{BundledIcon, bundled_icon, text_input};
use crate::ui::i18n;
use crate::ui::window::ReaderWindow;
use gpui_kit::base::Disableable as _;
use gpui_kit::base::{StyledExt as _, v_virtual_list};
use gpui_kit::component::{
    ActiveTheme as _, Sizable as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::Scrollbar,
    select::Select,
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::rc::Rc;

impl Settings {
    pub(in crate::ui) fn render_general_settings(
        &self,
        owner: &ReaderWindow,
        cx: &mut Context<ReaderWindow>,
    ) -> impl IntoElement {
        let selected_provider = self.library_source.provider();
        let sidebar = owner.sidebar.read(cx);
        let active_provider_settings =
            selected_provider.and_then(|provider| self.provider_settings.get(&provider));
        let feed_query = self.added_feeds_search.trim().to_lowercase();
        let feed_indices = sidebar
            .feeds
            .iter()
            .enumerate()
            .filter_map(|(index, feed)| {
                let matches = feed_query.is_empty()
                    || feed.title.to_lowercase().contains(&feed_query)
                    || feed.feed_url.to_lowercase().contains(&feed_query)
                    || feed
                        .folder
                        .as_deref()
                        .is_some_and(|folder| folder.to_lowercase().contains(&feed_query));
                matches.then_some(index)
            })
            .collect::<Vec<_>>();
        let feeds = if feed_indices.is_empty() {
            div()
                .pt_3()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(owner.t("No feeds found"))
                .into_any_element()
        } else {
            let pending_remove = sidebar.pending_remove_feed;
            let item_sizes = Rc::new(
                feed_indices
                    .iter()
                    .map(|index| {
                        let feed = &sidebar.feeds[*index];
                        let row_height = if pending_remove == Some(feed.id) {
                            104.
                        } else {
                            64.
                        };
                        size(px(600.), px(row_height))
                    })
                    .collect::<Vec<_>>(),
            );
            // Keep scrollbar geometry based on every estimated row while the virtual list mounts only visible rows.
            let content_height = item_sizes
                .iter()
                .fold(px(0.), |height, item| height + item.height);
            let feed_indices = Rc::new(feed_indices);
            let feeds = v_virtual_list(
                cx.entity().clone(),
                "settings-added-feeds-list",
                item_sizes,
                move |this, range, _window, cx| {
                    range
                        .map(|row| {
                            let index = feed_indices[row];
                            this.sidebar
                                .read(cx)
                                .feeds
                                .get(index)
                                .cloned()
                                .map(|feed| {
                                    this.settings
                                        .render_added_feed_row(this, &feed, index + 1, cx)
                                        .into_any_element()
                                })
                                .unwrap_or_else(|| div().into_any_element())
                        })
                        .collect::<Vec<_>>()
                },
            )
            .track_scroll(&self.added_feeds_scroll)
            .size_full();
            div()
                .w_full()
                .h(px(360.))
                .min_h_0()
                .mt_3()
                .mb_2()
                .overflow_hidden()
                .p_2()
                .border_1()
                .border_color(cx.theme().border)
                .rounded(cx.theme().radius)
                .child(
                    div()
                        .id("settings-added-feeds-scroll")
                        .size_full()
                        .min_h_0()
                        .pr_4()
                        .relative()
                        .overflow_hidden()
                        .on_scroll_wheel(|_, _, cx| cx.stop_propagation())
                        .child(feeds)
                        .child(
                            Scrollbar::vertical(&self.added_feeds_scroll)
                                .scroll_size(size(px(600.), content_height)),
                        ),
                )
                .into_any_element()
        };
        v_flex()
            .child(self.settings_heading(owner, "General"))
            .child(div().pb_1().font_semibold().child(owner.t("Language")))
            .child(div().text_sm().text_color(cx.theme().muted_foreground).child(owner.t("Choose the language used throughout the app.")))
            .child(
                div()
                    .pt_3()
                    .w(px(260.))
                    .child(
                        Select::new(&self.language_select)
                            .small()
                            .placeholder(owner.t("Language")),
                    ),
            )
            .child(div().my_6().border_t_1().border_color(cx.theme().border))
            .child(div().pb_2().font_semibold().child(owner.t("Subscription source")))
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(owner.t(
                        "Choose local subscriptions or connect a provider. Each source keeps its own feeds and reading state.",
                    )),
            )
            .child(div().pt_4().pb_2().text_sm().child(owner.t("Mode / Provider")))
            .child(
                div().w(px(260.)).child(
                    Select::new(&self.upstream_select)
                        .small()
                        .placeholder(owner.t("Mode / Provider")),
                ),
            )
            .when(self.library_source.provider().is_some(), |view| {
                view.when_some(active_provider_settings, |view, settings| {
                    view.child(
                        div()
                            .pt_3()
                            .text_sm()
                            .text_color(cx.theme().primary)
                            .child(i18n::format(
                                owner.preferences.language,
                                "Connected: {}",
                                &settings.endpoint,
                            )),
                    )
                })
                .child(div().pt_4().pb_2().text_sm().child(owner.t("Server URL")))
                .child(text_input(&self.provider_url_input))
                .when(self.library_source == LibrarySource::FreshRss, |view| {
                    view.child(
                        div()
                            .pt_2()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(owner.t(
                                "Connects through the Google Reader API.",
                            )),
                    )
                })
                .when(self.library_source == LibrarySource::FreshRss, |view| view
                    .child(div().pt_4().pb_2().text_sm().child(owner.t("Username")))
                    .child(text_input(&self.provider_username_input)))
                .child(div().pt_4().pb_2().text_sm().child(owner.t(if self.library_source == LibrarySource::Miniflux { "API Token" } else { "API Password" })))
                .child(text_input(&self.provider_secret_input).mask_toggle())
                .child(
                    h_flex()
                        .gap_2()
                        .pt_4()
                        .child(
                            Button::new("connect-miniflux").small()
                                .primary()
                                .label(owner.t(if active_provider_settings.is_some() {
                                    "Update connection"
                                } else {
                                    "Connect and sync"
                                }))
                                .loading(self.is_connecting)
                                .on_click(cx.listener(|this, _, _, cx| this.connect_provider(cx))),
                        )
                        .when(active_provider_settings.is_some(), |row| {
                            row.child(
                                Button::new("disconnect-provider").small()
                                    .ghost()
                                    .disabled(self.is_connecting)
                                    .label(owner.t("Disconnect"))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.disconnect_provider(cx)
                                    })),
                            )
                        }),
                )
            })
            .when(self.library_source == LibrarySource::Local, |view| {
                view.child(
                    div()
                        .pt_4()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(owner.t("Feeds and articles stay on this device and refresh directly from their RSS sources.")),
                )
            })
            .child(div().my_6().border_t_1().border_color(cx.theme().border))
            .child(div().pb_2().font_semibold().child(owner.t("Feed management")))
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(i18n::format(
                        owner.preferences.language,
                        "{} feeds in this source. Additions and removals apply only here.",
                        sidebar.feeds.len(),
                    )),
            )
            .child(div().pt_4().child(text_input(&self.add_input)))
            .child(
                h_flex()
                    .gap_2()
                    .pt_2()
                    .child(
                        Button::new("settings-add-feed").small()
                            .primary()
                            .icon(bundled_icon(BundledIcon::Plus))
                            .label(owner.t("Add feed"))
                            .on_click(cx.listener(|this, _, window, cx| this.add_feed(window, cx))),
                    )
                    .child(
                        Button::new("settings-refresh").small()
                            .ghost()
                            .icon(bundled_icon(BundledIcon::Refresh))
                            .label(owner.t("Sync now"))
                            .loading(sidebar.is_refreshing)
                            .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .pt_3()
                    .child(
                        Button::new("settings-import").small()
                            .ghost()
                            .label(owner.t("Import OPML"))
                            .on_click(cx.listener(|this, _, _, cx| this.import_opml(cx))),
                    )
                    .child(
                        Button::new("settings-export").small()
                            .ghost()
                            .label(owner.t("Export OPML"))
                            .on_click(cx.listener(|this, _, _, cx| this.export_opml(cx))),
                    ),
            )
            .child(div().my_6().border_t_1().border_color(cx.theme().border))
            .child(div().font_semibold().child(owner.t("Added feeds")))
            .child(
                div()
                    .pt_1()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(owner.t(
                        "Use Edit on a feed to change its details and title translation.",
                    )),
            )
            .child(div().pt_3().child(text_input(&self.added_feeds_search_input)))
            .child(feeds)
            .child(div().my_6().border_t_1().border_color(cx.theme().border))
            .child(div().pb_2().font_semibold().child(owner.t("Reset settings")))
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(owner.t(
                        "Restore interface and translation settings, including saved translation keys, to defaults. Feeds, articles and Provider connections are kept.",
                    )),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .pt_3()
                    .when(!self.pending_reset_settings, |row| {
                        row.child(
                            Button::new("reset-settings")
                                .small()
                                .ghost()
                                .label(owner.t("Reset settings"))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.settings.pending_reset_settings = true;
                                    cx.notify();
                                })),
                        )
                    })
                    .when(self.pending_reset_settings, |row| {
                        row.child(
                            div()
                                .flex_1()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child(owner.t("Reset app settings now?")),
                        )
                        .child(
                            Button::new("confirm-reset-settings")
                                .small()
                                .secondary()
                                .label(owner.t("Restore defaults"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.reset_settings(window, cx)
                                })),
                        )
                        .child(
                            Button::new("cancel-reset-settings")
                                .small()
                                .ghost()
                                .label(owner.t("Cancel"))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.settings.pending_reset_settings = false;
                                    cx.notify();
                                })),
                        )
                    }),
            )
    }

    fn render_added_feed_row(
        &self,
        owner: &ReaderWindow,
        feed: &panda_core::Feed,
        number: usize,
        cx: &mut Context<ReaderWindow>,
    ) -> impl IntoElement {
        let id = feed.id;
        let row = h_flex()
            .w_full()
            .items_center()
            .justify_between()
            .gap_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .w(px(20.))
                    .text_right()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(number.to_string()),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .child(div().text_sm().child(feed.title.clone()))
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(i18n::format(
                                owner.preferences.language,
                                "{} unread",
                                feed.unread,
                            )),
                    ),
            )
            .child(
                h_flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new(("settings-edit-feed", id as u64))
                            .small()
                            .ghost()
                            .label(owner.t("Edit"))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.begin_edit_feed(id, window, cx);
                            })),
                    )
                    .child(
                        Button::new(("settings-remove-feed", id as u64))
                            .small()
                            .ghost()
                            .label(owner.t("Remove"))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.sidebar.update(cx, |sidebar, cx| {
                                    sidebar.pending_remove_feed = Some(id);
                                    cx.notify();
                                });
                                cx.notify();
                            })),
                    ),
            );
        v_flex()
            .w_full()
            .gap_2()
            .child(row)
            .when(owner.sidebar.read(cx).pending_remove_feed == Some(id), |view| {
                view.child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(owner.t("Remove this feed and its cached articles from the selected source?")),
                        )
                        .child(
                            Button::new(("confirm-remove-feed", id as u64)).small()
                                .secondary()
                                .label(owner.t("Confirm removal"))
                                .on_click(cx.listener(move |this, _, _, cx| this.remove_feed(id, cx))),
                        )
                        .child(
                            Button::new(("cancel-remove-feed", id as u64)).small()
                                .ghost()
                                .label(owner.t("Cancel"))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.sidebar.update(cx, |sidebar, cx| {
                                        sidebar.pending_remove_feed = None;
                                        cx.notify();
                                    });
                                    cx.notify();
                                })),
                        ),
                )
            })
    }
}
