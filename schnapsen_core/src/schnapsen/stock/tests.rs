use super::*;
use cards::{Card, Suit, Rank};

fn empty_stock() -> Stock {
    let stock_data_empty = StockData {cards: vec![], closed: false};
    Stock::new(stock_data_empty)
}

fn empty_dummy_stock() -> DummyStock {
    DummyStock::new(0, None, false)
}

fn closed_stock() -> Stock {
    let cards = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Ten)];
    let stock_data = StockData {cards, closed: true};
    Stock::new(stock_data)
}

fn closed_dummy_stock() -> DummyStock {
    DummyStock::new(2, Some(Rank::Ace), true)
}

fn normal_stock() -> Stock {
    let stock_data = StockData {
        cards: vec![Card::new(Suit::Leaves, Rank::Ace),
                    Card::new(Suit::Leaves, Rank::Ten)],
        closed: false};
    Stock::new(stock_data)
}

fn normal_dummy_stock() -> DummyStock {
    DummyStock::new(2, Some(Rank::Ace), false)
}

#[test]
fn test_stock_is_empty_iff_length_is_zero() {
    let stock_empty = empty_stock();

    check_is_empty_when_length_is_zero(&stock_empty);

    let stock_not_empty = normal_stock();

    check_is_empty_when_length_is_zero(&stock_not_empty);
}

#[test]
fn test_dummy_stock_is_empty_iff_length_is_zero() {
    let stock_empty = empty_dummy_stock();

    check_is_empty_when_length_is_zero(&stock_empty);

    let stock_not_empty = normal_dummy_stock();

    check_is_empty_when_length_is_zero(&stock_not_empty);
}

fn check_is_empty_when_length_is_zero(stock: &IStock) {
    let length_zero = stock.len() == 0;
    assert_eq!(length_zero, stock.is_empty());
}

#[test]
fn test_stock_trump_card_rank_is_none_when_empty() {
    let stock = empty_stock();
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_dummy_stock_trump_card_rank_is_none_when_empty() {
    let stock = empty_dummy_stock();
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_stock_trump_card_rank_is_none_when_closed() {
    let stock = closed_stock();
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_dummy_stock_trump_card_rank_is_none_when_closed() {
    let stock = closed_dummy_stock();
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_stock_trump_card_rank_is_ok() {
    let stock = normal_stock();
    assert_eq!(Some(Rank::Ace), stock.trump_card_rank());
}

#[test]
fn test_dummy_stock_trump_card_rank_is_ok() {
    let stock = normal_dummy_stock();
    assert_eq!(Some(Rank::Ace), stock.trump_card_rank());
}

#[test]
fn test_stock_exchange_trump_card_is_none_when_empty() {
    let mut stock = empty_stock();
    
    assert_eq!(None, stock.exchange_trump_card(Rank::Unter));
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_dummy_stock_exchange_trump_card_is_none_when_empty() {
    let mut stock = empty_dummy_stock();
    
    assert_eq!(None, stock.exchange_trump_card(Rank::Unter));
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_stock_exchange_trump_card_is_none_when_closed() {
    let mut stock = closed_stock();
    
    assert_eq!(None, stock.exchange_trump_card(Rank::Unter));
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_dummy_stock_exchange_trump_card_is_none_when_closed() {
    let mut stock = closed_dummy_stock();
    
    assert_eq!(None, stock.exchange_trump_card(Rank::Unter));
    assert_eq!(None, stock.trump_card_rank());
}

#[test]
fn test_stock_exchange_trump_card_is_ok() {
    let mut stock = normal_stock();

    let old_trump_card_rank = stock.trump_card_rank();
    let new_rank = Rank::Unter;
    assert_eq!(old_trump_card_rank, stock.exchange_trump_card(new_rank));
    assert_eq!(Some(new_rank), stock.trump_card_rank());
}

#[test]
fn test_dummy_stock_exchange_trump_card_is_ok() {
    let mut stock = normal_dummy_stock();

    let old_trump_card_rank = stock.trump_card_rank();
    let new_rank = Rank::Unter;
    assert_eq!(old_trump_card_rank, stock.exchange_trump_card(new_rank));
    assert_eq!(Some(new_rank), stock.trump_card_rank());
}

#[test]
fn test_stock_deal_is_none_when_empty() {
    let mut stock = empty_stock();
    assert_eq!(None, stock.deal());
}

#[test]
fn test_dummy_stock_deal_is_none_when_empty() {
    let mut stock = empty_dummy_stock();
    assert_eq!(None, stock.deal());
}

#[test]
fn test_stock_deal_is_none_when_closed() {
    let mut stock = closed_stock();
    assert_eq!(None, stock.deal());
}

#[test]
fn test_dummy_stock_deal_is_none_when_closed() {
    let mut stock = closed_dummy_stock();
    assert_eq!(None, stock.deal());
}

#[test]
fn test_stock_deal_ok() {
    let mut stock = normal_stock();

    let stock_size = stock.len();

    stock.deal();
    assert_eq!(stock_size - 1, stock.len());
}

#[test]
fn test_dummy_stock_deal_ok() {
    let mut stock = normal_dummy_stock();

    let stock_size = stock.len();

    stock.deal();
    assert_eq!(stock_size - 1, stock.len());
}
