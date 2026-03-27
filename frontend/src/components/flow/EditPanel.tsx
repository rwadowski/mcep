import { useReactFlow, type Node } from '@xyflow/react';
import type { DataType, Definition } from '../../types';
import type { BlockData, SinkData, SourceData } from './nodes';

const DATA_TYPES: DataType[] = ['Boolean', 'UnsignedInt', 'SignedInt', 'Float', 'Text'];

interface Props {
  node: Node;
  definitions: Definition[];
  onClose: () => void;
}

export function EditPanel({ node, definitions, onClose }: Props) {
  const { updateNodeData } = useReactFlow();

  const update = (patch: Record<string, unknown>) => updateNodeData(node.id, patch);

  return (
    <div className="edit-panel">
      <div className="edit-panel-header">
        <span>Edit {node.type}</span>
        <button className="fn-delete" onClick={onClose}>×</button>
      </div>

      {node.type === 'source' && <SourceForm data={node.data as SourceData} update={update} />}
      {node.type === 'block'  && <BlockForm  data={node.data as BlockData}  update={update} definitions={definitions} />}
      {node.type === 'sink'   && <SinkForm   data={node.data as SinkData}   update={update} />}
    </div>
  );
}

// ─── Source form ─────────────────────────────────────────────────────────────

function SourceForm({ data, update }: { data: SourceData; update: (p: Record<string, unknown>) => void }) {
  return (
    <>
      <label>Port ID
        <input value={data.portId} onChange={(e) => update({ portId: e.target.value })} placeholder="my-source" />
      </label>
      <label>Data Type
        <select value={data.dataType} onChange={(e) => update({ dataType: e.target.value })}>
          {DATA_TYPES.map((t) => <option key={t}>{t}</option>)}
        </select>
      </label>
    </>
  );
}

// ─── Sink form ────────────────────────────────────────────────────────────────

function SinkForm({ data, update }: { data: SinkData; update: (p: Record<string, unknown>) => void }) {
  return (
    <>
      <label>Port ID
        <input value={data.portId} onChange={(e) => update({ portId: e.target.value })} placeholder="my-sink" />
      </label>
      <label>Data Type
        <select value={data.dataType} onChange={(e) => update({ dataType: e.target.value })}>
          {DATA_TYPES.map((t) => <option key={t}>{t}</option>)}
        </select>
      </label>
    </>
  );
}

// ─── Block form ───────────────────────────────────────────────────────────────

function BlockForm({
  data,
  update,
  definitions,
}: {
  data: BlockData;
  update: (p: Record<string, unknown>) => void;
  definitions: Definition[];
}) {
  const selectDef = (id: number) => {
    const def = definitions.find((d) => d.id === id);
    if (!def) return;
    const body = def.body as { inputs?: { name: string; data_type: DataType }[]; outputs?: { name: string; data_type: DataType }[] };
    update({
      definitionId: id,
      definitionName: def.name,
      inputs: body.inputs ?? [],
      outputs: body.outputs ?? [],
    });
  };

  return (
    <>
      <label>Definition
        <select value={data.definitionId} onChange={(e) => selectDef(parseInt(e.target.value))}>
          {definitions.map((d) => (
            <option key={d.id} value={d.id}>{d.name} v{d.version}</option>
          ))}
        </select>
      </label>
      <label>Instance ID
        <input
          type="number"
          value={data.instanceId}
          onChange={(e) => update({ instanceId: parseInt(e.target.value) || 0 })}
        />
      </label>

      <div className="ep-section">
        <span className="ep-section-title">Inputs</span>
        {data.inputs.length === 0 && <p className="empty">none</p>}
        {data.inputs.map((p, i) => (
          <div key={i} className="ep-port-row">
            <input
              value={p.name}
              onChange={(e) => {
                const inputs = [...data.inputs];
                inputs[i] = { ...inputs[i], name: e.target.value };
                update({ inputs });
              }}
              placeholder="name"
            />
            <select
              value={p.data_type}
              onChange={(e) => {
                const inputs = [...data.inputs];
                inputs[i] = { ...inputs[i], data_type: e.target.value as DataType };
                update({ inputs });
              }}
            >
              {DATA_TYPES.map((t) => <option key={t}>{t}</option>)}
            </select>
            <button className="fn-delete" onClick={() => update({ inputs: data.inputs.filter((_, j) => j !== i) })}>×</button>
          </div>
        ))}
        <button className="btn-secondary btn-sm" onClick={() => update({ inputs: [...data.inputs, { name: '', data_type: 'Text' }] })}>
          + Input
        </button>
      </div>

      <div className="ep-section">
        <span className="ep-section-title">Outputs</span>
        {data.outputs.length === 0 && <p className="empty">none</p>}
        {data.outputs.map((p, i) => (
          <div key={i} className="ep-port-row">
            <input
              value={p.name}
              onChange={(e) => {
                const outputs = [...data.outputs];
                outputs[i] = { ...outputs[i], name: e.target.value };
                update({ outputs });
              }}
              placeholder="name"
            />
            <select
              value={p.data_type}
              onChange={(e) => {
                const outputs = [...data.outputs];
                outputs[i] = { ...outputs[i], data_type: e.target.value as DataType };
                update({ outputs });
              }}
            >
              {DATA_TYPES.map((t) => <option key={t}>{t}</option>)}
            </select>
            <button className="fn-delete" onClick={() => update({ outputs: data.outputs.filter((_, j) => j !== i) })}>×</button>
          </div>
        ))}
        <button className="btn-secondary btn-sm" onClick={() => update({ outputs: [...data.outputs, { name: '', data_type: 'Text' }] })}>
          + Output
        </button>
      </div>
    </>
  );
}