// use didkit::ssi::{self, jsonld::StaticLoader, rdf};
// use sha256::digest;
use std::error::Error;

pub struct Urdna2015 {
    pub value: String,
}

// https://w3c.github.io/vc-di-eddsa/#transformation-ed25519signature2020
pub fn get_urdna2015_normalized_str(doc_str: &str) -> Urdna2015 {
    // let mut loader = StaticLoader;

    // let json = ssi::jsonld::syntax::to_value_with(
    //     serde_json::from_str::<serde_json::Value>(&doc_str).unwrap(),
    //     Default::default,
    // )
    // .unwrap();

    // let doc = ssi::jsonld::RemoteDocument::new(None, None, json);
    // let mut generator =
    //     rdf_types::generator::Blank::new_with_prefix("b".to_string()).with_default_metadata();

    // let mut to_rdf = doc
    //     .to_rdf(&mut generator, &mut loader)
    //     .await
    //     .map_err(Box::new)
    //     .unwrap();

    // let dataset: rdf::DataSet = to_rdf
    //     .cloned_quads()
    //     .map(|q| q.map_predicate(|p| p.into_iri().unwrap()))
    //     .collect();

    // let dataset_normalized = ssi::urdna2015::normalize(dataset.quads().map(Into::into));
    // let r = dataset_normalized.into_nquads();

    return Urdna2015 {
        value: doc_str.to_string(),
    };
}
