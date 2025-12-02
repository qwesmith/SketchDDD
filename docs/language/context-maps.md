# Context Maps

Context maps define how bounded contexts relate to each other. They model the integration patterns between different parts of your system.

## Basic Syntax

```sddd
map MapName: SourceContext -> TargetContext {
  pattern: PatternType

  mappings {
    SourceType -> TargetType
  }
}
```

## Defining Context Maps

```sddd
context Sales {
  entity Customer { id: UUID, name: String }
  entity Order { id: UUID, total: Money }
}

context Shipping {
  entity Recipient { id: UUID, name: String, address: Address }
  entity Shipment { id: UUID, orderId: UUID }
}

map SalesToShipping: Sales -> Shipping {
  pattern: CustomerSupplier

  mappings {
    Customer -> Recipient
    Order -> Shipment
  }
}
```

## Integration Patterns

### CustomerSupplier

The upstream (supplier) context provides what the downstream (customer) needs:

```sddd
map OrderToFulfillment: Orders -> Fulfillment {
  pattern: CustomerSupplier

  mappings {
    Order -> FulfillmentRequest
    Customer -> ShippingRecipient
  }
}
```

**Use when:** Downstream has influence over upstream's development priorities.

### Conformist

The downstream context conforms to the upstream's model without influence:

```sddd
map ToExternalPaymentGateway: Billing -> PaymentGateway {
  pattern: Conformist

  mappings {
    Invoice -> PaymentRequest
    PaymentResult -> TransactionResult
  }
}
```

**Use when:** Working with external systems you can't change.

### AntiCorruptionLayer (ACL)

The downstream protects itself from upstream changes with a translation layer:

```sddd
map LegacyIntegration: NewSystem -> LegacySystem {
  pattern: AntiCorruptionLayer

  mappings {
    ModernCustomer -> LegacyCustomerRecord  // "Translates to legacy format"
    ModernOrder -> LegacyOrderRecord
  }
}
```

**Use when:** Integrating with legacy systems or unstable APIs.

### OpenHostService (OHS)

The upstream provides a well-defined protocol for multiple downstreams:

```sddd
map ProductCatalogAPI: Catalog -> PublicAPI {
  pattern: OpenHostService

  mappings {
    Product -> ProductDTO
    Category -> CategoryDTO
  }
}
```

**Use when:** Multiple consumers need access to your context.

### PublishedLanguage

Shared language used across contexts:

```sddd
map SharedTypes: Core -> Analytics {
  pattern: PublishedLanguage

  mappings {
    Money -> Money  // Same concept in both contexts
    DateRange -> DateRange
  }
}
```

**Use when:** Certain concepts are truly shared across contexts.

### SharedKernel

Two contexts share a common subset:

```sddd
map SharedIdentity: Auth -> UserManagement {
  pattern: SharedKernel

  mappings {
    UserId -> UserId  // Shared identity type
    AuthToken -> AuthToken
  }
}
```

**Use when:** Tight collaboration between teams, careful coordination required.

### SeparateWays

Contexts are completely independent:

```sddd
// No map needed - contexts don't interact
context Marketing { ... }
context HRSystem { ... }
```

**Use when:** No integration benefit, or integration cost too high.

## Object Mappings

Define how types translate between contexts:

```sddd
map SalesToSupport: Sales -> Support {
  pattern: CustomerSupplier

  mappings {
    // Simple mapping
    Customer -> SupportContact

    // With description
    Order -> SupportTicket  // "Orders become tickets when issues arise"

    // Multiple source types can map to same target
    Complaint -> SupportTicket
    Inquiry -> SupportTicket
  }
}
```

## Morphism Mappings

Map relationships between contexts:

```sddd
map CommerceToAnalytics: Commerce -> Analytics {
  pattern: PublishedLanguage

  mappings {
    Customer -> AnalyticsUser
    Order -> AnalyticsEvent
  }

  morphismMappings {
    placedBy -> triggeredBy  // Customer relationship maps
    items -> eventItems      // Order items relationship maps
  }
}
```

## Multiple Maps

A context can have relationships with many other contexts:

```sddd
context Orders {
  entity Order { ... }
  entity Customer { ... }
}

map OrdersToShipping: Orders -> Shipping {
  pattern: CustomerSupplier
  mappings { Order -> Shipment }
}

map OrdersToBilling: Orders -> Billing {
  pattern: CustomerSupplier
  mappings { Order -> Invoice }
}

map OrdersToNotifications: Orders -> Notifications {
  pattern: OpenHostService
  mappings { Order -> OrderNotification, Customer -> NotificationRecipient }
}
```

## Validation

SketchDDD validates context maps:

- Source and target contexts must exist
- Mapped types must exist in their respective contexts
- Pattern must be valid

```bash
sketchddd check model.sddd
```

Errors you might see:

```
error[E0101]: Context 'Shipping' not found
  --> model.sddd:45:25
   |
45 | map SalesToShipping: Sales -> Shipping {
   |                               ^^^^^^^^ Unknown context

error[E0102]: Type 'Customer' not found in context 'Sales'
  --> model.sddd:49:5
   |
49 |     Customer -> Recipient
   |     ^^^^^^^^ Did you mean 'Client'?
```

## Category Theory: Maps as Functors

In category theory, a context map is a **sketch morphism (functor)** that:

1. Maps objects to objects
2. Maps morphisms to morphisms
3. Preserves composition and identities

```
Source Context          Target Context
    A ----f---> B           A' ----f'---> B'
    |           |           |             |
    g           h     =>    g'            h'
    |           |           |             |
    v           v           v             v
    C ----k---> D           C' ----k'---> D'
```

The functor `F: Source -> Target` satisfies:
- `F(A) = A'`, `F(B) = B'`, etc.
- `F(f) = f'`, `F(g) = g'`, etc.
- `F(g ∘ f) = F(g) ∘ F(f)`

This ensures that relationships are preserved across context boundaries.

## Best Practices

### 1. Document the Relationship

```sddd
map OrderToFulfillment: Orders -> Fulfillment {
  pattern: CustomerSupplier
  // Orders team provides what Fulfillment needs
  // Fulfillment team can request changes to Orders API

  mappings {
    Order -> FulfillmentRequest
  }
}
```

### 2. Choose Patterns Based on Team Dynamics

| Situation | Pattern |
|-----------|---------|
| Can influence upstream | CustomerSupplier |
| Can't influence upstream | Conformist |
| Protecting from upstream changes | AntiCorruptionLayer |
| Providing API to many consumers | OpenHostService |
| Truly shared concepts | PublishedLanguage |
| Tight team collaboration | SharedKernel |
| No integration needed | SeparateWays |

### 3. Use ACL for External Systems

```sddd
map StripeIntegration: Billing -> StripeAPI {
  pattern: AntiCorruptionLayer

  // Our clean domain types
  mappings {
    Payment -> StripeCharge
    Customer -> StripeCustomer
  }
  // ACL handles translation to/from Stripe's API
}
```

### 4. Be Explicit About Mappings

```sddd
// Good: Clear what maps to what
mappings {
  Customer -> ShippingRecipient    // Customer becomes recipient
  CustomerAddress -> DeliveryAddress // Address is copied
  Order -> Shipment                  // Order triggers shipment
}

// Avoid: Vague or incomplete mappings
mappings {
  Customer -> Customer  // Same name, but are they really the same?
}
```
