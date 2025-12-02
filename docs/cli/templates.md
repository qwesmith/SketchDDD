# Templates

Templates provide starting points for new SketchDDD projects with pre-defined domain structures.

## Using Templates

### Create Project from Template

```bash
sketchddd init my-project --template <TEMPLATE_NAME>
```

### List Available Templates

```bash
sketchddd template list
```

Output:

```
Built-in Templates:
  minimal        - Minimal empty project
  ecommerce      - E-commerce domain model
  microservices  - Microservices architecture example

Installed Templates:
  my-company     - Custom company template
```

## Built-in Templates

### minimal

Empty project with just the structure:

```bash
sketchddd init my-domain --template minimal
```

Creates:

```
my-domain/
└── my-domain.sddd
```

Content:

```sddd
context MyDomain {
  // Define your domain here
}
```

### ecommerce

E-commerce domain with customers, orders, products:

```bash
sketchddd init my-shop --template ecommerce
```

Creates:

```sddd
context Commerce {
  entity Customer {
    id: UUID
    email: Email
    name: String
    createdAt: DateTime
  }

  entity Product {
    id: UUID
    name: String
    description: String
    price: Money
    stock: Int
  }

  entity Order {
    id: UUID
    orderNumber: String
    total: Money
    status: OrderStatus
    createdAt: DateTime
  }

  entity LineItem {
    id: UUID
    quantity: Int
    unitPrice: Money
  }

  value Money {
    amount: Decimal
    currency: Currency
  }

  enum OrderStatus = Pending | Confirmed | Processing | Shipped | Delivered | Cancelled
  enum Currency = USD | EUR | GBP | JPY

  morphisms {
    customer: Order -> Customer
    items: Order -> List<LineItem>
    product: LineItem -> Product
    orderStatus: Order -> OrderStatus
  }

  aggregate Order {
    root: Order
    contains: [LineItem]
    invariant: total = sum(items.map(i => i.quantity * i.unitPrice))
  }
}
```

### microservices

Microservices architecture with multiple contexts:

```bash
sketchddd init my-system --template microservices
```

Creates:

```sddd
context UserService {
  entity User {
    id: UUID
    email: Email
    role: UserRole
  }
  enum UserRole = Admin | Member | Guest
}

context OrderService {
  entity Order {
    id: UUID
    userId: UUID
    status: OrderStatus
  }
  enum OrderStatus = Created | Paid | Fulfilled
}

context NotificationService {
  entity Notification {
    id: UUID
    recipientId: UUID
    channel: NotificationChannel
    sentAt: DateTime?
  }
  enum NotificationChannel = Email | SMS | Push
}

map UserToOrder: UserService -> OrderService {
  pattern: CustomerSupplier
  mappings {
    User -> Order  // "User places orders"
  }
}

map OrderToNotification: OrderService -> NotificationService {
  pattern: OpenHostService
  mappings {
    Order -> Notification
  }
}
```

## Template Management

### Get Template Info

```bash
sketchddd template info ecommerce
```

Output:

```
Template: ecommerce
Source: Built-in

Description:
  Complete e-commerce domain model including customers, products,
  orders, and inventory management.

Contexts:
  - Commerce

Entities:
  - Customer
  - Product
  - Order
  - LineItem

Value Objects:
  - Money
  - Address

Files:
  - commerce.sddd (main model)
```

### Validate a Template

```bash
sketchddd template validate ./my-template
```

## Creating Custom Templates

### Create Template Structure

```bash
sketchddd template create my-template
```

Creates:

```
my-template/
├── template.json    # Template metadata
└── my-template.sddd # Template model
```

### Template Metadata

Edit `template.json`:

```json
{
  "name": "my-template",
  "description": "My custom domain template",
  "version": "1.0.0",
  "author": "Your Name",
  "files": ["my-template.sddd"],
  "variables": {
    "PROJECT_NAME": {
      "description": "Name of the project",
      "default": "MyProject"
    }
  }
}
```

### Template Variables

Use variables in template files:

```sddd
context {{PROJECT_NAME}} {
  entity {{PROJECT_NAME}}Entity {
    id: UUID
  }
}
```

Variables are replaced during `init`:

```bash
sketchddd init my-project --template my-template
# PROJECT_NAME becomes "my-project"
```

### Install Custom Template

```bash
# From local directory
sketchddd template install ./my-template

# From URL (coming soon)
sketchddd template install https://github.com/user/template.git
```

### Remove Template

```bash
sketchddd template remove my-template

# Force remove without confirmation
sketchddd template remove my-template --force
```

## Template Storage

Templates are stored in:

- **Built-in**: Bundled with SketchDDD
- **User templates**: `~/.sketchddd/templates/`
- **Project templates**: `.sketchddd/templates/`

## Best Practices

### 1. Start with Built-in Templates

Begin with a built-in template and customize:

```bash
sketchddd init my-project --template ecommerce
# Then edit to fit your domain
```

### 2. Create Team Templates

Share common patterns across your organization:

```bash
# Create template
sketchddd template create company-standard
# Edit to match company standards
# Share via git repository
```

### 3. Keep Templates Focused

Templates should be:

- **Complete enough** to be useful
- **Simple enough** to understand
- **Flexible enough** to customize

### 4. Document Your Templates

Include comments in template files:

```sddd
// This template provides the standard e-commerce structure
// used across all Company X projects.
//
// Customize:
// - Add additional payment methods to PaymentType enum
// - Extend Customer with company-specific fields
// - Add your shipping providers

context Commerce {
  // ...
}
```
