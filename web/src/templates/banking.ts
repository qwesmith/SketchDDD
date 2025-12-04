import type { Template } from './types';

export const bankingTemplate: Template = {
  metadata: {
    id: 'banking',
    name: 'Banking',
    description: 'A banking domain model with accounts, transactions, transfers, and statements.',
    category: 'banking',
    tags: ['accounts', 'transactions', 'transfers', 'finance', 'banking'],
    version: '1.0.0',
  },
  context: {
    name: 'Banking',
    nodes: {
      'customer-entity': {
        node: {
          kind: 'entity',
          name: 'Customer',
          fields: [
            { id: 'f1', name: 'id', type: 'UUID', optional: false },
            { id: 'f2', name: 'customerNumber', type: 'String', optional: false },
            { id: 'f3', name: 'name', type: 'String', optional: false },
            { id: 'f4', name: 'email', type: 'Email', optional: false },
            { id: 'f5', name: 'phone', type: 'String', optional: true },
            { id: 'f6', name: 'joinedAt', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 50, y: 100 },
      },
      'account-entity': {
        node: {
          kind: 'entity',
          name: 'Account',
          fields: [
            { id: 'f7', name: 'id', type: 'UUID', optional: false },
            { id: 'f8', name: 'accountNumber', type: 'String', optional: false },
            { id: 'f9', name: 'balance', type: 'Decimal', optional: false },
            { id: 'f10', name: 'openedAt', type: 'DateTime', optional: false },
            { id: 'f11', name: 'closedAt', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 300, y: 100 },
      },
      'account-type-enum': {
        node: {
          kind: 'enum',
          name: 'AccountType',
          variants: [
            { id: 'v1', name: 'Checking' },
            { id: 'v2', name: 'Savings' },
            { id: 'v3', name: 'MoneyMarket' },
            { id: 'v4', name: 'CertificateOfDeposit' },
          ],
        },
        position: { x: 300, y: 350 },
      },
      'account-status-enum': {
        node: {
          kind: 'enum',
          name: 'AccountStatus',
          variants: [
            { id: 'v5', name: 'Active' },
            { id: 'v6', name: 'Frozen' },
            { id: 'v7', name: 'Closed' },
            { id: 'v8', name: 'Dormant' },
          ],
        },
        position: { x: 50, y: 350 },
      },
      'transaction-entity': {
        node: {
          kind: 'entity',
          name: 'Transaction',
          fields: [
            { id: 'f12', name: 'id', type: 'UUID', optional: false },
            { id: 'f13', name: 'referenceNumber', type: 'String', optional: false },
            { id: 'f14', name: 'amount', type: 'Decimal', optional: false },
            { id: 'f15', name: 'description', type: 'String', optional: true },
            { id: 'f16', name: 'processedAt', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 550, y: 100 },
      },
      'transaction-type-enum': {
        node: {
          kind: 'enum',
          name: 'TransactionType',
          variants: [
            { id: 'v9', name: 'Deposit' },
            { id: 'v10', name: 'Withdrawal' },
            { id: 'v11', name: 'Transfer' },
            { id: 'v12', name: 'Fee' },
            { id: 'v13', name: 'Interest' },
            { id: 'v14', name: 'Adjustment' },
          ],
        },
        position: { x: 550, y: 350 },
      },
      'transfer-entity': {
        node: {
          kind: 'entity',
          name: 'Transfer',
          fields: [
            { id: 'f17', name: 'id', type: 'UUID', optional: false },
            { id: 'f18', name: 'amount', type: 'Decimal', optional: false },
            { id: 'f19', name: 'memo', type: 'String', optional: true },
            { id: 'f20', name: 'scheduledFor', type: 'DateTime', optional: true },
            { id: 'f21', name: 'completedAt', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 800, y: 100 },
      },
      'transfer-status-enum': {
        node: {
          kind: 'enum',
          name: 'TransferStatus',
          variants: [
            { id: 'v15', name: 'Pending' },
            { id: 'v16', name: 'Processing' },
            { id: 'v17', name: 'Completed' },
            { id: 'v18', name: 'Failed' },
            { id: 'v19', name: 'Cancelled' },
          ],
        },
        position: { x: 800, y: 350 },
      },
      'statement-entity': {
        node: {
          kind: 'entity',
          name: 'Statement',
          fields: [
            { id: 'f22', name: 'id', type: 'UUID', optional: false },
            { id: 'f23', name: 'periodStart', type: 'DateTime', optional: false },
            { id: 'f24', name: 'periodEnd', type: 'DateTime', optional: false },
            { id: 'f25', name: 'openingBalance', type: 'Decimal', optional: false },
            { id: 'f26', name: 'closingBalance', type: 'Decimal', optional: false },
            { id: 'f27', name: 'generatedAt', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 1050, y: 100 },
      },
      'money-value': {
        node: {
          kind: 'value',
          name: 'Money',
          fields: [
            { id: 'f28', name: 'amount', type: 'Decimal', optional: false },
            { id: 'f29', name: 'currency', type: 'String', optional: false },
          ],
        },
        position: { x: 1050, y: 350 },
      },
      'account-aggregate': {
        node: {
          kind: 'aggregate',
          name: 'AccountAggregate',
          rootId: 'account-entity',
          memberIds: ['transaction-entity'],
          invariants: [
            'balance = openingBalance + sum(transactions.amount)',
            'balance >= 0 OR accountType = Checking',
          ],
        },
        position: { x: 400, y: 500 },
      },
    },
    morphisms: [
      {
        id: 'm1',
        name: 'accounts',
        sourceId: 'customer-entity',
        targetId: 'account-entity',
        cardinality: 'many',
      },
      {
        id: 'm2',
        name: 'accountType',
        sourceId: 'account-entity',
        targetId: 'account-type-enum',
        cardinality: 'one',
      },
      {
        id: 'm3',
        name: 'status',
        sourceId: 'account-entity',
        targetId: 'account-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm4',
        name: 'account',
        sourceId: 'transaction-entity',
        targetId: 'account-entity',
        cardinality: 'one',
      },
      {
        id: 'm5',
        name: 'type',
        sourceId: 'transaction-entity',
        targetId: 'transaction-type-enum',
        cardinality: 'one',
      },
      {
        id: 'm6',
        name: 'sourceAccount',
        sourceId: 'transfer-entity',
        targetId: 'account-entity',
        cardinality: 'one',
      },
      {
        id: 'm7',
        name: 'destinationAccount',
        sourceId: 'transfer-entity',
        targetId: 'account-entity',
        cardinality: 'one',
      },
      {
        id: 'm8',
        name: 'status',
        sourceId: 'transfer-entity',
        targetId: 'transfer-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm9',
        name: 'sourceTransaction',
        sourceId: 'transfer-entity',
        targetId: 'transaction-entity',
        cardinality: 'optional',
      },
      {
        id: 'm10',
        name: 'destinationTransaction',
        sourceId: 'transfer-entity',
        targetId: 'transaction-entity',
        cardinality: 'optional',
      },
      {
        id: 'm11',
        name: 'account',
        sourceId: 'statement-entity',
        targetId: 'account-entity',
        cardinality: 'one',
      },
      {
        id: 'm12',
        name: 'transactions',
        sourceId: 'statement-entity',
        targetId: 'transaction-entity',
        cardinality: 'many',
      },
    ],
  },
};
