import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDeployments, deleteDeployment } from '../api/deployments';
import type { Deployment } from '../types';

export default function DeploymentsPage() {
  const navigate = useNavigate();
  const [deployments, setDeployments] = useState<Deployment[]>([]);
  const [error, setError] = useState<string | null>(null);

  const load = () =>
    getDeployments()
      .then(setDeployments)
      .catch((e: unknown) => setError(e instanceof Error ? e.message : 'Error'));

  useEffect(() => { load(); }, []);

  const handleDelete = async (id: number) => {
    if (!confirm('Delete this deployment?')) return;
    try {
      await deleteDeployment(id);
      load();
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Error');
    }
  };

  return (
    <div className="page">
      <div className="page-header">
        <h1>Deployments</h1>
        <button className="btn-primary" onClick={() => navigate('/deployments/new')}>+ New Flow</button>
      </div>

      {error && <div className="error">{error}</div>}

      <div className="list">
        {deployments.length === 0 && <p className="empty">No deployments yet.</p>}
        {deployments.map((dep) => (
          <div key={dep.id} className="card">
            <div className="card-header">
              <div>
                <span className="name">{dep.name}</span>
                <span className="badge">v{dep.version}</span>
                <span className="id-badge">#{dep.id}</span>
              </div>
              <button className="btn-danger btn-sm" onClick={() => handleDelete(dep.id)}>Delete</button>
            </div>

            <div className="deployment-sections">
              <section>
                <h3>Blocks ({dep.blocks.length})</h3>
                {dep.blocks.map((b, i) => (
                  <div key={i} className="tag">def:{b.definition_id} inst:{b.id}</div>
                ))}
              </section>
              <section>
                <h3>Sources ({dep.sources.length})</h3>
                {dep.sources.map((s, i) => (
                  <div key={i} className="tag">{s.id} <span className="badge">{s.data_type}</span></div>
                ))}
              </section>
              <section>
                <h3>Sinks ({dep.sinks.length})</h3>
                {dep.sinks.map((s, i) => (
                  <div key={i} className="tag">{s.id} <span className="badge">{s.data_type}</span></div>
                ))}
              </section>
              <section>
                <h3>Connections ({dep.connections.length})</h3>
                {dep.connections.map((c, i) => (
                  <div key={i} className="connection">
                    <code>{JSON.stringify(c.from)}</code>
                    <span className="arrow">→</span>
                    <code>{JSON.stringify(c.to)}</code>
                  </div>
                ))}
              </section>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
