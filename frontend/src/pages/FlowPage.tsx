import { useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  addEdge,
  useNodesState,
  useEdgesState,
  type Connection,
  type Edge,
  type Node,
  type NodeTypes,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { createDeployment } from '../api/deployments';
import { getDefinitions } from '../api/definitions';
import type { BlockConnection, DataType, Definition, NewDeployment } from '../types';
import {
  SourceNodeComponent,
  BlockNodeComponent,
  SinkNodeComponent,
  type BlockData,
  type SinkData,
  type SourceData,
} from '../components/flow/nodes';
import { EditPanel } from '../components/flow/EditPanel';

// nodeTypes must be stable (defined outside the component)
const nodeTypes: NodeTypes = {
  source: SourceNodeComponent,
  block: BlockNodeComponent,
  sink: SinkNodeComponent,
};

let uid = 0;
const nextId = (prefix: string) => `${prefix}-${++uid}`;

export default function FlowPage() {
  const navigate = useNavigate();
  const [definitions, setDefinitions] = useState<Definition[]>([]);
  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [name, setName] = useState('');
  const [version, setVersion] = useState('1.0.0');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const nextPos = useRef({ x: 80, y: 80 });

  useEffect(() => {
    getDefinitions().then(setDefinitions).catch(() => {});
  }, []);

  // Keep selectedNode in sync with nodes array (data may have changed via updateNodeData)
  useEffect(() => {
    if (!selectedNode) return;
    const updated = nodes.find((n) => n.id === selectedNode.id);
    if (updated) setSelectedNode(updated);
  }, [nodes]);

  const onConnect = useCallback(
    (params: Connection) =>
      setEdges((eds) => addEdge({ ...params, type: 'smoothstep', animated: true }, eds)),
    [],
  );

  const spawnPos = () => {
    const p = { ...nextPos.current };
    nextPos.current = { x: p.x + 20, y: p.y + 40 };
    return p;
  };

  const addSource = () => {
    const id = nextId('src');
    const data: SourceData = { portId: id, dataType: 'Text' };
    setNodes((ns) => [...ns, { id, type: 'source', position: spawnPos(), data }]);
  };

  const addSink = () => {
    const id = nextId('snk');
    const data: SinkData = { portId: id, dataType: 'Text' };
    setNodes((ns) => [...ns, { id, type: 'sink', position: { x: spawnPos().x + 500, y: spawnPos().y }, data }]);
  };

  const addBlock = () => {
    if (definitions.length === 0) { setError('No definitions available'); return; }
    const def = definitions[0];
    const id = nextId('blk');
    const body = def.body as { inputs?: { name: string; data_type: DataType }[]; outputs?: { name: string; data_type: DataType }[] };
    const data: BlockData = {
      definitionId: def.id,
      instanceId: uid,
      definitionName: def.name,
      inputs: body.inputs ?? [],
      outputs: body.outputs ?? [],
    };
    setNodes((ns) => [...ns, { id, type: 'block', position: { x: spawnPos().x + 260, y: spawnPos().y }, data }]);
  };

  const handleSubmit = async () => {
    setError(null);
    if (!name.trim()) { setError('Name is required'); return; }

    const sourceNodes = nodes.filter((n) => n.type === 'source');
    const sinkNodes   = nodes.filter((n) => n.type === 'sink');
    const blockNodes  = nodes.filter((n) => n.type === 'block');

    const nodeMap = new Map(nodes.map((n) => [n.id, n]));

    const connections: BlockConnection[] = edges.flatMap((edge) => {
      const src = nodeMap.get(edge.source);
      const tgt = nodeMap.get(edge.target);
      if (!src || !tgt) return [];

      const from = buildJunction(src, edge.sourceHandle ?? 'out');
      const to   = buildJunction(tgt, edge.targetHandle ?? 'in');
      if (!from || !to) return [];

      // data_type from the source side
      return [{ from: { ...from, data_type: from.data_type }, to: { ...to, data_type: from.data_type } }];
    });

    const payload: NewDeployment = {
      name: name.trim(),
      version: version.trim(),
      sources: sourceNodes.map((n) => { const d = n.data as SourceData; return { id: d.portId, data_type: d.dataType }; }),
      sinks:   sinkNodes.map((n)   => { const d = n.data as SinkData;   return { id: d.portId, data_type: d.dataType }; }),
      blocks:  blockNodes.map((n)  => { const d = n.data as BlockData;  return { definition_id: d.definitionId, id: d.instanceId }; }),
      connections,
    };

    setLoading(true);
    try {
      await createDeployment(payload);
      navigate('/deployments');
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flow-page">
      {/* Top bar */}
      <div className="flow-topbar">
        <div className="flow-topbar-meta">
          <input
            className="flow-name-input"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Flow name"
          />
          <input
            className="flow-version-input"
            value={version}
            onChange={(e) => setVersion(e.target.value)}
            placeholder="1.0.0"
          />
        </div>
        <div className="flow-topbar-actions">
          <button className="btn-secondary" onClick={addSource}>+ Source</button>
          <button className="btn-secondary" onClick={addBlock} disabled={definitions.length === 0}>+ Block</button>
          <button className="btn-secondary" onClick={addSink}>+ Sink</button>
          <div className="flow-topbar-sep" />
          <button className="btn-secondary" onClick={() => navigate('/deployments')}>Cancel</button>
          <button className="btn-primary" onClick={handleSubmit} disabled={loading}>
            {loading ? 'Creating…' : 'Create Flow'}
          </button>
        </div>
      </div>

      {error && <div className="error flow-error">{error}</div>}

      {/* Canvas + edit panel */}
      <div className="flow-workspace">
        <div className="flow-canvas-area">
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            nodeTypes={nodeTypes}
            onNodeClick={(_, node) => setSelectedNode(node)}
            onPaneClick={() => setSelectedNode(null)}
            fitView
            deleteKeyCode="Delete"
          >
            <Background gap={16} color="#1e2130" />
            <Controls />
            <MiniMap nodeColor={miniMapColor} maskColor="rgba(15,17,23,0.6)" style={{ background: '#1a1d27' }} />
          </ReactFlow>
        </div>

        {selectedNode && (
          <EditPanel
            node={selectedNode}
            definitions={definitions}
            onClose={() => setSelectedNode(null)}
          />
        )}
      </div>
    </div>
  );
}

function miniMapColor(node: Node) {
  if (node.type === 'source') return '#34d399';
  if (node.type === 'block')  return '#818cf8';
  if (node.type === 'sink')   return '#f472b6';
  return '#64748b';
}

function buildJunction(node: Node, handle: string) {
  if (node.type === 'source') {
    const d = node.data as SourceData;
    return { source: d.portId, block: null, sink: null, data_type: d.dataType };
  }
  if (node.type === 'sink') {
    const d = node.data as SinkData;
    return { source: null, block: null, sink: d.portId, data_type: d.dataType };
  }
  if (node.type === 'block') {
    const d = node.data as BlockData;
    const ref = `${d.definitionId}.${d.instanceId}`;
    if (handle.startsWith('out-')) {
      const portName = handle.slice(4);
      const port = d.outputs.find((o) => o.name === portName);
      return { block: ref, source: null, sink: null, data_type: port?.data_type ?? 'Text' };
    }
    if (handle.startsWith('in-')) {
      const portName = handle.slice(3);
      const port = d.inputs.find((i) => i.name === portName);
      return { block: ref, source: null, sink: null, data_type: port?.data_type ?? 'Text' };
    }
  }
  return null;
}