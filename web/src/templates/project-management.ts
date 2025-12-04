import type { Template } from './types';

export const projectManagementTemplate: Template = {
  metadata: {
    id: 'project-management',
    name: 'Project Management',
    description: 'A project management domain model with projects, tasks, teams, and sprints.',
    category: 'project-management',
    tags: ['projects', 'tasks', 'teams', 'sprints', 'agile'],
    version: '1.0.0',
  },
  context: {
    name: 'ProjectManagement',
    nodes: {
      'team-entity': {
        node: {
          kind: 'entity',
          name: 'Team',
          fields: [
            { id: 'f1', name: 'id', type: 'UUID', optional: false },
            { id: 'f2', name: 'name', type: 'String', optional: false },
            { id: 'f3', name: 'description', type: 'String', optional: true },
            { id: 'f4', name: 'createdAt', type: 'DateTime', optional: false },
          ],
        },
        position: { x: 50, y: 100 },
      },
      'team-member-entity': {
        node: {
          kind: 'entity',
          name: 'TeamMember',
          fields: [
            { id: 'f5', name: 'id', type: 'UUID', optional: false },
            { id: 'f6', name: 'name', type: 'String', optional: false },
            { id: 'f7', name: 'email', type: 'Email', optional: false },
            { id: 'f8', name: 'avatar', type: 'String', optional: true },
          ],
        },
        position: { x: 50, y: 350 },
      },
      'member-role-enum': {
        node: {
          kind: 'enum',
          name: 'MemberRole',
          variants: [
            { id: 'v1', name: 'Owner' },
            { id: 'v2', name: 'Admin' },
            { id: 'v3', name: 'Developer' },
            { id: 'v4', name: 'Designer' },
            { id: 'v5', name: 'Viewer' },
          ],
        },
        position: { x: 50, y: 550 },
      },
      'project-entity': {
        node: {
          kind: 'entity',
          name: 'Project',
          fields: [
            { id: 'f9', name: 'id', type: 'UUID', optional: false },
            { id: 'f10', name: 'key', type: 'String', optional: false },
            { id: 'f11', name: 'name', type: 'String', optional: false },
            { id: 'f12', name: 'description', type: 'String', optional: true },
            { id: 'f13', name: 'startDate', type: 'DateTime', optional: true },
            { id: 'f14', name: 'targetEndDate', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 300, y: 100 },
      },
      'project-status-enum': {
        node: {
          kind: 'enum',
          name: 'ProjectStatus',
          variants: [
            { id: 'v6', name: 'Planning' },
            { id: 'v7', name: 'Active' },
            { id: 'v8', name: 'OnHold' },
            { id: 'v9', name: 'Completed' },
            { id: 'v10', name: 'Cancelled' },
          ],
        },
        position: { x: 300, y: 350 },
      },
      'sprint-entity': {
        node: {
          kind: 'entity',
          name: 'Sprint',
          fields: [
            { id: 'f15', name: 'id', type: 'UUID', optional: false },
            { id: 'f16', name: 'name', type: 'String', optional: false },
            { id: 'f17', name: 'goal', type: 'String', optional: true },
            { id: 'f18', name: 'startDate', type: 'DateTime', optional: false },
            { id: 'f19', name: 'endDate', type: 'DateTime', optional: false },
            { id: 'f20', name: 'velocity', type: 'Int', optional: true },
          ],
        },
        position: { x: 550, y: 100 },
      },
      'sprint-status-enum': {
        node: {
          kind: 'enum',
          name: 'SprintStatus',
          variants: [
            { id: 'v11', name: 'Planning' },
            { id: 'v12', name: 'Active' },
            { id: 'v13', name: 'Review' },
            { id: 'v14', name: 'Completed' },
          ],
        },
        position: { x: 550, y: 350 },
      },
      'task-entity': {
        node: {
          kind: 'entity',
          name: 'Task',
          fields: [
            { id: 'f21', name: 'id', type: 'UUID', optional: false },
            { id: 'f22', name: 'key', type: 'String', optional: false },
            { id: 'f23', name: 'title', type: 'String', optional: false },
            { id: 'f24', name: 'description', type: 'String', optional: true },
            { id: 'f25', name: 'storyPoints', type: 'Int', optional: true },
            { id: 'f26', name: 'createdAt', type: 'DateTime', optional: false },
            { id: 'f27', name: 'completedAt', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 800, y: 100 },
      },
      'task-status-enum': {
        node: {
          kind: 'enum',
          name: 'TaskStatus',
          variants: [
            { id: 'v15', name: 'Backlog' },
            { id: 'v16', name: 'Todo' },
            { id: 'v17', name: 'InProgress' },
            { id: 'v18', name: 'InReview' },
            { id: 'v19', name: 'Done' },
            { id: 'v20', name: 'Blocked' },
          ],
        },
        position: { x: 1050, y: 100 },
      },
      'task-priority-enum': {
        node: {
          kind: 'enum',
          name: 'TaskPriority',
          variants: [
            { id: 'v21', name: 'Critical' },
            { id: 'v22', name: 'High' },
            { id: 'v23', name: 'Medium' },
            { id: 'v24', name: 'Low' },
          ],
        },
        position: { x: 1050, y: 350 },
      },
      'task-type-enum': {
        node: {
          kind: 'enum',
          name: 'TaskType',
          variants: [
            { id: 'v25', name: 'Story' },
            { id: 'v26', name: 'Bug' },
            { id: 'v27', name: 'Task' },
            { id: 'v28', name: 'Epic' },
            { id: 'v29', name: 'Subtask' },
          ],
        },
        position: { x: 800, y: 350 },
      },
      'comment-entity': {
        node: {
          kind: 'entity',
          name: 'Comment',
          fields: [
            { id: 'f28', name: 'id', type: 'UUID', optional: false },
            { id: 'f29', name: 'content', type: 'String', optional: false },
            { id: 'f30', name: 'createdAt', type: 'DateTime', optional: false },
            { id: 'f31', name: 'updatedAt', type: 'DateTime', optional: true },
          ],
        },
        position: { x: 800, y: 550 },
      },
    },
    morphisms: [
      {
        id: 'm1',
        name: 'members',
        sourceId: 'team-entity',
        targetId: 'team-member-entity',
        cardinality: 'many',
      },
      {
        id: 'm2',
        name: 'role',
        sourceId: 'team-member-entity',
        targetId: 'member-role-enum',
        cardinality: 'one',
      },
      {
        id: 'm3',
        name: 'team',
        sourceId: 'project-entity',
        targetId: 'team-entity',
        cardinality: 'one',
      },
      {
        id: 'm4',
        name: 'status',
        sourceId: 'project-entity',
        targetId: 'project-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm5',
        name: 'lead',
        sourceId: 'project-entity',
        targetId: 'team-member-entity',
        cardinality: 'optional',
      },
      {
        id: 'm6',
        name: 'project',
        sourceId: 'sprint-entity',
        targetId: 'project-entity',
        cardinality: 'one',
      },
      {
        id: 'm7',
        name: 'status',
        sourceId: 'sprint-entity',
        targetId: 'sprint-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm8',
        name: 'project',
        sourceId: 'task-entity',
        targetId: 'project-entity',
        cardinality: 'one',
      },
      {
        id: 'm9',
        name: 'sprint',
        sourceId: 'task-entity',
        targetId: 'sprint-entity',
        cardinality: 'optional',
      },
      {
        id: 'm10',
        name: 'assignee',
        sourceId: 'task-entity',
        targetId: 'team-member-entity',
        cardinality: 'optional',
      },
      {
        id: 'm11',
        name: 'reporter',
        sourceId: 'task-entity',
        targetId: 'team-member-entity',
        cardinality: 'one',
      },
      {
        id: 'm12',
        name: 'status',
        sourceId: 'task-entity',
        targetId: 'task-status-enum',
        cardinality: 'one',
      },
      {
        id: 'm13',
        name: 'priority',
        sourceId: 'task-entity',
        targetId: 'task-priority-enum',
        cardinality: 'one',
      },
      {
        id: 'm14',
        name: 'type',
        sourceId: 'task-entity',
        targetId: 'task-type-enum',
        cardinality: 'one',
      },
      {
        id: 'm15',
        name: 'parent',
        sourceId: 'task-entity',
        targetId: 'task-entity',
        cardinality: 'optional',
      },
      {
        id: 'm16',
        name: 'task',
        sourceId: 'comment-entity',
        targetId: 'task-entity',
        cardinality: 'one',
      },
      {
        id: 'm17',
        name: 'author',
        sourceId: 'comment-entity',
        targetId: 'team-member-entity',
        cardinality: 'one',
      },
    ],
  },
};
