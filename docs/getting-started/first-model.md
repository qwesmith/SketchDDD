# Your First Model

Let's build a complete domain model for a library management system step by step.

## The Domain

We're modeling a library where:

- Members can borrow books
- Books have authors and are organized by genre
- Members have borrowing limits
- Late returns incur fines

## Step 1: Define the Context

Every model starts with a bounded context:

```sddd
context Library {
  // We'll add our domain elements here
}
```

A bounded context represents a linguistic boundary where terms have consistent meanings.

## Step 2: Define Core Entities

Entities are objects with unique identity that persists over time:

```sddd
context Library {
  entity Member {
    id: UUID
    name: String
    email: Email
    membershipDate: Date
    status: MembershipStatus
  }

  entity Book {
    id: UUID
    isbn: ISBN
    title: String
    publicationYear: Int
    available: Boolean
  }

  entity Author {
    id: UUID
    name: String
    biography: String?
  }

  entity Loan {
    id: UUID
    borrowDate: Date
    dueDate: Date
    returnDate: Date?
  }
}
```

!!! note "Optional Fields"
    The `?` suffix marks optional fields. `biography: String?` means the field can be `None`/`null`.

## Step 3: Define Value Objects

Value objects are defined by their attributes, not identity:

```sddd
context Library {
  // ... entities above ...

  value ISBN {
    prefix: String
    registrationGroup: String
    registrant: String
    publication: String
    checkDigit: String
  }

  value Fine {
    amount: Decimal
    currency: Currency
    reason: String
  }

  value Address {
    street: String
    city: String
    postalCode: String
    country: String
  }
}
```

!!! tip "When to Use Value Objects"
    Use value objects when:

    - Two instances with the same attributes are interchangeable
    - The concept doesn't need to be tracked over time
    - It represents a measurement, description, or specification

## Step 4: Define Enums

Enums represent fixed sets of values:

```sddd
context Library {
  // ... previous definitions ...

  enum MembershipStatus = Active | Suspended | Expired | Cancelled

  enum Genre =
    | Fiction
    | NonFiction
    | Science
    | History
    | Biography
    | Children
    | Mystery
    | Romance

  enum LoanStatus = Active | Returned | Overdue | Lost
}
```

## Step 5: Define Relationships (Morphisms)

Morphisms define how entities relate to each other:

```sddd
context Library {
  // ... previous definitions ...

  morphisms {
    // A book has one or more authors
    writtenBy: Book -> List<Author>

    // A book belongs to a genre
    genre: Book -> Genre

    // A loan connects a member and a book
    borrower: Loan -> Member
    loanedBook: Loan -> Book
    loanStatus: Loan -> LoanStatus

    // A member has an address
    address: Member -> Address?

    // Fines are associated with loans
    fines: Loan -> List<Fine>
  }
}
```

## Step 6: Define Aggregates

Aggregates define consistency boundaries:

```sddd
context Library {
  // ... previous definitions ...

  aggregate Member {
    root: Member
    contains: [Address]
    invariant: status != Suspended || loans.all(l => l.status == Returned)
  }

  aggregate Book {
    root: Book
    contains: []
  }

  aggregate Loan {
    root: Loan
    contains: [Fine]
    invariant: returnDate == null || returnDate >= borrowDate
  }
}
```

!!! info "Aggregate Rules"
    - The **root** is the entry point - all access goes through it
    - **contains** lists entities that exist only within this aggregate
    - **invariant** defines rules that must always be true

## Complete Model

Here's the full model:

```sddd
context Library {
  // === Entities ===
  entity Member {
    id: UUID
    name: String
    email: Email
    membershipDate: Date
    status: MembershipStatus
  }

  entity Book {
    id: UUID
    isbn: ISBN
    title: String
    publicationYear: Int
    available: Boolean
  }

  entity Author {
    id: UUID
    name: String
    biography: String?
  }

  entity Loan {
    id: UUID
    borrowDate: Date
    dueDate: Date
    returnDate: Date?
  }

  // === Value Objects ===
  value ISBN {
    prefix: String
    registrationGroup: String
    registrant: String
    publication: String
    checkDigit: String
  }

  value Fine {
    amount: Decimal
    currency: Currency
    reason: String
  }

  value Address {
    street: String
    city: String
    postalCode: String
    country: String
  }

  // === Enums ===
  enum MembershipStatus = Active | Suspended | Expired | Cancelled
  enum Genre = Fiction | NonFiction | Science | History | Biography | Children | Mystery | Romance
  enum LoanStatus = Active | Returned | Overdue | Lost

  // === Relationships ===
  morphisms {
    writtenBy: Book -> List<Author>
    genre: Book -> Genre
    borrower: Loan -> Member
    loanedBook: Loan -> Book
    loanStatus: Loan -> LoanStatus
    address: Member -> Address?
    fines: Loan -> List<Fine>
  }

  // === Aggregates ===
  aggregate Member {
    root: Member
    contains: [Address]
  }

  aggregate Book {
    root: Book
  }

  aggregate Loan {
    root: Loan
    contains: [Fine]
    invariant: returnDate == null || returnDate >= borrowDate
  }
}
```

## Step 7: Validate

```bash
sketchddd check library.sddd
```

## Step 8: Generate Code

```bash
# Generate Rust code
sketchddd codegen library.sddd --target rust --output src/domain.rs

# Generate TypeScript types
sketchddd codegen library.sddd --target typescript --output src/domain.ts
```

## Step 9: Visualize

```bash
sketchddd viz library.sddd --format mermaid --output library.md
```

## Adding a Second Context

Real systems often have multiple contexts. Let's add a Notifications context:

```sddd
context Library {
  // ... as above ...
}

context Notifications {
  entity Notification {
    id: UUID
    recipientEmail: Email
    subject: String
    body: String
    sentAt: DateTime?
  }

  enum NotificationType = DueReminder | OverdueNotice | FineNotice | WelcomeEmail
}

// Define how contexts relate
map LibraryToNotifications: Library -> Notifications {
  pattern: CustomerSupplier

  mappings {
    Member -> Notification  // "Members receive notifications"
  }
}
```

## Next Steps

- [Language Overview](../language/overview.md) - Complete language reference
- [Code Generation](../codegen/overview.md) - Customize generated code
- [Context Maps](../language/context-maps.md) - Model context relationships
