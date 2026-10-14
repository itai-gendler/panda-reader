use crate::app::preferences::LibrarySource;
use crate::services::favicon::{feed_host, local_favicon_path};
use crate::ui::components::letter_avatar;
use crate::ui::components::{BundledIcon, bundled_icon, preview_text, sidebar_font};
use gpui_kit::base::{StyledExt as _, v_virtual_list};
use gpui_kit::component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    menu::ContextMenuExt as _,
    scroll::Scrollbar,
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use panda_core::{Feed, Scope};
use std::rc::Rc;

use super::state::{Sidebar, SidebarEvent};

enum SidebarRow {
    Feed {
        feed_index: usize,
        nested: bool,
        has_error: bool,
    },
    Folder {
        name: String,
        unread: i64,
        collapsed: bool,
    },
}

impl SidebarRow {
    fn height(&self) -> f32 {
        match self {
            Self::Feed { has_error, .. } => {
                if *has_error {
                    44.
                } else {
                    30.
                }
            }
            Self::Folder { .. } => 30.,
        }
    }
}

#[derive(Clone, Debug)]
struct FolderDrag(String);

struct FolderDragPreview {
    name: SharedString,
}

impl Render for FolderDragPreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .px_2()
            .py_1()
            .rounded(cx.theme().radius)
            .bg(cx.theme().sidebar_accent)
            .text_color(cx.theme().sidebar_accent_foreground)
            .text_sm()
            .child(bundled_icon(BundledIcon::Folder).small())
            .child(self.name.clone())
    }
}

impl Sidebar {
    pub(in crate::ui) fn render_feed_row(
        icons_dir: &std::path::Path,
        event_entity: WeakEntity<Sidebar>,
        emit: crate::ui::context_menus::PaneMenuEmitter,
        language: crate::app::preferences::Language,
        local: bool,
        feed: &Feed,
        nested: bool,
        selected_scope: &Scope,
        cx: &mut Context<Sidebar>,
    ) -> impl IntoElement {
        let id = feed.id;
        let is_selected = selected_scope == &Scope::Feed(id);
        let menu_emit = emit;
        let event_for_feed = event_entity.clone();
        let menu_feed = feed.clone();
        let sidebar_bg = cx.theme().sidebar;
        let active_bg = cx.theme().sidebar_accent;
        h_flex()
            .id(("feed-row", id as u64))
            .w_full()
            .h(px(if feed.last_error.is_some() { 44. } else { 30. }))
            .items_center()
            .rounded(cx.theme().radius)
            .bg(if is_selected { active_bg } else { sidebar_bg })
            .hover(|style| style.bg(active_bg))
            .child(
                h_flex()
                    .id(("feed", id as u64))
                    .flex_1()
                    .min_w_0()
                    .items_center()
                    .gap_2()
                    .when(nested, |row| row.pl_5())
                    .px_2()
                    .py_1()
                    .cursor_pointer()
                    .child({
                        let avatar = letter_avatar(&feed.title);
                        let danger = feed.last_error.is_some();
                        let accent_bg = if danger {
                            cx.theme().danger.opacity(0.18)
                        } else {
                            cx.theme().accent.opacity(0.16)
                        };
                        let accent_fg = if danger {
                            cx.theme().danger
                        } else {
                            cx.theme().accent
                        };
                        let letter = {
                            let avatar = avatar.clone();
                            move || {
                                div()
                                    .size(px(20.))
                                    .rounded(px(4.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(accent_bg)
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(accent_fg)
                                    .child(avatar.clone())
                                    .into_any_element()
                            }
                        };
                        match feed_host(feed.site_url.as_deref(), &feed.feed_url)
                            .and_then(|host| local_favicon_path(icons_dir, &host))
                        {
                            Some(path) => {
                                let loading_avatar = avatar.clone();
                                img(path)
                                    .id(("feed-icon", id as u64))
                                    .size(px(20.))
                                    .rounded(px(4.))
                                    .overflow_hidden()
                                    .object_fit(ObjectFit::Cover)
                                    .with_fallback(letter)
                                    .with_loading(move || {
                                        div()
                                            .size(px(20.))
                                            .rounded(px(4.))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .bg(accent_bg)
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(accent_fg)
                                            .child(loading_avatar.clone())
                                            .into_any_element()
                                    })
                                    .into_any_element()
                            }
                            None => letter(),
                        }
                    })
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .overflow_hidden()
                                    .text_sm()
                                    .truncate()
                                    .child(feed.title.clone()),
                            )
                            .when_some(feed.last_error.clone(), |row, error| {
                                row.child(
                                    div()
                                        .overflow_hidden()
                                        .text_xs()
                                        .truncate()
                                        .text_color(cx.theme().danger)
                                        .child(preview_text(&error, 42)),
                                )
                            }),
                    )
                    .when(feed.unread > 0, |row| {
                        row.child(div().text_xs().child(feed.unread.to_string()))
                    })
                    .on_click(move |_, _, cx| {
                        let _ = event_for_feed.update(cx, |_, cx| {
                            cx.emit(SidebarEvent::SelectScope(Scope::Feed(id)));
                        });
                    })
                    .context_menu(move |menu, _, _| {
                        crate::ui::context_menus::feed_context_menu(
                            menu, &menu_feed, &menu_emit, language, local,
                        )
                    }),
            )
    }

    pub(in crate::ui) fn render_folder_row(
        event_entity: WeakEntity<Sidebar>,
        emit: crate::ui::context_menus::PaneMenuEmitter,
        folder: &str,
        unread: i64,
        folder_collapsed: bool,
        selected: bool,
        selected_icon: &str,
        language: crate::app::preferences::Language,
        cx: &mut Context<Sidebar>,
    ) -> impl IntoElement {
        let folder_for_scope = folder.to_owned();
        let folder_for_toggle = folder.to_owned();
        let folder_for_drop = folder.to_owned();
        let folder_for_menu = folder.to_owned();
        let selected_icon = selected_icon.to_owned();
        let folder_icon = crate::ui::context_menus::folder_icon_element(&selected_icon);
        let folder_key = {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            folder.hash(&mut hasher);
            hasher.finish()
        };
        let event_for_folder = event_entity.clone();
        let emit_for_drop = emit.clone();
        let emit_for_toggle = emit.clone();
        let menu_emit = emit;
        h_flex()
            .id(("folder-row", folder_key))
            .w_full()
            .h(px(30.))
            .items_center()
            .rounded(cx.theme().radius)
            .bg(if selected {
                cx.theme().sidebar_accent
            } else {
                cx.theme().sidebar
            })
            .hover(|style| style.bg(cx.theme().sidebar_accent))
            .text_color(if selected {
                cx.theme().sidebar_accent_foreground
            } else {
                cx.theme().sidebar_foreground
            })
            .cursor_pointer()
            .on_drag(FolderDrag(folder.to_owned()), |drag, _, _, cx| {
                cx.new(|_| FolderDragPreview {
                    name: SharedString::from(drag.0.clone()),
                })
            })
            .can_drop(|drag, _, _| drag.is::<FolderDrag>())
            .on_drop(move |drag: &FolderDrag, _, cx| {
                emit_for_drop(
                    crate::ui::context_menus::PaneMenuEvent::ReorderFolderBefore {
                        dragged: drag.0.clone(),
                        target: folder_for_drop.clone(),
                    },
                    cx,
                );
            })
            .context_menu(move |menu, window, cx| {
                crate::ui::context_menus::folder_context_menu(
                    menu,
                    window,
                    cx,
                    &folder_for_menu,
                    &menu_emit,
                    language,
                    &selected_icon,
                )
            })
            .child(
                div()
                    .id(("folder-toggle", folder_key))
                    .flex()
                    .items_center()
                    .justify_center()
                    .px_1()
                    .py_1()
                    .cursor_pointer()
                    .child(
                        Icon::new(if folder_collapsed {
                            IconName::ChevronRight
                        } else {
                            IconName::ChevronDown
                        })
                        .small(),
                    )
                    .on_click(move |_, _, cx| {
                        emit_for_toggle(
                            crate::ui::context_menus::PaneMenuEvent::ToggleFolderCollapsed(
                                folder_for_toggle.clone(),
                            ),
                            cx,
                        );
                    }),
            )
            .child(
                h_flex()
                    .id(("folder", folder_key))
                    .flex_1()
                    .min_w_0()
                    .items_center()
                    .gap_2()
                    .pr_2()
                    .py_1()
                    .cursor_pointer()
                    .child(folder_icon)
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .text_sm()
                            .child(folder.to_owned()),
                    )
                    .when(unread > 0, |row| {
                        row.child(div().text_xs().child(unread.to_string()))
                    })
                    .on_click(move |_, _, cx| {
                        let folder = folder_for_scope.clone();
                        let _ = event_for_folder.update(cx, |_, cx| {
                            cx.emit(SidebarEvent::SelectScope(Scope::Folder(folder)));
                        });
                    }),
            )
    }

    pub(in crate::ui) fn render_sidebar(&mut self, cx: &mut Context<Sidebar>) -> impl IntoElement {
        let all_feeds = &self.feeds;
        let scroll = self.scroll.clone();
        let is_refreshing = self.is_refreshing;
        let language = self.language;
        let selected_scope = self.selected_scope.clone();
        let folder_icons = self.folder_icons.clone();
        let sidebar_bg = cx.theme().sidebar;
        let sidebar_text = cx.theme().sidebar_foreground;
        let mut folders: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();
        for (index, feed) in all_feeds.iter().enumerate() {
            folders
                .entry(feed.folder.clone().unwrap_or_default())
                .or_default()
                .push(index);
        }
        for feeds in folders.values_mut() {
            feeds.sort_by_cached_key(|index| all_feeds[*index].title.to_lowercase());
        }
        let mut folder_names = folders
            .keys()
            .filter(|name| !name.is_empty())
            .cloned()
            .collect::<Vec<_>>();
        folder_names.sort_by_key(|name| name.to_lowercase());
        let mut folder_order = self
            .folder_order
            .iter()
            .filter(|name| folder_names.iter().any(|folder| folder == *name))
            .cloned()
            .collect::<Vec<_>>();
        for name in &folder_names {
            if !folder_order.iter().any(|existing| existing == name) {
                folder_order.push(name.clone());
            }
        }

        let mut rows = Vec::with_capacity(all_feeds.len() + folder_order.len());
        if let Some(root_feeds) = folders.remove("") {
            rows.extend(root_feeds.into_iter().map(|feed_index| SidebarRow::Feed {
                feed_index,
                nested: false,
                has_error: all_feeds[feed_index].last_error.is_some(),
            }));
        }
        for folder in folder_order {
            let Some(feeds) = folders.remove(&folder) else {
                continue;
            };
            let collapsed = self.collapsed_folders.iter().any(|name| name == &folder);
            let unread = feeds.iter().map(|index| all_feeds[*index].unread).sum();
            rows.push(SidebarRow::Folder {
                name: folder,
                unread,
                collapsed,
            });
            if !collapsed {
                rows.extend(feeds.into_iter().map(|feed_index| SidebarRow::Feed {
                    feed_index,
                    nested: true,
                    has_error: all_feeds[feed_index].last_error.is_some(),
                }));
            }
        }
        let item_sizes = Rc::new(
            rows.iter()
                .map(|row| size(px(228.), px(row.height() + 2.)))
                .collect::<Vec<_>>(),
        );
        let rows = Rc::new(rows);
        let scroll = scroll.clone();
        let event_entity = cx.entity().downgrade();
        let list_event_entity = event_entity.clone();
        let list_menu_emit = crate::ui::context_menus::pane_menu_emitter(event_entity.clone());
        let list_owner = cx.entity().clone();
        let feed_list = v_virtual_list(
            list_owner,
            "feed-list",
            item_sizes,
            move |this, range, _window, cx| {
                range
                    .map(|index| match &rows[index] {
                        SidebarRow::Feed {
                            feed_index, nested, ..
                        } => {
                            let feed = this.feeds.get(*feed_index).cloned();
                            let icons_dir = this.icons_dir.clone();
                            let selected_scope = this.selected_scope.clone();
                            feed.map(|feed| {
                                Sidebar::render_feed_row(
                                    &icons_dir,
                                    list_event_entity.clone(),
                                    list_menu_emit.clone(),
                                    this.language,
                                    this.library_source == LibrarySource::Local,
                                    &feed,
                                    *nested,
                                    &selected_scope,
                                    cx,
                                )
                                .into_any_element()
                            })
                            .unwrap_or_else(|| div().into_any_element())
                        }
                        SidebarRow::Folder {
                            name,
                            unread,
                            collapsed,
                        } => Sidebar::render_folder_row(
                            list_event_entity.clone(),
                            list_menu_emit.clone(),
                            name,
                            *unread,
                            *collapsed,
                            matches!(&this.selected_scope, Scope::Folder(selected) if selected == name),
                            folder_icons.get(name).map(String::as_str).unwrap_or("folder"),
                            this.language,
                            cx,
                        )
                        .into_any_element(),
                    })
                    .collect::<Vec<_>>()
            },
        )
        .track_scroll(&scroll)
        .size_full();
        v_flex()
            .w(px(228.))
            .h_full()
            .font(sidebar_font())
            .bg(sidebar_bg)
            .text_color(sidebar_text)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .child(div().h(px(8.)))
            .child(
                v_flex()
                    .gap_0p5()
                    .px_2()
                    .child(Sidebar::scope_row(
                        language,
                        &selected_scope,
                        event_entity.clone(),
                        "scope-all",
                        "All Articles",
                        IconName::BookOpen,
                        Scope::All,
                        cx,
                    ))
                    .child(Sidebar::scope_row(
                        language,
                        &selected_scope,
                        event_entity.clone(),
                        "scope-unread",
                        "Unread",
                        IconName::Inbox,
                        Scope::Unread,
                        cx,
                    ))
                    .child(Sidebar::scope_row(
                        language,
                        &selected_scope,
                        event_entity.clone(),
                        "scope-starred",
                        "Starred",
                        IconName::Star,
                        Scope::Starred,
                        cx,
                    ))
                    .child(Sidebar::scope_row(
                        language,
                        &selected_scope,
                        event_entity.clone(),
                        "scope-later",
                        "Read Later",
                        IconName::Inbox,
                        Scope::Later,
                        cx,
                    )),
            )
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .pt_4()
                    .pb_1()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(crate::ui::i18n::text(language, "Feeds"))
                    .child(
                        Button::new("refresh")
                            .small()
                            .ghost()
                            .icon(bundled_icon(BundledIcon::Refresh))
                            .tooltip(crate::ui::i18n::text(
                                language,
                                "Refresh feeds (Shift-click to force refresh)",
                            ))
                            .loading(is_refreshing)
                            .on_click(move |event: &ClickEvent, _, cx| {
                                let force = event.modifiers().shift;
                                let _ = event_entity.update(cx, |_, cx| {
                                    cx.emit(SidebarEvent::Refresh { force });
                                });
                            }),
                    ),
            )
            .child(
                div()
                    .id("feed-list-scroll")
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .px_2()
                    .child(feed_list)
                    .child(Scrollbar::vertical(&scroll)),
            )
    }

    pub(in crate::ui) fn scope_row(
        language: crate::app::preferences::Language,
        selected_scope: &Scope,
        event_entity: WeakEntity<Sidebar>,
        id: &'static str,
        label: &'static str,
        icon: IconName,
        scope: Scope,
        cx: &mut Context<Sidebar>,
    ) -> impl IntoElement {
        let selected = selected_scope == &scope;
        let scope_for_click = scope.clone();
        h_flex()
            .id(id)
            .w_full()
            .items_center()
            .gap_2()
            .px_2()
            .py_1()
            .rounded(cx.theme().radius)
            .cursor_pointer()
            .hover(|style| style.bg(cx.theme().sidebar_accent))
            .bg(if selected {
                cx.theme().sidebar_accent
            } else {
                cx.theme().sidebar
            })
            .text_color(if selected {
                cx.theme().sidebar_accent_foreground
            } else {
                cx.theme().sidebar_foreground
            })
            .child(match &scope {
                Scope::All => bundled_icon(BundledIcon::ListFlat).small(),
                Scope::Starred => bundled_icon(BundledIcon::Star).small(),
                Scope::Later => bundled_icon(BundledIcon::Bookmark).small(),
                Scope::Folder(_) => Icon::new(IconName::FolderClosed).small(),
                _ => Icon::new(icon).small(),
            })
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .child(crate::ui::i18n::text(language, label)),
            )
            .on_click(move |_, _, cx| {
                let scope = scope_for_click.clone();
                let _ = event_entity.update(cx, |_, cx| {
                    cx.emit(SidebarEvent::SelectScope(scope));
                });
            })
    }
}

impl Render for Sidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_sidebar(cx)
    }
}
