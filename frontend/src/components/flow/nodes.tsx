import { Handle, Position, useReactFlow, type Node, type NodeProps } from '@xyflow/react';
import type { BlockPort, DataType } from '../../types';

// ─── Source ──────────────────────────────────────────────────────────────────

export interface SourceData extends Record<string, unknown> {
  portId: string;
  dataType: DataType;
}
export type SourceNode = Node<SourceData, 'source'>;

export function SourceNodeComponent({ id, data, selected }: NodeProps<SourceNode>) {
  const { deleteElements } = useReactFlow();
  return (
    <div className={`fn fn-source${selected ? ' fn-selected' : ''}`}>
      <div className="fn-header">
        <span>Source</span>
        <button className="fn-delete" onClick={() => deleteElements({ nodes: [{ id }] })}>×</button>
      </div>
      <div className="fn-body">
        <p className="fn-port-name">{data.portId || <em>unnamed</em>}</p>
        <span className="fn-badge">{data.dataType}</span>
      </div>
      <Handle type="source" position={Position.Right} id="out" className="fn-handle" />
    </div>
  );
}

// ─── Block ───────────────────────────────────────────────────────────────────

export interface BlockData extends Record<string, unknown> {
  definitionId: number;
  instanceId: number;
  definitionName: string;
  inputs: BlockPort[];
  outputs: BlockPort[];
}
export type BlockNode = Node<BlockData, 'block'>;

export function BlockNodeComponent({ id, data, selected }: NodeProps<BlockNode>) {
  const { deleteElements } = useReactFlow();
  return (
    <div className={`fn fn-block${selected ? ' fn-selected' : ''}`}>
      <div className="fn-header">
        <span>{data.definitionName || `def:${data.definitionId}`}</span>
        <button className="fn-delete" onClick={() => deleteElements({ nodes: [{ id }] })}>×</button>
      </div>
      <div className="fn-ports">
        <div className="fn-inputs-col">
          {data.inputs.map((p) => (
            <div key={p.name} className="fn-port fn-port-in">
              <Handle type="target" position={Position.Left} id={`in-${p.name}`} className="fn-handle" />
              <span className="fn-port-name">{p.name}</span>
              <span className="fn-badge">{p.data_type}</span>
            </div>
          ))}
          {data.inputs.length === 0 && <span className="fn-no-ports">—</span>}
        </div>
        <div className="fn-outputs-col">
          {data.outputs.map((p) => (
            <div key={p.name} className="fn-port fn-port-out">
              <span className="fn-badge">{p.data_type}</span>
              <span className="fn-port-name">{p.name}</span>
              <Handle type="source" position={Position.Right} id={`out-${p.name}`} className="fn-handle" />
            </div>
          ))}
          {data.outputs.length === 0 && <span className="fn-no-ports">—</span>}
        </div>
      </div>
      <div className="fn-footer">inst#{data.instanceId}</div>
    </div>
  );
}

// ─── Sink ─────────────────────────────────────────────────────────────────────

export interface SinkData extends Record<string, unknown> {
  portId: string;
  dataType: DataType;
}
export type SinkNode = Node<SinkData, 'sink'>;

export function SinkNodeComponent({ id, data, selected }: NodeProps<SinkNode>) {
  const { deleteElements } = useReactFlow();
  return (
    <div className={`fn fn-sink${selected ? ' fn-selected' : ''}`}>
      <Handle type="target" position={Position.Left} id="in" className="fn-handle" />
      <div className="fn-header">
        <span>Sink</span>
        <button className="fn-delete" onClick={() => deleteElements({ nodes: [{ id }] })}>×</button>
      </div>
      <div className="fn-body">
        <p className="fn-port-name">{data.portId || <em>unnamed</em>}</p>
        <span className="fn-badge">{data.dataType}</span>
      </div>
    </div>
  );
}
