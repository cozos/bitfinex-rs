extern crate bitfinex;

use bitfinex::api::*;
use bitfinex::pairs::*;
use bitfinex::currency::*;
use bitfinex::precision::*;

fn main() {
    public_endpoints();
    //private_endpoints(); --> It is Under development
}

fn public_endpoints() {
    let api = Bitfinex::new(None, None);
  
    // TICKER
    let trading_pair = api.ticker.trading_pair(ETHUSD.to_owned());
    match trading_pair {
        Ok(answer) => println!("bid: {:?}  ask: {:?}", answer.bid, answer.ask),
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.ticker.funding_currency(USD.to_owned());
    match funding_currency {
        Ok(answer) => println!("bid: {:?}  ask: {:?}", answer.bid, answer.ask),
        Err(e) => println!("Error: {}", e),
    }

    // TRADES
    let trading_pairs = api.trades.trading_pair(ETHUSD.to_owned());
    match trading_pairs {
        Ok(trades) => {
            for trade in &trades {
                println!("Trading => amount: {:?}  price: {:?}", trade.amount, trade.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.trades.funding_currency(USD.to_owned());
    match funding_currency {
        Ok(trades) => {
            for trade in &trades {
                println!("Funding => amount: {:?}  price: {:?}", trade.amount, trade.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }        

    // BOOK
    let trading_pairs = api.book.trading_pair(ETHUSD.to_owned(), P0.to_owned());
    match trading_pairs {
        Ok(books) => {
            for book in &books {
                println!("Trading => price: {:?} amount: {:?}", book.price, book.amount);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.book.funding_currency(USD.to_owned(), P0.to_owned());
    match funding_currency {
        Ok(books) => {
            for book in &books {
                println!("Funding => rate: {:?} amount: {:?}", book.rate, book.amount);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }  

    // CANDLES
    let last = api.candles.last(ETHUSD.to_owned(), "1m".to_owned());
    match last {
        Ok(answer) => println!("Candle Last => High: {:?} low: {:?}", answer.high, answer.low),
        Err(e) => println!("Error: {}", e),
    }    

    let history = api.candles.history(ETHUSD.to_owned(), "12h".to_owned());
    match history {
        Ok(candles) => {
            for candle in &candles {
                println!("Candle History => High: {:?} Low: {:?}", candle.high, candle.low);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }             
}

fn private_endpoints() {
    let api = Bitfinex::new(None, None);
  
    // ORDERS
    let active_orders = api.orders.active_orders(IOTUSD.to_owned());
    match active_orders {
        Ok(orders) => {
            for order in &orders {
                println!("Active orders => Symbol: {:?} amount: {:?} price: {:?}", order.symbol, order.amount, order.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }       
}