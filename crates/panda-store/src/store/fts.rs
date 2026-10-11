use super::*;

pub(super) fn sync_article_fts(connection: &Connection, article_id: i64) -> anyhow::Result<()> {
    let article = connection
        .query_row(
            "SELECT title,author,content_html,processed_html,translated_html FROM articles WHERE id=?1",
            [article_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            },
        )
        .optional()?;
    let Some((title, author, content_html, canonical_html, translated_html)) = article else {
        connection.execute("DELETE FROM article_fts WHERE rowid=?1", [article_id])?;
        return Ok(());
    };
    let source_html = canonical_html.unwrap_or(content_html);
    let content_text = plain_text(&source_html);
    let translated_text = translated_html
        .as_deref()
        .filter(|html| !html.trim().is_empty())
        .map(plain_text)
        .unwrap_or_default();
    connection.execute("DELETE FROM article_fts WHERE rowid=?1", [article_id])?;
    connection.execute(
        "INSERT INTO article_fts(rowid,title,author,content_text,translated_text) VALUES(?1,?2,?3,?4,?5)",
        params![
            article_id,
            cjk_token_text(&title),
            cjk_token_text(author.as_deref().unwrap_or_default()),
            cjk_token_text(&content_text),
            cjk_token_text(&translated_text),
        ],
    )?;
    Ok(())
}

pub(super) fn fts_query(search: &str) -> String {
    let normalized = cjk_token_text(search.trim());
    if normalized.is_empty() {
        String::new()
    } else {
        format!("\"{}\"", normalized.replace('"', "\"\""))
    }
}

// Separate adjacent Han characters for unicode61 so CJK searches can match
// short phrases instead of requiring the entire uninterrupted run to match.
fn cjk_token_text(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut previous_was_han = false;
    for character in value.chars() {
        let is_han = matches!(
            character as u32,
            0x3400..=0x4DBF
                | 0x4E00..=0x9FFF
                | 0xF900..=0xFAFF
                | 0x20000..=0x2FA1F
                | 0x30000..=0x323AF
        );
        if is_han && previous_was_han {
            output.push(' ');
        }
        output.push(character);
        previous_was_han = is_han;
    }
    output
}
