# Morphisms

Morphisms define relationships between objects in your domain model. In category theory, a morphism is an arrow between objects - in DDD terms, it's how domain concepts relate to each other.

## Basic Syntax

```sddd
context Commerce {
  morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
    product: LineItem -> Product
    price: Product -> Money
  }
}
```

## Morphism Types

### One-to-One

A single object relates to exactly one other object:

```sddd
morphisms {
  // Every order has exactly one customer
  customer: Order -> Customer

  // Every product has exactly one category
  category: Product -> Category
}
```

### One-to-Many

A single object relates to multiple objects:

```sddd
morphisms {
  // An order has multiple line items
  items: Order -> List<LineItem>

  // A customer has multiple addresses
  addresses: Customer -> List<Address>

  // A category has multiple products
  products: Category -> Set<Product>
}
```

### Optional Relationships

Use `?` for relationships that may not exist:

```sddd
morphisms {
  // An order may or may not have a discount
  discount: Order -> Discount?

  // A product may not have a parent category
  parentCategory: Category -> Category?
}
```

### Many-to-Many

Model with explicit join entities or multiple morphisms:

```sddd
context Library {
  entity Book { ... }
  entity Author { ... }

  // Using lists (simple approach)
  morphisms {
    authors: Book -> List<Author>
    books: Author -> List<Book>
  }
}

// Or with an explicit join entity
context Library {
  entity Book { ... }
  entity Author { ... }
  entity BookAuthor {
    id: UUID
    role: AuthorRole  // e.g., "Primary", "Contributor"
  }

  morphisms {
    book: BookAuthor -> Book
    author: BookAuthor -> Author
    contributions: Book -> List<BookAuthor>
    works: Author -> List<BookAuthor>
  }
}
```

## Morphism Annotations

Add metadata to morphisms:

```sddd
morphisms {
  // Cascading delete
  items: Order -> List<LineItem> [cascade]

  // Lazy loading hint
  orders: Customer -> List<Order> [lazy]

  // Custom name for generated code
  owner: Pet -> Person [name: "petOwner"]

  // Multiple annotations
  reviews: Product -> List<Review> [lazy, cascade, indexed]
}
```

## Bidirectional Relationships

Define both directions explicitly:

```sddd
morphisms {
  // Forward direction
  customer: Order -> Customer

  // Reverse direction
  orders: Customer -> List<Order>
}
```

!!! note
    SketchDDD doesn't automatically infer reverse relationships. Define them explicitly if needed.

## Composition and Navigation

Morphisms can be composed to navigate the model:

```sddd
context Commerce {
  morphisms {
    items: Order -> List<LineItem>
    product: LineItem -> Product
    category: Product -> Category
  }

  // In invariants/equations, you can compose:
  // order.items.product.category
}
```

## Self-Referential Morphisms

Objects can relate to themselves:

```sddd
context Organization {
  entity Employee {
    id: UUID
    name: String
  }

  morphisms {
    manager: Employee -> Employee?
    directReports: Employee -> List<Employee>
  }
}

context Content {
  entity Category {
    id: UUID
    name: String
  }

  morphisms {
    parent: Category -> Category?
    children: Category -> List<Category>
  }
}
```

## Generated Code

=== "Rust"
    ```rust
    pub struct Order {
        pub id: OrderId,
        pub customer_id: CustomerId,  // One-to-one
        pub items: Vec<LineItem>,     // One-to-many
        pub discount: Option<Discount>, // Optional
    }

    impl Order {
        // Navigation method
        pub fn customer(&self, repo: &CustomerRepo) -> Option<Customer> {
            repo.find(self.customer_id)
        }
    }
    ```

=== "TypeScript"
    ```typescript
    interface Order {
      readonly id: OrderId;
      readonly customerId: CustomerId;  // Reference
      readonly items: LineItem[];        // Embedded
      readonly discount?: Discount;      // Optional
    }
    ```

=== "Kotlin"
    ```kotlin
    data class Order(
        val id: OrderId,
        val customerId: CustomerId,     // Reference
        val items: List<LineItem>,      // Collection
        val discount: Discount? = null  // Optional
    )
    ```

## Best Practices

### 1. Use Meaningful Names

```sddd
// Good: Clear domain meaning
morphisms {
  placedBy: Order -> Customer
  boughtAt: Purchase -> Store
  managedBy: Project -> Employee
}

// Avoid: Generic or technical names
morphisms {
  ref1: Order -> Customer
  fk_customer: Order -> Customer
}
```

### 2. Consider Cardinality Carefully

```sddd
// Is it really a list, or always exactly one?
morphisms {
  // If every order has exactly one shipping address:
  shippingAddress: Order -> Address

  // If orders can have multiple shipping addresses:
  shippingAddresses: Order -> List<Address>
}
```

### 3. Use Optional for Truly Optional Relationships

```sddd
morphisms {
  // Required: Every order must have a customer
  customer: Order -> Customer

  // Optional: Not every order has a referral
  referredBy: Order -> Customer?
}
```

### 4. Model Domain Concepts, Not Database Structures

```sddd
// Good: Domain-focused
morphisms {
  purchasedItems: Customer -> List<Product>
}

// Avoid: Database-focused
morphisms {
  customer_product_junction: Customer -> List<CustomerProductJunction>
}
```

## Category Theory Background

In category theory, morphisms are arrows between objects that compose and respect identities:

- **Composition**: If `f: A -> B` and `g: B -> C`, then `g ∘ f: A -> C`
- **Identity**: Every object `A` has `id_A: A -> A`
- **Associativity**: `h ∘ (g ∘ f) = (h ∘ g) ∘ f`

These properties ensure that:

1. You can navigate through relationships consistently
2. Path expressions are well-defined
3. The model forms a coherent structure
