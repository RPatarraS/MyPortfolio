use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Security {
    pub id: u8, // id needed inside Security because parent Vector might mutate
    pub name: String,
    quantity: u8,
    entries: Vec<Entry>,
    current_price_per_unit: f32,
    current_total_invested_value: f32,
    current_total_current_value: f32,
}

impl Security {
    pub fn new(id: u8, name: String, quantity: u8) -> Self {
        Self {
            id: id,
            name: name,
            quantity: quantity,
            entries: Vec::new(),
            current_price_per_unit: 0.0,
            current_total_invested_value: 0.0,
            current_total_current_value: 0.0,
        }
    }

    pub fn add_entry(&mut self, date: String, quantity: u8, price_per_unit: f32) {
        self.entries
            .push(Entry::new(date, quantity, price_per_unit));
        self.quantity += quantity;
    }

    pub fn update_current_price(&mut self, price_per_unit: f32) {
        self.current_price_per_unit = price_per_unit;
    }

    pub fn get_current_price_per_unit(&self) -> f32 {
        self.current_price_per_unit
    }
    pub fn get_quantity(&self) -> u8 {
        self.quantity
    }

    pub fn get_entries(&self) -> Vec<(String, u8, f32)> {
        self.entries
            .iter()
            .map(|entry| (entry.date.clone(), entry.quantity, entry.price_per_unit))
            .collect()
    }

    pub fn get_total_invested_value(&self) -> f32 {
        self.current_total_invested_value
    }

    pub fn calculate_total_invested_value(&mut self) {
        self.current_total_invested_value = 0.0;
        for entry in self.entries.iter() {
            self.current_total_invested_value += entry.price_per_unit * entry.quantity as f32;
        }
    }

    pub fn get_total_current_value(&self) -> f32 {
        self.current_total_current_value
    }

    pub fn calculate_total_current_value(&mut self) {
        self.current_total_current_value = self.current_price_per_unit * self.quantity as f32;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Entry {
    date: String,
    quantity: u8,
    price_per_unit: f32,
}

impl Entry {
    pub fn new(date: String, quantity: u8, price_per_unit: f32) -> Self {
        Self {
            date: date,
            quantity: quantity,
            price_per_unit: price_per_unit,
        }
    }
}
