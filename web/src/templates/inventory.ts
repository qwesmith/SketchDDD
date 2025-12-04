import type { Template } from './types';

export const inventoryTemplate: Template = {
  metadata: {
    id: 'inventory',
    name: 'Inventory Management',
    description: 'An inventory management domain model with products, warehouses, stock moves, and suppliers.',
    category: 'inventory',
    tags: ['products', 'warehouses', 'stock', 'suppliers', 'logistics'],
    version: '1.0.0',
  },
  context: {
    name: 'Inventory',
    nodes: {
      'product-entity': {
        node: {
          kind: 'entity',
          name: 'Product',
          fields: [
            { id: 'f1', name: 'id', type: 'UUID', optional: false },
            { id: 'f2', name: 'sku', type: 'String', optional: false },
            { id: 'f3', name: 'name', type: 'String', optional: false },
            { id: 'f4', name: 'description', type: 'String', optional: true },
            { id: 'f5', name: 'reorderPoint', type: 'Int', optional: false },
            { id: 'f6', name: 'reorderQuantity', type: 'Int', optional: false },
          ],
        },
        position: { x: 50, y: 100 },
      },
      'product-category-entity': {
        node: {
          kind: 'entity',
          name: 'ProductCategory',
          fields: [
            { id: 'f7', name: 'id', type: 'UUID', optional: false },
            { id: 'f8', name: 'code', type: 'String', optional: false },
            { id: 'f9', name: 'name', type: 'String', optional: false },
          ],
        },
        position: { x: 50, y: 350 },
      },
      'warehouse-entity': {
        node: {
          kind: 'entity',
          name: 'Warehouse',
          fields: [
            { id: 'f10', name: 'id', type: 'UUID', optional: false },
            { id: 'f11', name: 'code', type: 'String', optional: false },
            { id: 'f12', name: 'name', type: 'String', optional: false },
            { id: 'f13', name: 'capacity', type: 'Int', optional: true },
          ],
        },
        position: { x: 300, y: 100 },
      },
      'location-value': {
        node: {
          kind: 'value',
          name: 'Location',
          fields: [
            { id: 'f14', name: 'aisle', type: 'String', optional: false },
            { id: 'f15', name: 'rack', type: 'String', optional: false },
            { id: 'f16', name: 'shelf', type: 'String', optional: false },
            { id: 'f17', name: 'bin', type: 'String', optional: true },
          ],
        },
        position: { x: 300, y: 350 },
      },
      'stock-level-entity': {
        node: {
          kind: 'entity',
          name: 'StockLevel',
          fields: [
            { id: 'f18', name: 'id', type: 'UUID', optional: false },
            { id: 'f19', name: 'quantity', type: 'Int', optional: false },
            { id: 'f20', name: 'reservedQuantity', type: 'Int', optional: false },
            { id: 'f21', name: 'lastUpdated', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 550, y: 100 },
      },
      'stock-move-entity': {
        node: {
          kind: 'entity',
          name: 'StockMove',
          fields: [
            { id: 'f22', name: 'id', type: 'UUID', optional: false },
            { id: 'f23', name: 'reference', type: 'String', optional: false },
            { id: 'f24', name: 'quantity', type: 'Int', optional: false },
            { id: 'f25', name: 'movedAt', type: 'DateTime', optional: false },
            { id: 'f26', name: 'notes', type: 'String', optional: true },
          ],
        },
        position: { x: 550, y: 350 },
      },
      'stock-move-type-enum': {
        node: {
          kind: 'enum',
          name: 'StockMoveType',
          variants: [
            { id: 'v1', name: 'Receive' },
            { id: 'v2', name: 'Ship' },
            { id: 'v3', name: 'Transfer' },
            { id: 'v4', name: 'Adjustment' },
            { id: 'v5', name: 'Return' },
            { id: 'v6', name: 'Scrap' },
          ],
        },
        position: { x: 800, y: 350 },
      },
      'supplier-entity': {
        node: {
          kind: 'entity',
          name: 'Supplier',
          fields: [
            { id: 'f27', name: 'id', type: 'UUID', optional: false },
            { id: 'f28', name: 'code', type: 'String', optional: false },
            { id: 'f29', name: 'name', type: 'String', optional: false },
            { id: 'f30', name: 'email', type: 'Email', optional: true },
            { id: 'f31', name: 'leadTimeDays', type: 'Int', optional: true },
          ],
        },
        position: { x: 800, y: 100 },
      },
      'purchase-order-entity': {
        node: {
          kind: 'entity',
          name: 'PurchaseOrder',
          fields: [
            { id: 'f32', name: 'id', type: 'UUID', optional: false },
            { id: 'f33', name: 'orderNumber', type: 'String', optional: false },
            { id: 'f34', name: 'orderedAt', type: 'DateTime', optional: false },
            { id: 'f35', name: 'expectedDelivery', type: 'DateTime', optional: true },
            { id: 'f36', name: 'totalAmount', type: 'Decimal', optional: false },
          ],
        },
        position: { x: 1050, y: 100 },
      },
      'po-status-enum': {
        node: {
          kind: 'enum',
          name: 'PurchaseOrderStatus',
          variants: [
            { id: 'v7', name: 'Draft' },
            { id: 'v8', name: 'Submitted' },
            { id: 'v9', name: 'Confirmed' },
            { id: 'v10', name: 'PartiallyReceived' },
            { id: 'v11', name: 'Received' },
            { id: 'v12', name: 'Cancelled' },
          ],
        },
        position: { x: 1050, y: 350 },
      },
    },
    morphisms: [
      {
        id: 'm1',
        name: 'category',
        sourceId: 'product-entity',
        targetId: 'product-category-entity',
        cardinality: 'optional',
      },
      {
        id: 'm2',
        name: 'address',
        sourceId: 'warehouse-entity',
        targetId: 'location-value',
        cardinality: 'one',
      },
      {
        id: 'm3',
        name: 'product',
        sourceId: 'stock-level-entity',
        targetId: 'product-entity',
        cardinality: 'one',
      },
      {
        id: 'm4',
        name: 'warehouse',
        sourceId: 'stock-level-entity',
        targetId: 'warehouse-entity',
        cardinality: 'one',
      },
      {
        id: 'm5',
        name: 'location',
        sourceId: 'stock-level-entity',
        targetId: 'location-value',
        cardinality: 'optional',
      },
      {
        id: 'm6',
        name: 'product',
        sourceId: 'stock-move-entity',
        targetId: 'product-entity',
        cardinality: 'one',
      },
      {
        id: 'm7',
        name: 'sourceWarehouse',
        sourceId: 'stock-move-entity',
        targetId: 'warehouse-entity',
        cardinality: 'optional',
      },
      {
        id: 'm8',
        name: 'destinationWarehouse',
        sourceId: 'stock-move-entity',
        targetId: 'warehouse-entity',
        cardinality: 'optional',
      },
      {
        id: 'm9',
        name: 'type',
        sourceId: 'stock-move-entity',
        targetId: 'stock-move-type-enum',
        cardinality: 'one',
      },
      {
        id: 'm10',
        name: 'preferredSupplier',
        sourceId: 'product-entity',
        targetId: 'supplier-entity',
        cardinality: 'optional',
      },
      {
        id: 'm11',
        name: 'supplier',
        sourceId: 'purchase-order-entity',
        targetId: 'supplier-entity',
        cardinality: 'one',
      },
      {
        id: 'm12',
        name: 'status',
        sourceId: 'purchase-order-entity',
        targetId: 'po-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm13',
        name: 'deliverTo',
        sourceId: 'purchase-order-entity',
        targetId: 'warehouse-entity',
        cardinality: 'one',
      },
    ],
  },
};
