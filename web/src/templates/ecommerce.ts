import type { Template } from './types';

export const ecommerceTemplate: Template = {
  metadata: {
    id: 'ecommerce',
    name: 'E-Commerce',
    description: 'A complete e-commerce domain model with orders, products, customers, and payments.',
    category: 'ecommerce',
    tags: ['orders', 'products', 'customers', 'payments', 'retail'],
    version: '1.0.0',
  },
  context: {
    name: 'E-Commerce',
    nodes: {
      'customer-entity': {
        node: {
          kind: 'entity',
          name: 'Customer',
          fields: [
            { id: 'f1', name: 'id', type: 'UUID', optional: false },
            { id: 'f2', name: 'email', type: 'Email', optional: false },
            { id: 'f3', name: 'name', type: 'String', optional: false },
            { id: 'f4', name: 'createdAt', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 50, y: 100 },
      },
      'address-value': {
        node: {
          kind: 'value',
          name: 'Address',
          fields: [
            { id: 'f5', name: 'street', type: 'String', optional: false },
            { id: 'f6', name: 'city', type: 'String', optional: false },
            { id: 'f7', name: 'postalCode', type: 'String', optional: false },
            { id: 'f8', name: 'country', type: 'String', optional: false },
          ],
        },
        position: { x: 50, y: 300 },
      },
      'product-entity': {
        node: {
          kind: 'entity',
          name: 'Product',
          fields: [
            { id: 'f9', name: 'id', type: 'UUID', optional: false },
            { id: 'f10', name: 'sku', type: 'String', optional: false },
            { id: 'f11', name: 'name', type: 'String', optional: false },
            { id: 'f12', name: 'description', type: 'String', optional: true },
            { id: 'f13', name: 'price', type: 'Decimal', optional: false },
          ],
        },
        position: { x: 300, y: 100 },
      },
      'money-value': {
        node: {
          kind: 'value',
          name: 'Money',
          fields: [
            { id: 'f14', name: 'amount', type: 'Decimal', optional: false },
            { id: 'f15', name: 'currency', type: 'String', optional: false },
          ],
        },
        position: { x: 300, y: 300 },
      },
      'order-entity': {
        node: {
          kind: 'entity',
          name: 'Order',
          fields: [
            { id: 'f16', name: 'id', type: 'UUID', optional: false },
            { id: 'f17', name: 'orderNumber', type: 'String', optional: false },
            { id: 'f18', name: 'placedAt', type: 'DateTime', optional: false },
            { id: 'f19', name: 'totalAmount', type: 'Decimal', optional: false },
          ],
        },
        position: { x: 550, y: 100 },
      },
      'line-item-entity': {
        node: {
          kind: 'entity',
          name: 'LineItem',
          fields: [
            { id: 'f20', name: 'id', type: 'UUID', optional: false },
            { id: 'f21', name: 'quantity', type: 'Int', optional: false },
            { id: 'f22', name: 'unitPrice', type: 'Decimal', optional: false },
          ],
        },
        position: { x: 550, y: 300 },
      },
      'order-status-enum': {
        node: {
          kind: 'enum',
          name: 'OrderStatus',
          variants: [
            { id: 'v1', name: 'Pending' },
            { id: 'v2', name: 'Confirmed' },
            { id: 'v3', name: 'Shipped' },
            { id: 'v4', name: 'Delivered' },
            { id: 'v5', name: 'Cancelled' },
          ],
        },
        position: { x: 800, y: 100 },
      },
      'payment-entity': {
        node: {
          kind: 'entity',
          name: 'Payment',
          fields: [
            { id: 'f23', name: 'id', type: 'UUID', optional: false },
            { id: 'f24', name: 'amount', type: 'Decimal', optional: false },
            { id: 'f25', name: 'processedAt', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 800, y: 300 },
      },
      'payment-status-enum': {
        node: {
          kind: 'enum',
          name: 'PaymentStatus',
          variants: [
            { id: 'v6', name: 'Pending' },
            { id: 'v7', name: 'Processing' },
            { id: 'v8', name: 'Completed' },
            { id: 'v9', name: 'Failed' },
            { id: 'v10', name: 'Refunded' },
          ],
        },
        position: { x: 1050, y: 100 },
      },
      'order-aggregate': {
        node: {
          kind: 'aggregate',
          name: 'OrderAggregate',
          rootId: 'order-entity',
          memberIds: ['line-item-entity'],
          invariants: [
            'totalAmount = sum(lineItems.quantity * lineItems.unitPrice)',
            'lineItems.quantity > 0',
          ],
        },
        position: { x: 550, y: 500 },
      },
    },
    morphisms: [
      {
        id: 'm1',
        name: 'shippingAddress',
        sourceId: 'customer-entity',
        targetId: 'address-value',
        cardinality: 'optional',
      },
      {
        id: 'm2',
        name: 'billingAddress',
        sourceId: 'customer-entity',
        targetId: 'address-value',
        cardinality: 'one',
      },
      {
        id: 'm3',
        name: 'price',
        sourceId: 'product-entity',
        targetId: 'money-value',
        cardinality: 'one',
      },
      {
        id: 'm4',
        name: 'customer',
        sourceId: 'order-entity',
        targetId: 'customer-entity',
        cardinality: 'one',
      },
      {
        id: 'm5',
        name: 'lineItems',
        sourceId: 'order-entity',
        targetId: 'line-item-entity',
        cardinality: 'many',
      },
      {
        id: 'm6',
        name: 'product',
        sourceId: 'line-item-entity',
        targetId: 'product-entity',
        cardinality: 'one',
      },
      {
        id: 'm7',
        name: 'status',
        sourceId: 'order-entity',
        targetId: 'order-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm8',
        name: 'payments',
        sourceId: 'order-entity',
        targetId: 'payment-entity',
        cardinality: 'many',
      },
      {
        id: 'm9',
        name: 'status',
        sourceId: 'payment-entity',
        targetId: 'payment-status-enum',
        cardinality: 'one',
      },
    ],
  },
};
