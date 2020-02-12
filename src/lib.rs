use ::whisper::{
    knowledge_base::{KnowledgeBase, SerializedKnowledgeBase},
    prelude::*,
};

pub fn list(symbols: SymbolTable) -> KnowledgeBase {
    let list_kb_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/list.kb"));
    let deserialized: SerializedKnowledgeBase =
        bincode::deserialize_from(&mut &list_kb_bytes[..]).expect("this must not fail!");
    deserialized.into_knowledge_base(symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_no_bincode_errors() {
        let symbols = SymbolTable::new();
        let _ = list(symbols);
    }
}
