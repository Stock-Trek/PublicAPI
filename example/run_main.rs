use crate::{
    data::context::StockTrekContext,
    run_algorithm::Algorithm,
    schemas::signal::{Event, Generator, Metadata, Provenance, StockTrekEvent},
};
use std::collections::HashMap;
use uuid::Uuid;

pub fn main() {
    let algorithm = Algorithm::default();
    let exchanges = HashMap::new();
    let context = StockTrekContext { exchanges };
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
