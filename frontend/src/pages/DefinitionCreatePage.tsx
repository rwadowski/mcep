import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createDefinition } from '../api/definitions';
import type { BlockPort, BlockType, CodeBlockBody, DataType, Dependency, GithubBlockBody } from '../types';

const DATA_TYPES: DataType[] = ['Boolean', 'UnsignedInt', 'SignedInt', 'Float', 'Text'];

const emptyPort = (): BlockPort => ({ name: '', data_type: 'Text' });
const emptyDep = (): Dependency => ({ name: '' });

export default function DefinitionCreatePage() {
  const navigate = useNavigate();

  const [name, setName] = useState('');
  const [version, setVersion] = useState('1.0.0');
  const [description, setDescription] = useState('');
  const [help, setHelp] = useState('');
  const [blockType, setBlockType] = useState<BlockType>('CodeBlock');

  // CodeBlock fields
  const [source, setSource] = useState('');

  // Github fields — token is never stored in localStorage/sessionStorage/cache
  const [ghOwner, setGhOwner] = useState('');
  const [ghRepo, setGhRepo] = useState('');
  const [ghPath, setGhPath] = useState('');
  const [ghToken, setGhToken] = useState('');

  // Shared
  const [inputs, setInputs] = useState<BlockPort[]>([]);
  const [outputs, setOutputs] = useState<BlockPort[]>([]);
  const [dependencies, setDependencies] = useState<Dependency[]>([]);

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const updatePort = (
    list: BlockPort[],
    setList: (v: BlockPort[]) => void,
    i: number,
    field: keyof BlockPort,
    value: string,
  ) => {
    const next = [...list];
    next[i] = { ...next[i], [field]: value };
    setList(next);
  };

  const removePort = (list: BlockPort[], setList: (v: BlockPort[]) => void, i: number) =>
    setList(list.filter((_, j) => j !== i));

  const handleSubmit = async () => {
    setError(null);
    if (!name.trim()) { setError('Name is required'); return; }
    if (!version.trim()) { setError('Version is required'); return; }

    let body: CodeBlockBody | GithubBlockBody;

    if (blockType === 'CodeBlock') {
      body = { type: 'CodeBlock', inputs, outputs, source, dependencies };
    } else {
      if (!ghToken) { setError('GitHub token is required'); return; }
      body = {
        type: 'Github',
        inputs,
        outputs,
        source: { owner: ghOwner, repository: ghRepo, token: ghToken, path: ghPath },
        dependencies,
      };
    }

    setLoading(true);
    try {
      await createDefinition({
        name: name.trim(),
        version: version.trim(),
        body,
        description: description.trim() || null,
        help: help.trim() || null,
      });
      // Clear sensitive data immediately after successful submission
      setGhToken('');
      navigate('/definitions');
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="page">
      <div className="page-header">
        <h1>New Definition</h1>
        <button className="btn-secondary" onClick={() => navigate('/definitions')}>Cancel</button>
      </div>

      {error && <div className="error">{error}</div>}

      <div className="card form-card">
        <h2>General</h2>
        <div className="two-col">
          <label>Name
            <input value={name} onChange={(e) => setName(e.target.value)} placeholder="my-block" />
          </label>
          <label>Version
            <input value={version} onChange={(e) => setVersion(e.target.value)} placeholder="1.0.0" />
          </label>
        </div>
        <label>Description
          <input value={description} onChange={(e) => setDescription(e.target.value)} />
        </label>
        <label>Help
          <input value={help} onChange={(e) => setHelp(e.target.value)} />
        </label>
      </div>

      <div className="card form-card">
        <h2>Block Type</h2>
        <div className="type-toggle">
          {(['CodeBlock', 'Github'] as BlockType[]).map((t) => (
            <button
              key={t}
              className={blockType === t ? 'btn-primary' : 'btn-secondary'}
              onClick={() => setBlockType(t)}
            >
              {t}
            </button>
          ))}
        </div>

        {blockType === 'CodeBlock' && (
          <label>Source Code
            <textarea
              rows={12}
              value={source}
              onChange={(e) => setSource(e.target.value)}
              placeholder="def process(inputs):\n    return {}"
            />
          </label>
        )}

        {blockType === 'Github' && (
          <>
            <div className="security-note">
              GitHub token is sent directly to the server and is never stored or cached in the browser.
            </div>
            <div className="two-col">
              <label>Owner
                <input value={ghOwner} onChange={(e) => setGhOwner(e.target.value)} placeholder="octocat" />
              </label>
              <label>Repository
                <input value={ghRepo} onChange={(e) => setGhRepo(e.target.value)} placeholder="my-repo" />
              </label>
            </div>
            <label>File Path
              <input value={ghPath} onChange={(e) => setGhPath(e.target.value)} placeholder="blocks/my_block.py" />
            </label>
            <label>Token
              <input
                type="password"
                autoComplete="off"
                value={ghToken}
                onChange={(e) => setGhToken(e.target.value)}
                placeholder="ghp_..."
              />
            </label>
          </>
        )}
      </div>

      <PortsSection label="Inputs" ports={inputs} setPorts={setInputs} updatePort={updatePort} removePort={removePort} />
      <PortsSection label="Outputs" ports={outputs} setPorts={setOutputs} updatePort={updatePort} removePort={removePort} />

      <div className="card form-card">
        <div className="subsection-header">
          <h2>Dependencies</h2>
          <button className="btn-secondary btn-sm" onClick={() => setDependencies([...dependencies, emptyDep()])}>
            + Add
          </button>
        </div>
        {dependencies.length === 0 && <p className="empty">No dependencies.</p>}
        {dependencies.map((dep, i) => (
          <div key={i} className="row-inputs">
            <label>Package name
              <input
                value={dep.name}
                onChange={(e) => {
                  const next = [...dependencies];
                  next[i] = { name: e.target.value };
                  setDependencies(next);
                }}
                placeholder="numpy"
              />
            </label>
            <button className="btn-danger btn-sm" onClick={() => setDependencies(dependencies.filter((_, j) => j !== i))}>×</button>
          </div>
        ))}
      </div>

      <div className="form-actions">
        <button className="btn-primary" onClick={handleSubmit} disabled={loading}>
          {loading ? 'Creating…' : 'Create Definition'}
        </button>
        <button className="btn-secondary" onClick={() => navigate('/definitions')}>Cancel</button>
      </div>
    </div>
  );
}

function PortsSection({
  label, ports, setPorts, updatePort, removePort,
}: {
  label: string;
  ports: BlockPort[];
  setPorts: (v: BlockPort[]) => void;
  updatePort: (list: BlockPort[], setList: (v: BlockPort[]) => void, i: number, field: keyof BlockPort, value: string) => void;
  removePort: (list: BlockPort[], setList: (v: BlockPort[]) => void, i: number) => void;
}) {
  return (
    <div className="card form-card">
      <div className="subsection-header">
        <h2>{label}</h2>
        <button className="btn-secondary btn-sm" onClick={() => setPorts([...ports, emptyPort()])}>+ Add</button>
      </div>
      {ports.length === 0 && <p className="empty">No {label.toLowerCase()} defined.</p>}
      {ports.map((port, i) => (
        <div key={i} className="row-inputs">
          <label>Name
            <input
              value={port.name}
              onChange={(e) => updatePort(ports, setPorts, i, 'name', e.target.value)}
              placeholder="value"
            />
          </label>
          <label>Type
            <select
              value={port.data_type}
              onChange={(e) => updatePort(ports, setPorts, i, 'data_type', e.target.value)}
            >
              {DATA_TYPES.map((t) => <option key={t}>{t}</option>)}
            </select>
          </label>
          <button className="btn-danger btn-sm" onClick={() => removePort(ports, setPorts, i)}>×</button>
        </div>
      ))}
    </div>
  );
}