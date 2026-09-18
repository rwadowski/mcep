import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDeployments, deleteDeployment } from '../api/deployments';
import type { Deployment } from '../types';

export default function DeploymentsPage() {
  const navigate = useNavigate();
  const [deployments, setDeployments] = useState<Deployment[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<Deployment | null>(null);

  const load = () =>
    getDeployments()
      .then(setDeployments)
      .catch((e: unknown) => setError(e instanceof Error ? e.message : 'Error'));

  useEffect(() => { load(); }, []);

  const handleDelete = async (id: number) => {
    if (!confirm('Delete this deployment?')) return;
    try {
      await deleteDeployment(id);
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
          <h1>Deployments</h1>
          <button className="btn-primary btn-sm" onClick={() => navigate('/deployments/new')}>+ New</button>
        </div>
        <div className="sidebar-list">
          {deployments.length === 0 && <p className="empty">No deployments yet.</p>}
          {deployments.map((dep) => (
            <div
              key={dep.id}
              className={`sidebar-item${selected?.id === dep.id ? ' sidebar-item-active' : ''}`}
              onClick={() => setSelected(dep)}
            >
              <span className="sidebar-item-name">{dep.name}</span>
              <span className="badge">v{dep.version}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="sidebar-content">
        {error && <div className="error">{error}</div>}
        {selected ? (
          <div className="card">
            <div className="card-header">
              <div>
                <span className="name">{selected.name}</span>
                <span className="badge">v{selected.version}</span>
                <span className="id-badge">#{selected.id}</span>
              </div>
              <div className="card-actions">
                <button className="btn-secondary btn-sm" onClick={() => navigate(`/deployments/${selected.id}/edit`)}>Edit</button>
                <button className="btn-danger btn-sm" onClick={() => handleDelete(selected.id)}>Delete</button>
              </div>
            </div>

            <div className="deployment-sections">
              <section>
                <h3>Blocks ({selected.blocks.length})</h3>
                {selected.blocks.map((b, i) => (
                  <div key={i} className="tag">def:{b.definition_id} inst:{b.id}</div>
                ))}
              </section>
              <section>
                <h3>Sources ({selected.sources.length})</h3>
                {selected.sources.map((s, i) => (
                  <div key={i} className="tag">{s.id} <span className="badge">{s.data_type}</span></div>
                ))}
              </section>
              <section>
                <h3>Sinks ({selected.sinks.length})</h3>
                {selected.sinks.map((s, i) => (
                  <div key={i} className="tag">{s.id} <span className="badge">{s.data_type}</span></div>
                ))}
              </section>
              <section>
                <h3>Connections ({selected.connections.length})</h3>
                {selected.connections.map((c, i) => (
                  <div key={i} className="connection">
                    <code>{JSON.stringify(c.from)}</code>
                    <span className="arrow">→</span>
                    <code>{JSON.stringify(c.to)}</code>
                  </div>
                ))}
              </section>
            </div>
          </div>
        ) : (
          <p className="empty">Select a deployment to view details.</p>
        )}
      </div>
    </div>
  );
}
