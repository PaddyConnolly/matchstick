use criterion::{Criterion, criterion_group, criterion_main};
use matchbook::order::Order;
use matchbook::orderbook::Orderbook;
use matchbook::types::{OrderId, OrderType, Price, Quantity, Side};

fn add_order_benchmark(c: &mut Criterion) {
    c.bench_function("add_order", |b| {
        b.iter_batched(
            || {
                let ob = Orderbook::new();
                let order = Order::new(
                    OrderId::new("1".to_string()),
                    OrderType::GoodForDay,
                    Side::Buy,
                    Price(10),
                    Quantity(100),
                );
                (ob, order)
            },
            |(mut ob, order)| {
                let _ = ob.add_order(order);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cancel_order_benchmark(c: &mut Criterion) {
    c.bench_function("cancel_order", |b| {
        b.iter_batched(
            || {
                let mut ob = Orderbook::new();
                let order_id = OrderId::new("1".to_string());
                let order = Order::new(
                    order_id.clone(),
                    OrderType::GoodForDay,
                    Side::Buy,
                    Price(10),
                    Quantity(100),
                );
                let _ = ob.add_order(order.clone());
                (ob, order_id)
            },
            |(mut ob, order_id)| ob.cancel_order(order_id),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn modify_order_benchmark(c: &mut Criterion) {
    c.bench_function("modify_order", |b| {
        b.iter_batched(
            || {
                let mut ob = Orderbook::new();
                let order_id = OrderId::new("1".to_string());
                let order = Order::new(
                    order_id.clone(),
                    OrderType::GoodForDay,
                    Side::Buy,
                    Price(10),
                    Quantity(100),
                );
                let _ = ob.add_order(order.clone());
                (ob, order_id)
            },
            |(mut ob, order_id)| ob.modify_order(order_id, Quantity(200)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn match_orders_benchmark(c: &mut Criterion) {
    c.bench_function("match_orders", |b| {
        b.iter_batched(
            || {
                let mut ob = Orderbook::new();
                // Add resting sell orders
                for i in 0..100 {
                    let order = Order::new(
                        OrderId::new(format!("sell_{}", i)),
                        OrderType::GoodTillCancelled,
                        Side::Sell,
                        Price::new(1000 + i as u64),
                        Quantity(100),
                    );
                    ob.add_order(order).unwrap();
                }
                // Add buy order that will match
                let buy = Order::new(
                    OrderId::new("buy_aggressor".to_string()),
                    OrderType::GoodTillCancelled,
                    Side::Buy,
                    Price::new(1050),
                    Quantity(5000),
                );
                ob.add_order(buy).unwrap();
                ob
            },
            |mut ob| ob.match_orders(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn get_levels_benchmark(c: &mut Criterion) {
    c.bench_function("get_levels", |b| {
        b.iter_batched(
            || {
                let mut ob = Orderbook::new();
                // Build a realistic book with many levels
                for i in 0..1000 {
                    let bid = Order::new(
                        OrderId::new(format!("bid_{}", i)),
                        OrderType::GoodTillCancelled,
                        Side::Buy,
                        Price::new(1000 - (i % 100) as u64),
                        Quantity(100),
                    );
                    let ask = Order::new(
                        OrderId::new(format!("ask_{}", i)),
                        OrderType::GoodTillCancelled,
                        Side::Sell,
                        Price::new(1001 + (i % 100) as u64),
                        Quantity(100),
                    );
                    ob.add_order(bid).unwrap();
                    ob.add_order(ask).unwrap();
                }
                ob
            },
            |ob| ob.get_levels(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    add_order_benchmark,
    cancel_order_benchmark,
    modify_order_benchmark,
    match_orders_benchmark,
    get_levels_benchmark
);

criterion_main!(benches);
