use std::fs;
use std::io::BufReader;
use std::fs::File;
use sophia_iri::Iri;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle::TurtleParser;
use sophia_api::prelude::TripleParser;
use sophia_api::source::TripleSource;
use sophia_api::graph::MutableGraph;


const FILE_NAME: &str = "/Users/jajabro/development/knowledge_graph/organizational-change-tracker/rdf/ontology/bank-organization.ttl";

fn main() {
    let f_str = fs::read_to_string(FILE_NAME);
    let file = File::open(FILE_NAME);
    let mut b_str = BufReader::new(file.unwrap());


    let p = TurtleParser {
        base: Some(Iri::new_unchecked("http://localhost/ex".to_string())),
    };

    let mut g = FastGraph::new();

    let mut c = TripleParser::parse(&p, &mut b_str);

    let _d = p.parse_str(&f_str.unwrap());
    
    let _ = TripleSource::for_each_triple(&mut c, |t| {
        let _ =MutableGraph::insert_triple(&mut g, t);
    });

    sophia_api::graph::Graph::objects(&g).for_each(|p| {
        println!("{:?}", p);
    });
}




