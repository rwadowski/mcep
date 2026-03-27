import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDefinitions, updateDefinition, deleteDefinition } from '../api/definitions';
import type { Definition, UpdateDefinition } from '../types';

export default function DefinitionsPage() {
  const navigate = useNavigate();
  const [definitions, setDefinitions] = useState<Definition[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [editTarget, setEditTarget] = useState<Definition | null>(null);
  const [bodyText, setBodyText] = useState('{}');
  const [editMeta, setEditMeta] = useState({ name: '', version: '', description: '', help: '' });
  const [loading, setLoading] = useState(false);

  const load = () =>
    getDefinitions()
      .then(setDefinitions)
      .catch((e) => setError(e.message));

  useEffect(() => { load(); }, []);

  const openEdit = (def: Definition) => {
    setEditMeta({
      name: def.name,
      version: def.version,
      description: def.description ?? '',
      help: def.help ?? '',
    });
    setBodyText(JSON.stringify(def.body, null, 2));
    setEditTarget(def);
    setError(null);
  };

  const handleUpdate = async () => {
    if (!editTarget) return;
    setLoading(true);
    setError(null);
    try {
      const parsed = JSON.parse(bodyText);
      const update: UpdateDefinition = {
        id: editTarget.id,
        name: editMeta.name || null,
        version: editMeta.version || null,
        body: bodyText,
        body_type: parsed.type ?? null,
        description: editMeta.description || null,
        help: editMeta.help || null,
      };
      await updateDefinition(update);
      setEditTarget(null);
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
      load();
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Error');
    }
  };

  return (
    <div className="page">
      <div className="page-header">
        <h1>Definitions</h1>
        <button className="btn-primary" onClick={() => navigate('/definitions/new')}>+ New</button>
      </div>

      {error && <div className="error">{error}</div>}

      {editTarget && (
        <div className="card form-card">
          <h2>Edit: {editTarget.name}</h2>
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
            <textarea rows={8} value={bodyText} onChange={(e) => setBodyText(e.target.value)} />
          </label>
          <div className="form-actions">
            <button className="btn-primary" onClick={handleUpdate} disabled={loading}>
              {loading ? 'Saving…' : 'Save'}
            </button>
            <button className="btn-secondary" onClick={() => setEditTarget(null)}>Cancel</button>
          </div>
        </div>
      )}

      <div className="list">
        {definitions.length === 0 && <p className="empty">No definitions yet.</p>}
        {definitions.map((def) => (
          <div key={def.id} className="card">
            <div className="card-header">
              <div>
                <span className="name">{def.name}</span>
                <span className="badge">v{def.version}</span>
                <span className="badge">{def.body.type}</span>
              </div>
              <div className="card-actions">
                <button className="btn-secondary" onClick={() => openEdit(def)}>Edit</button>
                <button className="btn-danger" onClick={() => handleDelete(def.id)}>Delete</button>
              </div>
            </div>
            {def.description && <p className="description">{def.description}</p>}
            <details>
              <summary>Body</summary>
              <pre>{JSON.stringify(def.body, null, 2)}</pre>
            </details>
          </div>
        ))}
      </div>
    </div>
  );
}