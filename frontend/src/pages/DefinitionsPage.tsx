import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDefinitions, updateDefinition, deleteDefinition } from '../api/definitions';
import type { Definition, UpdateDefinition } from '../types';

export default function DefinitionsPage() {
  const navigate = useNavigate();
  const [definitions, setDefinitions] = useState<Definition[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<Definition | null>(null);
  const [bodyText, setBodyText] = useState('{}');
  const [editMeta, setEditMeta] = useState({ name: '', version: '', description: '', help: '' });
  const [loading, setLoading] = useState(false);

  const load = () =>
    getDefinitions()
      .then(setDefinitions)
      .catch((e) => setError(e.message));

  useEffect(() => { load(); }, []);

  const openDef = (def: Definition) => {
    setEditMeta({
      name: def.name,
      version: def.version,
      description: def.description ?? '',
      help: def.help ?? '',
    });
    setBodyText(JSON.stringify(def.body, null, 2));
    setSelected(def);
    setError(null);
  };

  const handleUpdate = async () => {
    if (!selected) return;
    setLoading(true);
    setError(null);
    try {
      const parsed = JSON.parse(bodyText);
      const update: UpdateDefinition = {
        id: selected.id,
        name: editMeta.name || null,
        version: editMeta.version || null,
        body: bodyText,
        body_type: parsed.type ?? null,
        description: editMeta.description || null,
        help: editMeta.help || null,
      };
      await updateDefinition(update);
      load();
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Error');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: number) => {
    if (!confirm('Delete this definition?')) return;
    try {
      await deleteDefinition(id);
      if (selected?.id === id) setSelected(null);
      load();
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Error');
    }
  };

  return (
    <div className="sidebar-layout">
      <div className="sidebar">
        <div className="sidebar-header">
          <h1>Definitions</h1>
          <button className="btn-primary btn-sm" onClick={() => navigate('/definitions/new')}>+ New</button>
        </div>
        <div className="sidebar-list">
          {definitions.length === 0 && <p className="empty">No definitions yet.</p>}
          {definitions.map((def) => (
            <div
              key={def.id}
              className={`sidebar-item${selected?.id === def.id ? ' sidebar-item-active' : ''}`}
              onClick={() => openDef(def)}
            >
              <span className="sidebar-item-name">{def.name}</span>
              <span className="badge">v{def.version}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="sidebar-content">
        {error && <div className="error">{error}</div>}
        {selected ? (
          <div className="card form-card">
            <h2>{selected.name}</h2>
            <div className="two-col">
              <label>Name
                <input value={editMeta.name} onChange={(e) => setEditMeta({ ...editMeta, name: e.target.value })} />
              </label>
              <label>Version
                <input value={editMeta.version} onChange={(e) => setEditMeta({ ...editMeta, version: e.target.value })} />
              </label>
            </div>
            <label>Description
              <input value={editMeta.description} onChange={(e) => setEditMeta({ ...editMeta, description: e.target.value })} />
            </label>
            <label>Help
              <input value={editMeta.help} onChange={(e) => setEditMeta({ ...editMeta, help: e.target.value })} />
            </label>
            <label>Body (JSON)
              <textarea rows={10} value={bodyText} onChange={(e) => setBodyText(e.target.value)} />
            </label>
            <div className="form-actions">
              <button className="btn-primary" onClick={handleUpdate} disabled={loading}>
                {loading ? 'Saving…' : 'Save'}
              </button>
              <button className="btn-danger btn-sm" onClick={() => handleDelete(selected.id)}>Delete</button>
            </div>
          </div>
        ) : (
          <p className="empty">Select a definition to edit.</p>
        )}
      </div>
    </div>
  );
}
