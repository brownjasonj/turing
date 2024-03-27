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


const FILE_NAME: &str = "/usr/local/google/home/jajabro/Development/knowledge-graph/organizational-change-tracker/rdf/ontology/bank-organization.ttl";

fn main() {
    let f_str = fs::read_to_string(FILE_NAME);
    let file = File::open(FILE_NAME);
    let mut b_str = BufReader::new(file.unwrap());


    let p = TurtleParser {
        base: Some(Iri::new_unchecked("http://localhost/ex".to_string())),
    };

    let g = FastGraph::new();

    let c = TripleParser::parse(&p, &mut b_str);

    let _c = p.parse_str(&f_str.unwrap());

}




