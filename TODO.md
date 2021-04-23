# TODO tasks:
 - [ ] models/jsonrpc.rs:ErrorDetail "data" field is  missing
 - [ ] models/trading.rs:TradeRequest does not allow setting vadious Order parameters
 - [ ] models/trading.rs:EditRequest does not allow changing various Order parameters
 - [ ] models/subscription/channels/book.rs: BookData is missing `type` field - do we need it?
 - [ ] models/subscription/channels/book.rs: add field names to OrderBookDelta and make sure it it serded into/from tuple
 - [ ] models/subscription/channels/trades.rs: TradesData is missing `mark_price` field
 - [ ] models/subscriptions/channels/user_changes.rs: implementation [PR#6](https://github.com/dovahcrow/deribit-rs/pull/6) submitted, pending
 - [ ] models/subscriptiobs/channels/market_data.rs: `/public/get_tradingview_chart_data` not implemented
 - [ ] use `rust_decimal::preload::Decimal` type instead of f64 all over the code for following fields: `price`, `stop_price`, `tick`
