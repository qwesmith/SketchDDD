import type { Template } from './types';

export const healthcareTemplate: Template = {
  metadata: {
    id: 'healthcare',
    name: 'Healthcare',
    description: 'A healthcare domain model with patients, appointments, treatments, and billing.',
    category: 'healthcare',
    tags: ['patients', 'appointments', 'medical', 'billing', 'healthcare'],
    version: '1.0.0',
  },
  context: {
    name: 'Healthcare',
    nodes: {
      'patient-entity': {
        node: {
          kind: 'entity',
          name: 'Patient',
          fields: [
            { id: 'f1', name: 'id', type: 'UUID', optional: false },
            { id: 'f2', name: 'medicalRecordNumber', type: 'String', optional: false },
            { id: 'f3', name: 'firstName', type: 'String', optional: false },
            { id: 'f4', name: 'lastName', type: 'String', optional: false },
            { id: 'f5', name: 'dateOfBirth', type: 'DateTime', optional: false },
            { id: 'f6', name: 'email', type: 'Email', optional: true },
          ],
        },
        position: { x: 50, y: 100 },
      },
      'contact-info-value': {
        node: {
          kind: 'value',
          name: 'ContactInfo',
          fields: [
            { id: 'f7', name: 'phone', type: 'String', optional: false },
            { id: 'f8', name: 'emergencyContact', type: 'String', optional: true },
            { id: 'f9', name: 'emergencyPhone', type: 'String', optional: true },
          ],
        },
        position: { x: 50, y: 350 },
      },
      'doctor-entity': {
        node: {
          kind: 'entity',
          name: 'Doctor',
          fields: [
            { id: 'f10', name: 'id', type: 'UUID', optional: false },
            { id: 'f11', name: 'licenseNumber', type: 'String', optional: false },
            { id: 'f12', name: 'name', type: 'String', optional: false },
            { id: 'f13', name: 'specialty', type: 'String', optional: false },
          ],
        },
        position: { x: 300, y: 100 },
      },
      'appointment-entity': {
        node: {
          kind: 'entity',
          name: 'Appointment',
          fields: [
            { id: 'f14', name: 'id', type: 'UUID', optional: false },
            { id: 'f15', name: 'scheduledAt', type: 'DateTime', optional: false },
            { id: 'f16', name: 'duration', type: 'Int', optional: false },
            { id: 'f17', name: 'reason', type: 'String', optional: true },
          ],
        },
        position: { x: 550, y: 100 },
      },
      'appointment-status-enum': {
        node: {
          kind: 'enum',
          name: 'AppointmentStatus',
          variants: [
            { id: 'v1', name: 'Scheduled' },
            { id: 'v2', name: 'CheckedIn' },
            { id: 'v3', name: 'InProgress' },
            { id: 'v4', name: 'Completed' },
            { id: 'v5', name: 'Cancelled' },
            { id: 'v6', name: 'NoShow' },
          ],
        },
        position: { x: 800, y: 100 },
      },
      'treatment-entity': {
        node: {
          kind: 'entity',
          name: 'Treatment',
          fields: [
            { id: 'f18', name: 'id', type: 'UUID', optional: false },
            { id: 'f19', name: 'code', type: 'String', optional: false },
            { id: 'f20', name: 'name', type: 'String', optional: false },
            { id: 'f21', name: 'administeredAt', type: 'DateTime', optional: false },
            { id: 'f22', name: 'notes', type: 'String', optional: true },
          ],
        },
        position: { x: 550, y: 350 },
      },
      'diagnosis-value': {
        node: {
          kind: 'value',
          name: 'Diagnosis',
          fields: [
            { id: 'f23', name: 'icdCode', type: 'String', optional: false },
            { id: 'f24', name: 'description', type: 'String', optional: false },
            { id: 'f25', name: 'diagnosedAt', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 300, y: 350 },
      },
      'invoice-entity': {
        node: {
          kind: 'entity',
          name: 'Invoice',
          fields: [
            { id: 'f26', name: 'id', type: 'UUID', optional: false },
            { id: 'f27', name: 'invoiceNumber', type: 'String', optional: false },
            { id: 'f28', name: 'issuedAt', type: 'DateTime', optional: false },
            { id: 'f29', name: 'totalAmount', type: 'Decimal', optional: false },
            { id: 'f30', name: 'paidAt', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 800, y: 350 },
      },
      'invoice-status-enum': {
        node: {
          kind: 'enum',
          name: 'InvoiceStatus',
          variants: [
            { id: 'v7', name: 'Draft' },
            { id: 'v8', name: 'Issued' },
            { id: 'v9', name: 'Paid' },
            { id: 'v10', name: 'Overdue' },
            { id: 'v11', name: 'Cancelled' },
          ],
        },
        position: { x: 1050, y: 350 },
      },
    },
    morphisms: [
      {
        id: 'm1',
        name: 'contactInfo',
        sourceId: 'patient-entity',
        targetId: 'contact-info-value',
        cardinality: 'one',
      },
      {
        id: 'm2',
        name: 'patient',
        sourceId: 'appointment-entity',
        targetId: 'patient-entity',
        cardinality: 'one',
      },
      {
        id: 'm3',
        name: 'doctor',
        sourceId: 'appointment-entity',
        targetId: 'doctor-entity',
        cardinality: 'one',
      },
      {
        id: 'm4',
        name: 'status',
        sourceId: 'appointment-entity',
        targetId: 'appointment-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm5',
        name: 'treatments',
        sourceId: 'appointment-entity',
        targetId: 'treatment-entity',
        cardinality: 'many',
      },
      {
        id: 'm6',
        name: 'diagnoses',
        sourceId: 'patient-entity',
        targetId: 'diagnosis-value',
        cardinality: 'many',
      },
      {
        id: 'm7',
        name: 'administeredBy',
        sourceId: 'treatment-entity',
        targetId: 'doctor-entity',
        cardinality: 'one',
      },
      {
        id: 'm8',
        name: 'patient',
        sourceId: 'invoice-entity',
        targetId: 'patient-entity',
        cardinality: 'one',
      },
      {
        id: 'm9',
        name: 'status',
        sourceId: 'invoice-entity',
        targetId: 'invoice-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm10',
        name: 'appointment',
        sourceId: 'invoice-entity',
        targetId: 'appointment-entity',
        cardinality: 'optional',
      },
    ],
  },
};
