use super::*;

impl Store {
    pub fn mark(&self, id: i64, field: MarkField, value: bool) -> anyhow::Result<bool> {
        let column = match field {
            MarkField::Read => "is_read",
            MarkField::Starred => "is_starred",
            MarkField::Later => "read_later",
        };
        let changed = self.connection.execute(
            &format!(
                "UPDATE articles SET {column}=?1 WHERE id=?2 AND {column} IS NOT ?1
                AND EXISTS(SELECT 1 FROM feeds f WHERE f.id=articles.feed_id AND f.workspace=?3)"
            ),
            params![value, id, self.workspace],
        )? > 0;
        if changed
            && !matches!(field, MarkField::Later)
            && let Some(remote_id) = self.remote_entry_id(id)?
        {
            self.connection.execute(
                "INSERT INTO pending_remote_marks(workspace,remote_id,field,value,revision) VALUES(?1,?2,?3,?4,1)
                 ON CONFLICT(workspace,remote_id,field) DO UPDATE SET value=excluded.value,revision=pending_remote_marks.revision+1",
                params![self.workspace, remote_id, column, value],
            )?;
        }
        Ok(changed)
    }

    pub fn mark_all_read(&self, scope: Scope, search: &str) -> anyhow::Result<usize> {
        enum ScopeBind {
            None,
            FeedId(i64),
            Folder(String),
        }
        let (filter, bind) = match scope {
            Scope::All | Scope::Unread => ("1=1", ScopeBind::None),
            Scope::Starred => ("a.is_starred=1", ScopeBind::None),
            Scope::Later => ("a.read_later=1", ScopeBind::None),
            Scope::Feed(id) => ("a.feed_id=?", ScopeBind::FeedId(id)),
            Scope::Folder(name) => ("f.folder=?", ScopeBind::Folder(name)),
        };
        let search_query = fts_query(search);
        let search_filter = if search_query.is_empty() {
            ""
        } else {
            "AND a.id IN (SELECT rowid FROM article_fts WHERE article_fts MATCH ?)"
        };
        let sql = format!(
            "SELECT a.id, a.remote_id FROM articles a JOIN feeds f ON f.id=a.feed_id
             WHERE f.workspace=? AND a.is_read=0 AND {filter} {search_filter}"
        );
        // Keep the article updates and their pending provider marks in one transaction.
        // A folder can contain thousands of rows, and autocommitting each row is expensive.
        let transaction = self.connection.unchecked_transaction()?;
        let rows: Vec<(i64, Option<i64>)> = {
            let mut statement = transaction.prepare(&sql)?;
            let mut params: Vec<rusqlite::types::Value> = vec![self.workspace.clone().into()];
            match &bind {
                ScopeBind::FeedId(feed_id) => params.push((*feed_id).into()),
                ScopeBind::Folder(folder) => params.push(folder.clone().into()),
                ScopeBind::None => {}
            }
            if !search_query.is_empty() {
                params.push(search_query.into());
            }
            statement
                .query_map(rusqlite::params_from_iter(params), |row| {
                    Ok((row.get(0)?, row.get(1)?))
                })?
                .collect::<Result<Vec<_>, _>>()?
        };
        let count = rows.len();
        for (id, remote_id) in rows {
            transaction.execute("UPDATE articles SET is_read=1 WHERE id=?1", [id])?;
            if let Some(remote_id) = remote_id {
                transaction.execute(
                    "INSERT INTO pending_remote_marks(workspace,remote_id,field,value,revision) VALUES(?1,?2,'is_read',1,1)
                     ON CONFLICT(workspace,remote_id,field) DO UPDATE SET value=excluded.value,revision=pending_remote_marks.revision+1",
                    params![self.workspace, remote_id],
                )?;
            }
        }
        transaction.commit()?;
        Ok(count)
    }
}
