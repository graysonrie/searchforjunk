use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, QueryParser, RangeQuery, TermQuery},
    schema::Schema,
    DateTime, Searcher, TantivyDocument, Term,
};

use crate::{
    shared::dtos::file_dto::FileDTO,
    tantivy_file_indexer::{
        converters::doc_to_dto::doc_to_dto, models::search_params_model::SearchParamsModel,
    },
};

pub fn advanced_query(
    schema: &Schema,
    searcher: &Searcher,
    search_params: &SearchParamsModel,
) -> tantivy::Result<Vec<FileDTO>> {
    let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

    if let Some(file_path) = &search_params.file_path {
        let field = schema.get_field("path").unwrap();
        let query_parser = QueryParser::for_index(searcher.index(), vec![field]);
        let query = query_parser.parse_query(file_path)?;
        queries.push((Occur::Should, Box::new(query)));
    }

    if let Some(query_str) = &search_params.name {
        let field = schema.get_field("name").unwrap();
        let query_parser = QueryParser::for_index(searcher.index(), vec![field]);
        let query = query_parser.parse_query(query_str)?;
        queries.push((Occur::Should, Box::new(query)));
    }

    if let Some(date_range) = &search_params.date_range {
        let start_date = DateTime::from_utc(date_range.start);
        let end_date = DateTime::from_utc(date_range.end);
        let query = RangeQuery::new_date("date_modified".to_string(), start_date..end_date);
        queries.push((Occur::Must, Box::new(query)));
    }

    if let Some(metadata) = &search_params.metadata {
        let field = schema.get_field("metadata").unwrap();
        let term = Term::from_field_text(field, metadata);
        let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);
        queries.push((Occur::Must, Box::new(query)));
    }

    // Combine all the queries into a BooleanQuery
    let boolean_query = BooleanQuery::new(queries);

    // Execute the query and collect the results
    let top_docs = searcher.search(
        &boolean_query,
        &TopDocs::with_limit(10).tweak_score(|segment_reader: &tantivy::SegmentReader| {
            let popularity_field = segment_reader
                .fast_fields()
                .f64("popularity")
                .expect("Failed to access popularity field");
            move |doc, original_score| {
                // Default to 1 if no popularity
                let pop_score = popularity_field.first(doc).unwrap_or(1.0);
                apply_popularity(original_score, pop_score)
            }
        }),
    )?;

    let results: Vec<FileDTO> = top_docs
        .into_iter()
        .map(|(_score, doc_address)| {
            let doc: TantivyDocument = searcher.doc(doc_address).unwrap();
            doc_to_dto(doc, schema, _score)
        })
        .collect();

    Ok(results)
}

pub fn apply_popularity(existing_score: f32, popularity_score: f64) -> f64 {
    (existing_score as f64) + popularity_score.log(10.0)
}
