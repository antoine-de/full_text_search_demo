#[macro_use]
extern crate tantivy;
#[macro_use]
extern crate rustler;

use rustler::resource::ResourceArc;
use rustler::ListIterator;
use rustler::{Encoder, Env, Error, NifResult, Term};
use search::Searcher;
use std::sync::Mutex;

struct SearcherResource(Mutex<Searcher>);

mod atoms;
mod search;

rustler_export_nifs! {
    "Elixir.Tantivy.NIF",
    [
        ("init", 0, init),
        ("search", 2, search),
        ("add_entry", 3, add_entry),
        ("add_entries", 2, add_entries),
        ("explain", 2, explain),
    ],
    Some(load)
}

fn load(env: Env, _info: Term) -> bool {
    resource_struct_init!(SearcherResource, env);
    true
}

fn init<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource = ResourceArc::new(SearcherResource(Mutex::new(Searcher::new())));
    let resp = (atoms::ok(), resource);
    Ok(resp.encode(env))
}

fn search<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<SearcherResource> = args[0].decode()?;
    let query: String = args[1].decode()?;

    let searcher = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    match searcher.search(query) {
        Ok(docs) => {
            let terms: Vec<Term<'a>> = docs.into_iter().map(|doc| doc_to_term(env, doc)).collect();
            Ok(terms.encode(env))
        }
        Err(error) => Ok((atoms::error(), error_to_term(env, error)).encode(env)),
    }
}

fn explain<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<SearcherResource> = args[0].decode()?;
    let query: String = args[1].decode()?;

    let searcher = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    match searcher.explain(query) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(error) => Ok((atoms::error(), error_to_term(env, error)).encode(env)),
    }
}

fn add_entry<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<SearcherResource> = args[0].decode()?;
    let title: String = args[1].decode()?;
    let body: String = args[2].decode()?;

    let mut searcher = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    match searcher.add_entry(title, body) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(error) => Ok((atoms::error().encode(env), error_to_term(env, error)).encode(env)),
    }
}

fn add_entries<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<SearcherResource> = args[0].decode()?;
    let docs: ListIterator = args[1].decode()?;

    let docs = docs.map(|d| d.decode()).collect::<Result<_, _>>()?;

    let mut searcher = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    match searcher.add_entries(docs) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(error) => Ok((atoms::error().encode(env), error_to_term(env, error)).encode(env)),
    }
}

fn doc_to_term<'a>(env: Env<'a>, doc: tantivy::Document) -> Term<'a> {
    let terms: Vec<Term<'a>> = doc
        .field_values()
        .into_iter()
        .map(|fv| match fv.value() {
            tantivy::schema::Value::Str(v) => v.encode(env),
            tantivy::schema::Value::U64(v) => v.encode(env),
            tantivy::schema::Value::I64(v) => v.encode(env),
            tantivy::schema::Value::Facet(v) => v.encoded_str().encode(env),
            tantivy::schema::Value::Bytes(v) => v.encode(env),
            tantivy::schema::Value::Date(v) => v.to_rfc3339().encode(env),
        })
        .collect();
    terms.encode(env)
}

fn error_to_term<'a>(env: Env<'a>, error: tantivy::Error) -> Term<'a> {
    format!("{}", error).encode(env)
}
