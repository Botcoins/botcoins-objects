use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PriceHistory {
	pub list: BTreeMap<i64, f64> // BTree for optimal selection speed for time
}

impl From<Vec<(i64, f64)>> for PriceHistory {
	fn from(data: Vec<(i64, f64)>) -> Self {
		let mut list = BTreeMap::new();

		for (time, value) in data {
			list.entry(time).or_insert(value);
		}

		PriceHistory { list }
	}
}

impl Into<Vec<(i64, f64)>> for PriceHistory {
	fn into(self) -> Vec<(i64, f64)> {
		let mut vec = Vec::with_capacity(self.list.len());

		for ent in self.list.into_iter() {
			vec.push(ent);
		}

		return vec;
	}
}
