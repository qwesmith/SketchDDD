# Functors and Context Maps

Functors are structure-preserving maps between categories. In SketchDDD, context maps are functors between bounded contexts.

## What is a Functor?

A functor F: C → D maps:
- Objects in C to objects in D
- Morphisms in C to morphisms in D

While preserving:
- **Identity**: F(id_A) = id_{F(A)}
- **Composition**: F(g ∘ f) = F(g) ∘ F(f)

## Context Maps as Functors

In DDD, bounded contexts communicate through context maps. These are functors:

```sddd
map OrderToShipping: Orders -> Shipping {
  pattern: CustomerSupplier
  mappings {
    Order -> Shipment
    Customer -> Recipient
    Address -> DeliveryAddress
  }
}
```

This functor maps:
- `Order` ↦ `Shipment`
- `Customer` ↦ `Recipient`
- `Address` ↦ `DeliveryAddress`

And implicitly maps morphisms:
- `customer: Order → Customer` ↦ `recipient: Shipment → Recipient`

## Types of Functors

### Faithful Functors

A functor is **faithful** if it's injective on morphisms. This means distinct relationships in the source remain distinct.

```sddd
map Faithful: Orders -> Reporting {
  // Different Order relationships map to different Report relationships
}
```

### Full Functors

A functor is **full** if it's surjective on morphisms. Every relationship in the target comes from the source.

```sddd
map Full: Orders -> OrderHistory {
  // All OrderHistory relationships come from Orders
}
```

### Forgetful Functors

**Forgetful functors** "forget" structure:

```sddd
map OrderToSummary: Orders -> Reports {
  mappings {
    Order -> OrderSummary  // Loses detailed fields
    LineItem -> (forgotten)  // Entire entity forgotten
  }
}
```

### Embedding Functors

**Embeddings** preserve all structure (fully faithful):

```sddd
map CoreToFull: CoreOrders -> FullOrders {
  // All structure preserved
  // CoreOrders is a "sub-context" of FullOrders
}
```

## Integration Patterns as Functors

### Customer/Supplier

The supplier context serves the customer context:

```sddd
map OrderToPayment: Orders -> Payments {
  pattern: CustomerSupplier

  mappings {
    Order -> PaymentRequest
    Money -> Amount
  }
}
```

The supplier (Payments) defines the interface, customer (Orders) adapts to it.

### Conformist

The downstream context conforms to the upstream model:

```sddd
map LocalToExternal: LocalInventory -> ExternalCatalog {
  pattern: Conformist

  // We use their model directly
  mappings {
    Product -> ExternalProduct
  }
}
```

### Anti-Corruption Layer

An ACL translates between contexts, preventing corruption:

```sddd
map LegacyToModern: LegacySystem -> ModernDomain {
  pattern: AntiCorruptionLayer

  // Translation layer
  mappings {
    LegacyOrder -> Order
    LegacyCustomer -> Customer
  }
}
```

The ACL is a functor with explicit translation logic.

### Open Host Service

The upstream provides a well-defined protocol:

```sddd
map ServiceAPI: InternalOrders -> PublicAPI {
  pattern: OpenHostService

  // Published language
  mappings {
    Order -> OrderDTO
    Customer -> CustomerDTO
  }
}
```

## Natural Transformations

A **natural transformation** η: F → G between functors is a family of morphisms that commute:

```
     F(A) ──F(f)──→ F(B)
       │              │
   η_A │              │ η_B
       ↓              ↓
     G(A) ──G(f)──→ G(B)
```

In SketchDDD, this represents consistent translations:

```sddd
// Two different ways to map Orders to External
map ToExternalV1: Orders -> External { ... }
map ToExternalV2: Orders -> External { ... }

// Natural transformation: V1 → V2
// Upgrade path that commutes with all operations
```

## Adjunctions

An **adjunction** F ⊣ G between functors captures a fundamental relationship:

```
Hom(F(A), B) ≅ Hom(A, G(B))
```

In domain terms:
- F: "Free" construction (create from minimal data)
- G: "Forgetful" functor (extract data)

Example:
- `F`: Create Order from OrderRequest
- `G`: Extract OrderRequest from Order

## Functor Composition

Functors compose, enabling multi-context mappings:

```sddd
// Orders → Shipping
map A: Orders -> Shipping { ... }

// Shipping → Logistics
map B: Shipping -> Logistics { ... }

// Composition: Orders → Logistics
// B ∘ A preserves all structure
```

## Practical Applications

### 1. API Boundaries

Context maps define API contracts:

```sddd
map InternalToAPI: Domain -> REST {
  mappings {
    Order -> OrderResponse
    Customer -> CustomerResponse
  }
}
```

### 2. Event Translation

Events crossing context boundaries:

```sddd
map OrderEvents: Orders -> Notifications {
  mappings {
    OrderCreated -> SendConfirmation
    OrderShipped -> SendShippingNotice
  }
}
```

### 3. Data Migration

Migrating between schema versions:

```sddd
map V1ToV2: SchemaV1 -> SchemaV2 {
  mappings {
    OldOrder -> NewOrder
    // Fields rearranged, types changed
  }
}
```

### 4. Testing

Test doubles as functors:

```sddd
map RealToMock: Production -> Testing {
  mappings {
    OrderRepository -> MockOrderRepository
    PaymentGateway -> FakePaymentGateway
  }
}
```

## Verification

SketchDDD verifies functorial properties:

```bash
sketchddd check domain.sddd

# Checks:
# ✓ All objects in source have mappings
# ✓ Morphism mappings are consistent
# ✓ Composition is preserved
# ✓ Identity is preserved
```

## Example: Complete Context Map

```sddd
context Orders {
  entity Order {
    id: UUID
    customer: Customer
    items: List<LineItem>
    total: Money
    status: OrderStatus
  }

  entity Customer { ... }
  entity LineItem { ... }
  value Money { ... }
  enum OrderStatus = ...

  morphisms {
    customer: Order -> Customer
    items: Order -> List<LineItem>
  }
}

context Shipping {
  entity Shipment {
    id: UUID
    recipient: Recipient
    packages: List<Package>
    status: ShipmentStatus
  }

  entity Recipient { ... }
  entity Package { ... }
  enum ShipmentStatus = ...

  morphisms {
    recipient: Shipment -> Recipient
    packages: Shipment -> List<Package>
  }
}

// Functor: Orders → Shipping
map OrderToShipping: Orders -> Shipping {
  pattern: CustomerSupplier

  mappings {
    Order -> Shipment
    Customer -> Recipient
    LineItem -> Package
    OrderStatus -> ShipmentStatus

    // Morphisms map accordingly:
    // customer ↦ recipient
    // items ↦ packages
  }
}
```

This functor ensures:
- Every Order can become a Shipment
- Customer relationships map to Recipient relationships
- The structure of orders is preserved in shipping
