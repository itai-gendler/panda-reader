use super::*;

impl Store {
    #[cfg(test)]
    pub async fn import_opml(&mut self, source: &str) -> anyhow::Result<usize> {
        let document = OPML::from_str(source)?;
        let mut entries = Vec::new();
        for outline in &document.body.outlines {
            collect_outlines(outline, None, &mut entries);
        }
        let mut added = 0;
        for (url, title, folder) in entries {
            let url = match normalize_http_url(&url) {
                Ok(url) => url,
                Err(_) => continue,
            };
            if self
                .connection
                .query_row(
                    "SELECT 1 FROM feeds WHERE feed_url=?1 AND workspace=?2",
                    params![url, self.workspace],
                    |_| Ok(()),
                )
                .optional()?
                .is_some()
            {
                continue;
            }
            match self.fetch_feed(&url, None, None).await {
                Ok((Some(feed), etag, modified)) => {
                    self.save_feed(&url, feed, None, etag, modified)?;
                    if let Some(folder) = folder {
                        self.connection.execute(
                            "UPDATE feeds SET folder=?1 WHERE feed_url=?2 AND workspace=?3",
                            params![folder, url, self.workspace],
                        )?;
                    }
                    if let Some(title) = title.filter(|name| !name.trim().is_empty()) {
                        self.connection.execute(
                            "UPDATE feeds SET title=?1 WHERE feed_url=?2 AND workspace=?3",
                            params![title, url, self.workspace],
                        )?;
                    }
                    added += 1;
                }
                Ok((None, _, _)) => {}
                Err(error) => eprintln!("skipping OPML feed {url}: {error}"),
            }
        }
        Ok(added)
    }

    pub fn export_opml(&self) -> anyhow::Result<String> {
        let mut statement = self.connection.prepare("SELECT COALESCE(custom_title,title),feed_url,folder FROM feeds WHERE workspace=?1 ORDER BY COALESCE(folder,''),COALESCE(custom_title,title) COLLATE NOCASE")?;
        let rows = statement
            .query_map([&self.workspace], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        let mut groups: BTreeMap<Option<String>, Vec<Outline>> = BTreeMap::new();
        for (title, url, folder) in rows {
            groups.entry(folder).or_default().push(Outline {
                text: title,
                xml_url: Some(url),
                r#type: Some("rss".into()),
                ..Outline::default()
            });
        }
        let mut document = OPML {
            head: Some(Head {
                title: Some("Panda Reader subscriptions".into()),
                ..Head::default()
            }),
            ..OPML::default()
        };
        for (folder, outlines) in groups {
            if let Some(folder) = folder.filter(|s| !s.trim().is_empty()) {
                document.body.outlines.push(Outline {
                    text: folder,
                    outlines,
                    ..Outline::default()
                });
            } else {
                document.body.outlines.extend(outlines);
            }
        }
        Ok(document.to_string()?)
    }
}
