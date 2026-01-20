use criterion::{Criterion, criterion_group, criterion_main};
use matchbook::order::Order;
use matchbook::orderbook::Orderbook;
use matchbook::types::{OrderId, OrderType, Price, Quantity, Side};
use std::hint::black_box;

fn add_order_benchmark(c: &mut Criterion) {
    let mut ob = Orderbook::new();
    let order = Order::new(
        OrderId("1".to_string()),
        OrderType::GoodForDay,
        Side::Buy,
        Price(10),
        Quantity(100),
    );
    c.bench_function("add_order", |b| {
        b.iter_batched(
            || {
                let mut ob = Orderbook::new();
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
                ob.add_order(order);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, add_order_benchmark);
criterion_main!(benches);
