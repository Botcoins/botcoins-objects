#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Coin {
	pub info: PartialCoin,
	pub rank: u16,
	pub price_usd: f64,
	pub pct_change_1h: f64,
	pub pct_change_1d: f64,
	pub pct_change_1w: f64,
	pub provider: Provider,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PartialCoin {
	pub common_name: String,
	pub symbol: String,
	pub slug: String,
	pub unique_id: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Provider {
	Default,
	Bitfinex,
	BitPay,
	Bittrex,
	Coinbase,
	CoinMarketCap,
}

impl Provider {
	pub fn ident(&self) -> u16 {
		match *self {
			Provider::Default => 0,
			Provider::CoinMarketCap => 1,
			Provider::Coinbase => 2,
			Provider::BitPay => 3,
			Provider::Bitfinex => 4,
			Provider::Bittrex => 5
		}
	}
}

rocket_response_derive!(Coin, PartialCoin, Provider);
