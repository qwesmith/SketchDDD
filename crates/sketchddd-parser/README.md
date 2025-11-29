# sketchddd-parser

DSL parser for [SketchDDD](https://github.com/ibrahimcesar/SketchDDD) domain models.

## Example

```sketchddd
context Commerce {
  objects { Customer, Order, LineItem, Product, Money }

  morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
  }

  aggregate Order {
    root: Order
    contains: [LineItem]
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  enum OrderStatus = Pending | Confirmed | Shipped
}
```

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE) at your option.
