use super::*;

impl Store {
    #[cfg(test)]
    pub async fn translate(
        &mut self,
        article_id: i64,
        target_lang: &str,
        translator: &panda_translate::AnyTranslator,
    ) -> anyhow::Result<Article> {
        let mut article = self.article(article_id)?;
        let source = article
            .extracted_html
            .as_deref()
            .filter(|html| !html.trim().is_empty())
            .or(article.source_html.as_deref())
            .unwrap_or(article.content_html.as_str())
            .to_owned();
        let url = article.url.as_deref().unwrap_or_default();
        let pipeline = "store-test-canonical-v1";
        article.canonical = Some(panda_core::CanonicalArticle {
            html: panda_core::CanonicalHtml(std::sync::Arc::from(source.as_str())),
            revision: Self::canonical_revision(&source, url, &article.summary.title, pipeline),
            pipeline_revision: std::sync::Arc::from(pipeline),
        });
        self.translate_with_source(article_id, target_lang, &source, translator)
            .await
    }

    #[cfg(test)]
    pub async fn translate_with_source(
        &mut self,
        article_id: i64,
        target_lang: &str,
        source: &str,
        translator: &panda_translate::AnyTranslator,
    ) -> anyhow::Result<Article> {
        let article = self.article(article_id)?;
        let title = article.summary.title.trim();
        let source_hash = panda_translate::translation_cache_hash(source, title, translator.id());
        let has_body = article
            .translated_html
            .as_deref()
            .is_some_and(|html| !html.trim().is_empty());
        let lang_ok = article
            .translated_lang
            .as_deref()
            .is_some_and(|lang| lang == target_lang);
        let hash_ok = article
            .translation_source_hash
            .as_deref()
            .is_some_and(|hash| hash == source_hash);
        if lang_ok && has_body && hash_ok {
            if let Some(translated_title) = article
                .translated_title
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                && article.summary.feed_auto_translate_titles
            {
                self.save_auto_translated_title(
                    article_id,
                    &article.summary.title,
                    translated_title,
                    target_lang,
                    &panda_translate::title_source_hash(&article.summary.title),
                )?;
                return self.article(article_id);
            }
            return Ok(article);
        }
        if source.trim().is_empty() && title.is_empty() {
            anyhow::bail!("Nothing to translate");
        }
        let result = translator
            .translate(panda_translate::TranslateRequest {
                html: source.to_owned(),
                title: (!title.is_empty()).then(|| title.to_owned()),
                target_lang: target_lang.to_owned(),
            })
            .await?;
        self.connection.execute(
            "UPDATE articles SET translated_html=?1, translated_title=?2, translated_lang=?3, translation_source_hash=?4,
                content_revision=content_revision+1 WHERE id=?5",
            params![result.html, result.title, target_lang, source_hash, article_id],
        )?;
        sync_article_fts(&self.connection, article_id)?;
        if let Some(translated_title) = result
            .title
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            && article.summary.feed_auto_translate_titles
        {
            self.save_auto_translated_title(
                article_id,
                &article.summary.title,
                translated_title,
                target_lang,
                &panda_translate::title_source_hash(&article.summary.title),
            )?;
        }
        self.article(article_id)
    }

    pub fn persist_translation(
        &self,
        article_id: i64,
        expected_content_revision: i64,
        target_lang: &str,
        source_hash: &str,
        translated_html: &str,
        translated_title: Option<&str>,
    ) -> anyhow::Result<()> {
        let updated = self.connection.execute(
            "UPDATE articles SET translated_html=?1, translated_title=?2, translated_lang=?3, translation_source_hash=?4,
                content_revision=content_revision+1 WHERE id=?5 AND content_revision=?6",
            params![translated_html, translated_title, target_lang, source_hash, article_id, expected_content_revision],
        )?;
        if updated == 0 {
            anyhow::bail!("Article content changed while translation was running");
        }
        sync_article_fts(&self.connection, article_id)?;
        Ok(())
    }

    pub fn translation_segments(
        &self,
        article_id: i64,
        target_lang: &str,
        backend_id: &str,
        prompt_revision: &str,
        context_hash: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut statement = self.connection.prepare(
            "SELECT segment_id,translated_html FROM article_translation_segments
             WHERE workspace=?1 AND article_id=?2 AND target_lang=?3 AND backend_id=?4 AND prompt_revision=?5 AND context_hash=?6",
        )?;
        let rows = statement.query_map(
            params![
                self.workspace,
                article_id,
                target_lang,
                backend_id,
                prompt_revision,
                context_hash
            ],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )?;
        rows.collect::<Result<HashMap<_, _>, _>>()
            .map_err(Into::into)
    }

    pub fn save_translation_segment(
        &self,
        article_id: i64,
        target_lang: &str,
        backend_id: &str,
        prompt_revision: &str,
        context_hash: &str,
        segment_id: &str,
        source_hash: &str,
        translated_html: &str,
    ) -> anyhow::Result<()> {
        self.connection.execute(
            "INSERT INTO article_translation_segments(workspace,article_id,target_lang,backend_id,prompt_revision,context_hash,segment_id,source_hash,translated_html)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)
             ON CONFLICT(workspace,article_id,target_lang,backend_id,prompt_revision,segment_id,source_hash)
             DO UPDATE SET context_hash=excluded.context_hash,translated_html=excluded.translated_html",
            params![
                self.workspace,
                article_id,
                target_lang,
                backend_id,
                prompt_revision,
                context_hash,
                segment_id,
                source_hash,
                translated_html
            ],
        )?;
        Ok(())
    }

    pub fn prune_translation_segments(
        &self,
        article_id: i64,
        target_lang: &str,
        backend_id: &str,
        prompt_revision: &str,
        context_hash: &str,
    ) -> anyhow::Result<()> {
        self.connection.execute(
            "DELETE FROM article_translation_segments
             WHERE workspace=?1 AND article_id=?2
               AND NOT (target_lang=?3 AND backend_id=?4 AND prompt_revision=?5 AND context_hash=?6)",
            params![
                self.workspace,
                article_id,
                target_lang,
                backend_id,
                prompt_revision,
                context_hash
            ],
        )?;
        Ok(())
    }

    pub fn save_auto_translated_title(
        &self,
        id: i64,
        source_title: &str,
        title: &str,
        lang: &str,
        source_hash: &str,
    ) -> anyhow::Result<()> {
        self.connection.execute("UPDATE articles SET auto_translated_title=?1,auto_translated_title_lang=?2,auto_translated_title_source_hash=?3 WHERE id=?4 AND articles.title=?5 AND EXISTS(SELECT 1 FROM feeds WHERE feeds.id=articles.feed_id AND feeds.workspace=?6)",params![title,lang,source_hash,id,source_title,self.workspace])?;
        Ok(())
    }

    pub fn record_translation_usage(
        &self,
        day: &str,
        provider: &str,
        requests: u64,
        characters: u64,
    ) -> anyhow::Result<()> {
        self.record_translation_usage_with_tokens(day, provider, requests, characters, 0, 0)
    }

    pub fn record_translation_usage_with_tokens(
        &self,
        day: &str,
        provider: &str,
        requests: u64,
        characters: u64,
        input_tokens: u64,
        output_tokens: u64,
    ) -> anyhow::Result<()> {
        self.connection.execute(
            "INSERT INTO translation_usage(day,provider,requests,characters,input_tokens,output_tokens) VALUES(?1,?2,?3,?4,?5,?6)
             ON CONFLICT(day,provider) DO UPDATE SET requests=requests+excluded.requests,characters=characters+excluded.characters,input_tokens=input_tokens+excluded.input_tokens,output_tokens=output_tokens+excluded.output_tokens",
            params![day,provider,requests,characters,input_tokens,output_tokens],
        )?;
        Ok(())
    }

    pub fn translation_usage(&self) -> anyhow::Result<Vec<panda_core::TranslationUsage>> {
        let mut stmt=self.connection.prepare("SELECT day,provider,requests,characters,input_tokens,output_tokens FROM translation_usage ORDER BY day DESC,provider")?;
        Ok(stmt
            .query_map([], |r| {
                Ok(panda_core::TranslationUsage {
                    day: r.get(0)?,
                    provider: r.get(1)?,
                    requests: r.get::<_, u64>(2)?,
                    characters: r.get::<_, u64>(3)?,
                    input_tokens: r.get::<_, u64>(4)?,
                    output_tokens: r.get::<_, u64>(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?)
    }
}
