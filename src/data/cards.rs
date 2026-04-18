use std::collections::HashMap;
use super::types::Card;

pub fn all_cards() -> Vec<Card> {
    const JSON: &str = include_str!("../../data/cards.json");
    serde_json::from_str(JSON).expect("cards.json parse error")
}

pub fn lane_counts(cards: &[Card]) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    m.insert("all".to_string(), cards.len());
    for c in cards {
        *m.entry(c.lane.id().to_string()).or_insert(0) += 1;
    }
    m
}
