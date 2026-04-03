use crate::{
    algorithm::StockTrekAlgorithm,
    context::StockTrekContext,
    schemas::signal::{Event, Generator, Metadata, Provenance, StockTrekEvent},
};
use std::collections::HashMap;
use uuid::Uuid;

#[traitreg::registry(StockTrekAlgorithm)]
static ALGORITHM_REGISTRY: () = ();

pub fn main() {
    let algorithm_name_arg = std::env::args().next();
    if let Some(algorithm) = get_algorithm(algorithm_name_arg) {
        let markets = HashMap::new();
        let context = StockTrekContext::new(markets);
        let signal = algorithm.create_signal(context);
        let metadata = Metadata {
            event: Event {
                event_id: Uuid::default(),
                generated_timestamp_ms: 64573689375942,
                generated_timezone: "UTC".into(),
            },
            generator: Generator {
                creator: "fdskf fhdisf".into(),
                generator_id: Uuid::default(),
                name: "fdosf gfog  joidsfsd".into(),
                version: "12.765.2".into(),
            },
            provenance: Provenance {
                description: "fsdl dsghdsgh af fdhsfdhs krfh kd".into(),
                methodology: "nflds fdslhf dshksd fhkd dhskf".into(),
                references: vec![],
            },
            schema: "https://stock-trek.com/docs/schemas/signal/v0.3/schema.json".into(),
        };
        let event = StockTrekEvent { metadata, signal };
        println!("Created event {}", event.metadata.event.event_id);
    }
}

pub fn get_algorithm(algorithm_name_arg: Option<String>) -> Option<Box<dyn StockTrekAlgorithm>> {
    if let Some(algorithm_name) = algorithm_name_arg {
        for registration in ALGORITHM_REGISTRY.iter() {
            if registration.has_constructor() && registration.name() == algorithm_name {
                return registration.instanciate();
            }
        }
        println!("Could not find algorithm with name {}", algorithm_name);
        let names = ALGORITHM_REGISTRY
            .iter()
            .map(|a| a.name())
            .collect::<Vec<_>>()
            .join(", ");
        println!("Found algorithms [{}]", names);
        None
    } else {
        println!(
            "No algorithm specified, algorithm name must be the first parameter when running the command"
        );
        None
    }
}
