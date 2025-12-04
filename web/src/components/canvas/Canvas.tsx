import { useCallback, useMemo } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  Panel,
  useNodesState,
  useEdgesState,
  addEdge,
  BackgroundVariant,
} from '@xyflow/react';
import type { Connection, Node, Edge, NodeTypes } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { useDomainStore } from '@/stores';
import { EntityNode } from '../nodes/EntityNode';
import { ValueNode } from '../nodes/ValueNode';
import { EnumNode } from '../nodes/EnumNode';
import { AggregateNode } from '../nodes/AggregateNode';
import type { NodeKind, DomainNode } from '@/types';

const nodeTypes: NodeTypes = {
  entity: EntityNode,
  value: ValueNode,
  enum: EnumNode,
  aggregate: AggregateNode,
};

export function Canvas() {
  const {
    activeContextId,
    contexts,
    addNode,
    moveNode,
    addMorphism,
    setSelectedNodes,
    setSelectedEdges,
  } = useDomainStore();

  const activeContext = activeContextId ? contexts[activeContextId] : null;

  // Convert domain nodes to React Flow nodes
  const initialNodes = useMemo(() => {
    if (!activeContext) return [];
    return Object.entries(activeContext.nodes).map(([id, { node, position }]) => ({
      id,
      type: node.kind,
      position,
      data: node as unknown as Record<string, unknown>,
    }));
  }, [activeContext]);

  // Convert morphisms to React Flow edges
  const initialEdges = useMemo(() => {
    if (!activeContext) return [];
    return activeContext.morphisms.map((morphism) => ({
      id: morphism.id,
      source: morphism.sourceId,
      target: morphism.targetId,
      label: morphism.name,
      animated: morphism.cardinality === 'many',
      style: {
        strokeDasharray: morphism.cardinality === 'optional' ? '5,5' : undefined,
      },
    }));
  }, [activeContext]);

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // Handle new connections (morphisms)
  const onConnect = useCallback(
    (params: Connection) => {
      if (!activeContextId || !params.source || !params.target) return;

      // Create a new morphism
      const sourceNode = activeContext?.nodes[params.source];
      const targetNode = activeContext?.nodes[params.target];

      if (!sourceNode || !targetNode) return;

      const morphismName = `${sourceNode.node.name.toLowerCase()}To${targetNode.node.name}`;

      addMorphism(activeContextId, {
        name: morphismName,
        sourceId: params.source,
        targetId: params.target,
        cardinality: 'one',
      });

      setEdges((eds) => addEdge(params, eds));
    },
    [activeContextId, activeContext, addMorphism, setEdges]
  );

  // Handle node position changes
  const onNodeDragStop = useCallback(
    (_: React.MouseEvent, node: Node) => {
      if (activeContextId) {
        moveNode(activeContextId, node.id, node.position);
      }
    },
    [activeContextId, moveNode]
  );

  // Handle selection changes
  const onSelectionChange = useCallback(
    ({ nodes, edges }: { nodes: Node[]; edges: Edge[] }) => {
      setSelectedNodes(nodes.map((n) => n.id));
      setSelectedEdges(edges.map((e) => e.id));
    },
    [setSelectedNodes, setSelectedEdges]
  );

  // Handle drop from palette
  const onDrop = useCallback(
    (event: React.DragEvent) => {
      event.preventDefault();

      if (!activeContextId) return;

      const kind = event.dataTransfer.getData('application/sketchddd-node') as NodeKind;
      if (!kind) return;

      // Get drop position relative to the canvas
      const reactFlowBounds = event.currentTarget.getBoundingClientRect();
      const position = {
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      };

      let node: DomainNode;
      const name = `New${kind.charAt(0).toUpperCase() + kind.slice(1)}`;

      switch (kind) {
        case 'entity':
          node = {
            kind: 'entity',
            name,
            fields: [{ id: crypto.randomUUID(), name: 'id', type: 'UUID', optional: false }],
          };
          break;
        case 'value':
          node = { kind: 'value', name, fields: [] };
          break;
        case 'enum':
          node = {
            kind: 'enum',
            name,
            variants: [{ id: crypto.randomUUID(), name: 'Variant1' }],
          };
          break;
        case 'aggregate':
          node = { kind: 'aggregate', name, rootId: '', memberIds: [], invariants: [] };
          break;
        default:
          return;
      }

      const nodeId = addNode(activeContextId, node, position);
      setNodes((nds) => [
        ...nds,
        {
          id: nodeId,
          type: kind,
          position,
          data: node as unknown as Record<string, unknown>,
        },
      ]);
    },
    [activeContextId, addNode, setNodes]
  );

  const onDragOver = useCallback((event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
  }, []);

  if (!activeContext) {
    return (
      <div className="flex-1 flex items-center justify-center bg-slate-100 dark:bg-slate-900">
        <div className="text-center text-slate-500 dark:text-slate-400">
          <p className="text-lg mb-2">No context selected</p>
          <p className="text-sm">Select or create a bounded context to start modeling</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 h-full" onDrop={onDrop} onDragOver={onDragOver}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeDragStop={onNodeDragStop}
        onSelectionChange={onSelectionChange}
        nodeTypes={nodeTypes}
        fitView
        snapToGrid
        snapGrid={[16, 16]}
        defaultEdgeOptions={{
          type: 'smoothstep',
        }}
      >
        <Background variant={BackgroundVariant.Dots} gap={16} size={1} />
        <Controls />
        <MiniMap
          nodeStrokeWidth={3}
          zoomable
          pannable
          className="bg-white dark:bg-slate-800"
        />
        <Panel position="top-left" className="bg-white/80 dark:bg-slate-800/80 backdrop-blur px-3 py-1.5 rounded shadow">
          <span className="text-sm font-medium">{activeContext.name}</span>
        </Panel>
      </ReactFlow>
    </div>
  );
}
