// use didkit::ssi::{self, jsonld::StaticLoader, rdf};
// use sha256::digest;
use std::error::Error;
use sha2::{Sha256, Digest};

/// Hashes a string using SHA-256
pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    
    // Convert the hash result to a hexadecimal string
    hex::encode(result)
}

pub struct Urdna2015 {
    pub value: String,
}

pub fn extract_after_last_delimiter(input: &str, delimiter: char) -> &str {
    // Split the string by the delimiter and collect the parts into a vector
    let parts: Vec<&str> = input.split(delimiter).collect();

    // Return the last part of the vector
    return parts.last().unwrap_or(&"")
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
