use client::*;
use errors::*;
use serde_json::from_str;
use serde_json::value;

#[derive(Serialize, Deserialize, Debug)]
pub struct TickersTrading {
    pub symbol: String,
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickersFunding {
    pub symbol: String,
    pub frr: f64,
    pub bid: f64,
    pub bid_period: i64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_period: i64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_relative: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
    pub _placeholder1: value::Value,
    pub _placeholder2: value::Value,
    pub frr_amount_available: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TickersData {
    TickersTrading(TickersTrading),
    TickersFunding(TickersFunding),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPair {
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub frr: f64,
    pub bid: f64,
    pub bid_period: i64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_period: i64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64
}

#[derive(Clone)]
pub struct Ticker {
    client: Client,
}

impl Ticker {
    pub fn new() -> Self {
        Ticker {
            client: Client::new(None, None),
        }
    }

    pub fn funding_currency<S>(&self, symbol: S) -> Result<FundingCurrency>
        where S: Into<String>
    {
        let endpoint: String = format!("ticker/f{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let ticker: FundingCurrency = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn trading_pair<S>(&self, symbol: S) -> Result<TradingPair>
        where S: Into<String>
    {
        let endpoint: String = format!("ticker/t{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let ticker: TradingPair = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn all_tickers<>(&self) -> Result<Vec<TickersData>> {
        let endpoint: String = format!("tickers?symbols=ALL");

        let data = self.client.get(endpoint, String::new())?;

        let ticker: Vec<TickersData> = from_str(data.as_str())?;

        Ok(ticker)
    }
}
