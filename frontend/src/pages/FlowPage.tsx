import { useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import {
  ReactFlow,
  ReactFlowProvider,
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

import { createDeployment, getDeployment, updateDeployment } from '../api/deployments';
import { getDefinitions } from '../api/definitions';
import type { BlockConnection, BlockJunction, DataType, Definition, Deployment, NewDeployment, UpdateDeployment } from '../types';
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
  const { id } = useParams<{ id: string }>();
  const deploymentId = id ? Number(id) : null;
  const isEditing = deploymentId !== null;

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
    let cancelled = false;
    (async () => {
      try {
        const defs = await getDefinitions();
        if (cancelled) return;
        setDefinitions(defs);

        if (deploymentId !== null) {
          const dep = await getDeployment(deploymentId);
          if (cancelled) return;
          setName(dep.name);
          setVersion(dep.version);
          uid = Math.max(uid, ...dep.blocks.map((b) => b.id), 0);
          const { nodes: loadedNodes, edges: loadedEdges } = buildGraphFromDeployment(dep, defs);
          setNodes(loadedNodes);
          setEdges(loadedEdges);
        }
      } catch (e: unknown) {
        if (!cancelled) setError(e instanceof Error ? e.message : 'Failed to load deployment');
      }
    })();
    return () => { cancelled = true; };
  }, [deploymentId]);

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

    const sources = sourceNodes.map((n) => { const d = n.data as SourceData; return { id: d.portId, data_type: d.dataType }; });
    const sinks   = sinkNodes.map((n)   => { const d = n.data as SinkData;   return { id: d.portId, data_type: d.dataType }; });
    const blocks  = blockNodes.map((n)  => { const d = n.data as BlockData;  return { definition_id: d.definitionId, id: d.instanceId }; });

    setLoading(true);
    try {
      if (isEditing && deploymentId !== null) {
        const payload: UpdateDeployment = {
          id: deploymentId,
          name: name.trim(),
          version: version.trim(),
          sources,
          sinks,
          blocks,
          connections,
        };
        await updateDeployment(payload);
      } else {
        const payload: NewDeployment = { name: name.trim(), version: version.trim(), sources, sinks, blocks, connections };
        await createDeployment(payload);
      }
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
            {loading ? (isEditing ? 'Saving…' : 'Creating…') : (isEditing ? 'Save Changes' : 'Create Flow')}
          </button>
        </div>
      </div>

      {error && <div className="error flow-error">{error}</div>}

      {/* Canvas + edit panel */}
      <ReactFlowProvider>
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
              fitViewOptions={{ maxZoom: 1 }}
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
      </ReactFlowProvider>
    </div>
  );
}

function miniMapColor(node: Node) {
  if (node.type === 'source') return '#34d399';
  if (node.type === 'block')  return '#818cf8';
  if (node.type === 'sink')   return '#f472b6';
  return '#64748b';
}

function buildGraphFromDeployment(dep: Deployment, definitions: Definition[]): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = [];
  const sourceIdToNode = new Map<string, string>();
  const sinkIdToNode = new Map<string, string>();
  const blockRefToDef = new Map<string, Definition | undefined>();
  const blockRefToNode = new Map<string, string>();

  dep.sources.forEach((s, i) => {
    const nodeId = `lsrc-${i}`;
    sourceIdToNode.set(s.id, nodeId);
    const data: SourceData = { portId: s.id, dataType: s.data_type };
    nodes.push({ id: nodeId, type: 'source', position: { x: 80, y: 80 + i * 140 }, data });
  });

  dep.blocks.forEach((b, i) => {
    const nodeId = `lblk-${i}`;
    const ref = `${b.definition_id}.${b.id}`;
    blockRefToNode.set(ref, nodeId);
    const def = definitions.find((d) => d.id === b.definition_id);
    blockRefToDef.set(ref, def);
    const body = def?.body as { inputs?: { name: string; data_type: DataType }[]; outputs?: { name: string; data_type: DataType }[] } | undefined;
    const data: BlockData = {
      definitionId: b.definition_id,
      instanceId: b.id,
      definitionName: def?.name ?? `def:${b.definition_id}`,
      inputs: body?.inputs ?? [],
      outputs: body?.outputs ?? [],
    };
    nodes.push({ id: nodeId, type: 'block', position: { x: 420, y: 80 + i * 160 }, data });
  });

  dep.sinks.forEach((s, i) => {
    const nodeId = `lsnk-${i}`;
    sinkIdToNode.set(s.id, nodeId);
    const data: SinkData = { portId: s.id, dataType: s.data_type };
    nodes.push({ id: nodeId, type: 'sink', position: { x: 760, y: 80 + i * 140 }, data });
  });

  // The backend only stores which block instance a connection touches, not the
  // port name — pick the first port of matching data_type as a best-effort guess.
  const resolveEndpoint = (junction: BlockJunction, side: 'from' | 'to'): { node: string; handle: string } | null => {
    if (junction.source) {
      const node = sourceIdToNode.get(junction.source);
      return node ? { node, handle: 'out' } : null;
    }
    if (junction.sink) {
      const node = sinkIdToNode.get(junction.sink);
      return node ? { node, handle: 'in' } : null;
    }
    if (junction.block) {
      const node = blockRefToNode.get(junction.block);
      if (!node) return null;
      const def = blockRefToDef.get(junction.block);
      const body = def?.body as { inputs?: { name: string; data_type: DataType }[]; outputs?: { name: string; data_type: DataType }[] } | undefined;
      const ports = side === 'from' ? body?.outputs ?? [] : body?.inputs ?? [];
      const port = ports.find((p) => p.data_type === junction.data_type) ?? ports[0];
      if (!port) return null;
      return { node, handle: side === 'from' ? `out-${port.name}` : `in-${port.name}` };
    }
    return null;
  };

  const edges: Edge[] = dep.connections.flatMap((conn, i) => {
    const from = resolveEndpoint(conn.from, 'from');
    const to = resolveEndpoint(conn.to, 'to');
    if (!from || !to) return [];
    return [{
      id: `e${i}`,
      source: from.node,
      sourceHandle: from.handle,
      target: to.node,
      targetHandle: to.handle,
      type: 'smoothstep',
      animated: true,
    }];
  });

  return { nodes, edges };
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