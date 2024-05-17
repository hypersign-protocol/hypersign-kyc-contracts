use didkit::ssi::{self, jsonld::StaticLoader, rdf};
use json_ld::JsonLdProcessor;
use sha256::digest;

// use sophia::api::inmem::FastGraph;
// use sophia::api::parser::nq;
// use sophia::api::serializer::jsonld;
// use sophia::api::serializer::Serializer;
// use sophia::api::triple::stream::TripleSource;
// use sophia::inmem::dataset::FastDataset;
// use sophia::turtle::parser::nq::NQuadsParser;
use std::error::Error;

pub struct Urdna2015 {
    pub value: String,
}

// pub fn test() {
//     let turtle = r#"
//             <http://localhost/ex#me> <http://example.org/ns/knows> _:b1.
//             _:b1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.org/ns/Person> <tag:g1>.
//             _:b1 <http://example.org/ns/name> "Alice" <tag:g1>.
//         "#;

//         let mut graph = FastDataset::new();
//         nq::parse_str(turtle).add_to_graph(&mut graph)?;

//         let mut buffer = Vec::new();
//         let mut serializer = jsonld::JsonLdSerializer::new(&mut buffer);
//         serializer.serialize_graph(&graph)?;

//         let json_ld = String::from_utf8(buffer)?;
//         print!("json {:?}", json_ld);
// }

// pub fn nquads_to_json_ld(nquads: &str) -> Result<String, Box<dyn Error>> {
//     // Parse the N-Quads into an RDF graph
//     let mut graph = FastGraph::new();
//     nq::parse_str(nquads).add_to_graph(&mut graph)?;

//     // Serialize the RDF graph to JSON-LD
//     let mut buffer = Vec::new();
//     let mut serializer = jsonld::JsonLdSerializer::new(&mut buffer);
//     serializer.serialize_graph(&graph)?;

//     // Convert the buffer to a JSON string
//     let json_ld = String::from_utf8(buffer)?;
//     Ok(json_ld)
// }

// https://w3c.github.io/vc-di-eddsa/#transformation-ed25519signature2020
pub async fn get_urdna2015_normalized_str(doc_str: &str) -> Urdna2015 {
    let mut loader = StaticLoader;

    let json = ssi::jsonld::syntax::to_value_with(
        serde_json::from_str::<serde_json::Value>(&doc_str).unwrap(),
        Default::default,
    )
    .unwrap();

    let doc = ssi::jsonld::RemoteDocument::new(None, None, json);
    let mut generator =
        rdf_types::generator::Blank::new_with_prefix("b".to_string()).with_default_metadata();

    let mut to_rdf = doc
        .to_rdf(&mut generator, &mut loader)
        .await
        .map_err(Box::new)
        .unwrap();

    let dataset: rdf::DataSet = to_rdf
        .cloned_quads()
        .map(|q| q.map_predicate(|p| p.into_iri().unwrap()))
        .collect();

    let dataset_normalized = ssi::urdna2015::normalize(dataset.quads().map(Into::into));
    let r = dataset_normalized.into_nquads();

    return Urdna2015 { value: r };
}
