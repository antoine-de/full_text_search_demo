extern crate tantivy;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::Tokenizer;
use tantivy::tokenizer::{LowerCaser, NgramTokenizer};
use tantivy::{Index, IndexWriter};

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

        let int_options = IntOptions::default()
            .set_stored();
        let _id = schema_builder.add_u64_field("id", int_options);

        let body_options = TextOptions::default()
            .set_indexing_options(ngrams_indexing);
        let _body = schema_builder.add_text_field("body", body_options);

        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());
        index.tokenizers().register(
            "ngram3",
            NgramTokenizer::new(3, 3, false).filter(LowerCaser),
        );
        let index_writer = index.writer(500_000_000).unwrap();

        Searcher {
            schema,
            index,
            index_writer,
        }
    }

    pub fn add_entry(&mut self, id: String, body: String) -> tantivy::Result<()> {
        // {

        // let ngram = self.index.tokenizers().get("ngram3").unwrap();
        // let other = body.clone();
        // let mut stream = ngram.token_stream(&other);

        // loop {
        //     let s = stream.next();
        //     match s {
        //         None => break,
        //         Some(s) => println!("token: {:?}", s)
        //     }
        // }
        // }

        // println!("===========");
        let schema_id = self.schema.get_field("id").unwrap();
        let schema_body = self.schema.get_field("body").unwrap();

        self.index_writer.add_document(doc!(
            schema_id => id,
            schema_body => body,
        ));

        self.index_writer.commit()?;

        println!("adding elt done");
        Ok(())
    }

    pub fn add_entries(&mut self, docs: Vec<(u64, String)>) -> tantivy::Result<()> {
        println!("adding entries {}", docs.len());

        let schema_id = self.schema.get_field("id").unwrap();
        let schema_body = self.schema.get_field("body").unwrap();

        for d in docs {
            self.index_writer.add_document(doc!(
                schema_id => d.0,
                schema_body => d.1,
            ));
        }

        self.index_writer.commit()?;

        println!("adding done");
        Ok(())
    }

    pub fn explain(&self, query: String) -> tantivy::Result<()> {
        let searcher = self.index.reader()?.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.schema.get_field("id").unwrap(),
                self.schema.get_field("body").unwrap(),
            ],
        );

        let query = query_parser.parse_query(&query)?;

        println!("nb segments: {}", searcher.segment_readers().len());
        for (s_num, r) in searcher.segment_readers().iter().enumerate() {
            println!(
                "==== segment: {}, has_deleted? = {}",
                s_num,
                r.has_deletes()
            );

            for d in 0..r.max_doc() {
                // }
                // for d in r.doc_ids_alive() {
                println!("==== doc_id: {}", d);

                let e = query.explain(&searcher, tantivy::DocAddress(s_num as u32, d));
                match e {
                    Ok(e) => println!("{}", e.to_pretty_json()),
                    Err(e) => println!("err: {}", e),
                }
                // let d = r.get_store_reader().get(d)?;

                // let w = query.weight(&searcher, true)?;
                // w.explain(r, d);
            }
        }

        Ok(())
    }

    pub fn search(&self, query: String) -> tantivy::Result<Vec<Document>> {
        let searcher = self.index.reader()?.searcher();

        // let id = self.schema.get_field("id").unwrap();
        // let id_term = Term::from_field_text(id, &query);
        // let id_query = FuzzyTermQuery::new(id_term, 1, true);
        // let id_docs = searcher
        //     .search(&id_query, &TopDocs::with_limit(10))
        //     .unwrap();

        // let body = self.schema.get_field("body").unwrap();
        // let body_term = Term::from_field_text(body, &query);
        // let body_query = FuzzyTermQuery::new(body_term, 1, true);
        // let body_docs = searcher
        //     .search(&body_query, &TopDocs::with_limit(10))
        //     .unwrap();

        // println!("body docs: {}, id docs: {}", body_docs.len(), id_docs.len());
        // println!("body docs: {:?}", &body_docs);

        // Ok(id_docs
        //     .iter()
        //     .chain(body_docs.iter())
        //     .map(|(_score, doc_address)| searcher.doc(*doc_address).unwrap())
        //     .collect())

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.schema.get_field("body").unwrap(),
            ],
        );
        // query_parser.set_conjunction_by_default();

        let query = query_parser.parse_query(&query)?;
        let top_docs = searcher.search(&*query, &TopDocs::with_limit(10))?;

        let docs = top_docs
            .iter()
            .map(|(_score, doc_address)| searcher.doc(*doc_address).unwrap())
            .collect();

        Ok(docs)
    }
}
