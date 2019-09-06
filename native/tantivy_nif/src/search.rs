extern crate tantivy;

use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser};
use tantivy::schema::*;
use tantivy::{Index, IndexWriter};
use tantivy::tokenizer::NgramTokenizer;

pub struct Searcher {
    schema: Schema,
    index: Index,
    index_writer: IndexWriter,
}

impl Searcher {
    pub fn new() -> Searcher {
        let mut schema_builder = Schema::builder();
        let ngrams_indexing = TextFieldIndexing::default()
            .set_tokenizer("ngram3")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let text_options = TextOptions::default()
            .set_indexing_options(ngrams_indexing.clone())
            .set_stored();
        let _title = schema_builder.add_text_field("title", text_options);

        let body_options = TextOptions::default()
            .set_indexing_options(ngrams_indexing);
        let _body = schema_builder.add_text_field("body", body_options);

        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());
        index.tokenizers()
            .register("ngram3", NgramTokenizer::new(3, 3, false));
        let index_writer = index.writer(50_000_000).unwrap();

        Searcher {
            schema,
            index,
            index_writer,
        }
    }

    pub fn add_entry(&mut self, title: String, body: String) -> tantivy::Result<()> {
        let schema_title = self.schema.get_field("title").unwrap();
        let schema_body = self.schema.get_field("body").unwrap();
        self.index_writer.add_document(doc!(
            schema_title => title,
            schema_body => body,
        ));

        self.index_writer.commit()?;

        Ok(())
    }

    pub fn search(&self, query: String) -> tantivy::Result<Vec<Document>> {
        let searcher = self.index.reader()?.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.schema.get_field("title").unwrap(),
                self.schema.get_field("body").unwrap(),
            ],
        );

        let query = query_parser.parse_query(&query)?;
        let top_docs = searcher.search(&*query, &TopDocs::with_limit(10))?;

        let docs = top_docs
            .iter()
            .map(|(_score, doc_address)| searcher.doc(*doc_address).unwrap())
            .collect();

        Ok(docs)
    }
}
