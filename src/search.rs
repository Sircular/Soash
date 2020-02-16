use atomic_counter::{AtomicCounter, RelaxedCounter};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{fs, sync::Mutex};
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    query::{AllQuery, BooleanQuery, Occur, Query, RangeQuery, TermQuery},
    schema::*,
    tokenizer::{Language, LowerCaser, RemoveLongFilter, Stemmer, Token, TokenStream, Tokenizer},
    DocAddress, Error, Index, IndexReader, IndexWriter, Term,
};

use crate::constants;

#[derive(Clone)]
struct HtmlTokenizer;

// Implementation largely borrowed from SimpleTokenizer; we can't just call out to the
// SimpleTokenizer due to Rust's borrowing rules.
impl<'a> Tokenizer<'a> for HtmlTokenizer {
    type TokenStreamImpl = HtmlTokenStream;

    fn token_stream(&self, raw_text: &'a str) -> Self::TokenStreamImpl {
        HtmlTokenStream::new(raw_text)
    }
}

struct HtmlTokenStream {
    text_tokens: Vec<String>,
    index: usize,
    token: Token,
}

impl HtmlTokenStream {
    fn new(raw_text: &str) -> HtmlTokenStream {
        let fragment = Html::parse_fragment(raw_text);
        let text_tokens = fragment
            .root_element()
            .text()
            .flat_map(|s| s.split(|c: char| !c.is_alphanumeric()))
            .filter(|s| s.len() > 0)
            .map(|s| String::from(s))
            .collect();

        HtmlTokenStream {
            text_tokens,
            index: 0,
            token: Token::default(),
        }
    }
}

impl TokenStream for HtmlTokenStream {
    fn advance(&mut self) -> bool {
        self.token.text.clear();
        self.token.position = self.token.position.wrapping_add(1);

        if self.index >= self.text_tokens.len() {
            return false;
        }

        // set these to zero, since HTML parsing changes the number and offsets of bytes
        // (converting HTML entities and the like).
        self.token.offset_from = 0;
        self.token.offset_to = 0;
        self.token.text.push_str(&self.text_tokens[self.index]);

        self.index += 1;
        true
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}

pub type DocumentId = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: DocumentId,
    pub title: String,
    pub body: String,
}

pub struct NoteStore {
    index: Index,
    reader: IndexReader,
    writer: Mutex<IndexWriter>,
    id_counter: RelaxedCounter,
}

impl NoteStore {
    pub fn new(index_dir: String) -> tantivy::Result<Self> {
        let mut builder = Schema::builder();

        let text_options = TextOptions::default()
            .set_indexing_options(TextFieldIndexing::default().set_tokenizer("en_html"))
            .set_stored();

        builder.add_u64_field("id", STORED | INDEXED | FAST);
        builder.add_u64_field("user_id", STORED | INDEXED | FAST);
        builder.add_text_field("title", text_options.clone());
        builder.add_text_field("body", text_options.clone());

        fs::create_dir_all(&index_dir)?;

        let index_dir = MmapDirectory::open(&index_dir)?;
        let index = Index::open_or_create(index_dir, builder.build())?;

        let en_html = HtmlTokenizer
            .filter(RemoveLongFilter::limit(40))
            .filter(LowerCaser)
            .filter(Stemmer::new(Language::English));
        index.tokenizers().register("en_html", en_html);

        let reader = index.reader()?;
        let writer = index.writer(constants::INDEXER_HEAP_SIZE)?;

        let store = NoteStore {
            index,
            reader,
            writer: Mutex::new(writer),
            id_counter: RelaxedCounter::new(0),
        };

        store.id_counter.add(store.next_id()?);

        Ok(store)
    }

    pub fn add_note(&self, user_id: u64, note: Note) -> tantivy::Result<DocumentId> {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();
        let user_id_field = schema.get_field("user_id").unwrap();

        let calculated_id = self.id_counter.inc();
        let mut writer = self.writer.lock()?;

        writer.add_document(doc!(
                id_field => calculated_id as u64,
                title_field => note.title,
                body_field => note.body,
                user_id_field => user_id,
        ));
        writer.commit()?;

        Ok(calculated_id)
    }

    pub fn get_note(&self, user_id: u64, id: DocumentId) -> tantivy::Result<Note> {
        let (_, doc) = self.get_note_doc(user_id, id)?;
        Ok(self.load_note(doc))
    }

    pub fn search_notes(
        &self,
        user_id: u64,
        query_text: &str,
        result_count: usize,
    ) -> tantivy::Result<Vec<Note>> {
        let schema = self.index.schema();
        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();
        let user_id_field = schema.get_field("user_id").unwrap();

        let user_query: Box<dyn Query> = Box::new(RangeQuery::new_u64(
            user_id_field,
            user_id as u64..(user_id + 1) as u64,
        ));

        let search_query =
            build_search_query(&self.index, vec![title_field, body_field], query_text);

        let compound_query = BooleanQuery::from(vec![
            (Occur::Must, user_query),
            (Occur::Should, search_query),
        ]);

        let reader = &self.reader;
        let searcher = reader.searcher();
        let fruit = searcher.search(&compound_query, &TopDocs::with_limit(result_count))?;

        Ok(fruit
            .iter()
            .filter(|(score, _)| *score > 1.)
            .map(|(_, addr)| self.load_note(searcher.doc(*addr).unwrap()))
            .collect())
    }

    pub fn search_similar(
        &self,
        user_id: u64,
        note_id: DocumentId,
        result_count: usize,
    ) -> tantivy::Result<Vec<Note>> {
        let (_, doc) = self.get_note_doc(user_id, note_id)?;
        let note = self.load_note(doc);

        let schema = self.index.schema();
        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();
        let user_id_field = schema.get_field("user_id").unwrap();

        let user_query: Box<dyn Query> = Box::new(RangeQuery::new_u64(
            user_id_field,
            user_id as u64..(user_id + 1) as u64,
        ));

        let title_query =
            build_search_query(&self.index, vec![title_field, body_field], &note.title);

        let body_query = build_search_query(&self.index, vec![title_field, body_field], &note.body);
        let compound_query = BooleanQuery::from(vec![
            (Occur::Must, user_query),
            (Occur::Should, title_query),
            (Occur::Should, body_query),
        ]);

        let reader = &self.reader;
        let searcher = reader.searcher();
        let fruit = searcher.search(&compound_query, &TopDocs::with_limit(result_count))?;

        Ok(fruit
            .iter()
            .filter(|(score, _)| *score > 1.)
            .map(|(_, addr)| self.load_note(searcher.doc(*addr).unwrap()))
            .filter(|note| note.id != note_id)
            .collect())
    }

    pub fn update_note(
        &self,
        user_id: u64,
        id: DocumentId,
        note: Note,
    ) -> tantivy::Result<DocumentId> {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();
        let user_id_field = schema.get_field("user_id").unwrap();

        if let Err(_) = self.get_note_doc(user_id, id) {
            return Err(Error::InvalidArgument(format!("{}, {}", user_id, id)));
        }

        let mut writer = self.writer.lock()?;
        writer.delete_term(Term::from_field_u64(id_field, id as u64));
        writer.add_document(doc!(
                id_field => id as u64,
                title_field => note.title,
                body_field => note.body,
                user_id_field => user_id,
        ));
        writer.commit()?;
        Ok(id)
    }

    pub fn delete_note(&self, user_id: u64, id: DocumentId) -> tantivy::Result<Note> {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();

        let (_, doc) = match self.get_note_doc(user_id, id) {
            Ok(pair) => pair,
            Err(_) => return Err(Error::InvalidArgument(format!("{}, {}", user_id, id))),
        };
        let note = self.load_note(doc);
        let mut writer = self.writer.lock()?;
        writer.delete_term(Term::from_field_u64(id_field, note.id as u64));
        writer.commit()?;
        Ok(note)
    }

    fn get_note_doc(
        &self,
        user_id: u64,
        id: DocumentId,
    ) -> tantivy::Result<(DocAddress, Document)> {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let user_id_field = schema.get_field("user_id").unwrap();

        let id_query: Box<dyn Query> =
            Box::new(RangeQuery::new_u64(id_field, id as u64..(id + 1) as u64));
        let user_query: Box<dyn Query> = Box::new(RangeQuery::new_u64(
            user_id_field,
            user_id as u64..(user_id + 1) as u64,
        ));
        let compound_query =
            BooleanQuery::from(vec![(Occur::Must, id_query), (Occur::Must, user_query)]);

        let searcher = self.reader.searcher();
        let fruit = match searcher.search(&compound_query, &TopDocs::with_limit(1)) {
            Ok(f) => f,
            _ => return Err(Error::InvalidArgument(format!("{}, {}", user_id, id))),
        };

        let addr = match fruit.as_slice() {
            [(_, addr)] => addr,
            _ => return Err(Error::InvalidArgument(format!("{}, {}", user_id, id))),
        }
        .clone();

        Ok((addr, searcher.doc(addr).expect("WIE???")))
    }

    fn load_note(&self, doc: Document) -> Note {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();
        return Note {
            id: doc.get_first(id_field).unwrap().u64_value() as DocumentId,
            title: String::from(doc.get_first(title_field).unwrap().text().unwrap()),
            body: String::from(doc.get_first(body_field).unwrap().text().unwrap()),
        };
    }

    pub fn next_id(&self) -> tantivy::Result<DocumentId> {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let searcher = self.reader.searcher();
        let fruit = searcher.search(
            &AllQuery,
            &TopDocs::with_limit(1).order_by_u64_field(id_field),
        )?;
        match fruit.as_slice() {
            [(_, addr)] => {
                let doc = searcher.doc(*addr).unwrap();
                let note = self.load_note(doc);
                Ok(note.id + 1)
            }
            _ => Ok(0),
        }
    }
}

fn build_search_query(index: &Index, fields: Vec<Field>, text: &str) -> Box<dyn Query> {
    let mut term_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
    for f in fields.clone().into_iter() {
        let mut tokens = Vec::new();
        let tokenizer = index.tokenizer_for_field(f).unwrap();
        let mut stream = tokenizer.token_stream(text);
        while let Some(token) = stream.next() {
            tokens.push(token.text.clone());
        }
        term_queries.push((Occur::Should, Box::new(build_multiterm_query(f, tokens))));
    }
    Box::new(BooleanQuery::from(term_queries))
}

fn build_multiterm_query(field: Field, tokens: Vec<String>) -> Box<dyn Query> {
    let mut term_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
    for t in tokens {
        term_queries.push((
            Occur::Should,
            Box::new(TermQuery::new(
                Term::from_field_text(field, &t),
                IndexRecordOption::Basic,
            )),
        ));
    }
    Box::new(BooleanQuery::from(term_queries))
}
