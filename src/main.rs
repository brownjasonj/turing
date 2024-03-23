use std::fs;
use sophia_turtle::parser::turtle::parse_bufread;
// use rio_turtle::{TurtleParser, TurtleError};
// use rio_api::parser::TriplesParser;
use std::io::BufReader;
use std::fs::File;
use sophia_iri::Iri;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle::TurtleParser;
use sophia_api::prelude::TripleParser;


const FILE_NAME: &str = "/Users/jajabro/development/knowledge_graph/organizational-change-tracker/rdf/ontology/bank-organization.ttl";

fn main() {
    let f_str = fs::read_to_string(FILE_NAME);


    let p = TurtleParser {
        base: Some(Iri::new_unchecked("http://localhost/ex".to_string())),
    };



    let g = FastGraph::new();

    // let c = TripleParser::parse(&p, f_bufreader);
    let c = p.parse_str(&f_str.unwrap());
}




