use crate::messages::{EventType, OrderEvent, Response};
use matchbook::{Order, OrderError, OrderId, OrderType, Orderbook, Price, Quantity, Side};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid channel from Kraken")]
    InvalidChannel,
    #[error("Invalid response type from Kraken")]
    InvalidType,
    #[error("Response is empty")]
    Empty,
    #[error("Order Error: {0:?}")]
    OrderError(#[from] OrderError),
}

pub fn process_message(orderbook: &mut Orderbook, message: Response) -> Result<(), ParseError> {
    if message.channel != "level3" {
        return Err(ParseError::InvalidChannel);
    }

    if message.data.is_empty() {
        return Err(ParseError::Empty);
    }

    if message.message_type == "snapshot" {
        println!("Processing snapshot...");
        orderbook.clear_trades()
    };

    // After processing:
    println!(
        "Book has {} bid levels, {} ask levels",
        orderbook.get_levels().bids().len(),
        orderbook.get_levels().asks().len()
    );

    for data in &message.data {
        for bid in &data.bids {
            process_event(orderbook, bid, true)?;
        }

        for ask in &data.asks {
            process_event(orderbook, ask, false)?;
        }
    }

    Ok(())
}

fn process_event(
    orderbook: &mut Orderbook,
    event: &OrderEvent,
    is_bid: bool,
) -> Result<(), ParseError> {
    match event.event {
        EventType::Add => {
            let order = to_order(event, is_bid);
            match orderbook.add_order(order.clone()) {
                Ok(()) => {}
                Err(OrderError::IdExists) => {
                    // Order exists - this is a replace/update
                    // Delete old and add new
                    let _ = orderbook.cancel_order(order.order_id.clone());
                    let _ = orderbook.add_order(order);
                }
                Err(e) => return Err(e.into()),
            }
        }
        EventType::Modify => {
            let id = to_order_id(&event.order_id);
            let qty = to_quantity(event.order_qty);
            orderbook.modify_order(id, qty)?;
        }
        EventType::Delete => {
            let id = to_order_id(&event.order_id);
            orderbook.cancel_order(id)?;
        }
    }
    Ok(())
}

pub fn to_order_id(kraken_id: &str) -> OrderId {
    OrderId::new(kraken_id.to_string())
}
pub fn to_price(kraken_price: f64) -> Price {
    Price::new((kraken_price * 100.0) as u64) // cents precision
}

pub fn to_quantity(kraken_qty: f64) -> Quantity {
    Quantity((kraken_qty * 100_000_000.0) as u64) // 8 decimals for BTC qty is fine
}

pub fn to_side(is_bid: bool) -> Side {
    if is_bid { Side::Buy } else { Side::Sell }
}

pub fn to_order(kraken_order: &OrderEvent, is_bid: bool) -> Order {
    let order_id = to_order_id(&kraken_order.order_id);
    let price = to_price(kraken_order.limit_price);
    let quantity = to_quantity(kraken_order.order_qty);
    let side = to_side(is_bid);

    Order::new(
        order_id,
        OrderType::GoodTillCancelled,
        side,
        price,
        quantity,
    )
}
