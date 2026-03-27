export type DataType = 'Boolean' | 'UnsignedInt' | 'SignedInt' | 'Float' | 'Text';
export type BlockType = 'CodeBlock' | 'Github';

export interface BlockPort {
  name: string;
  data_type: DataType;
}

export interface Dependency {
  name: string;
}

export interface CodeBlockBody {
  type: 'CodeBlock';
  inputs: BlockPort[];
  outputs: BlockPort[];
  source: string;
  dependencies: Dependency[];
}

// Token is intentionally excluded from this type — it is handled separately
// as a transient field that is never stored or cached.
export interface GithubBlockBody {
  type: 'Github';
  inputs: BlockPort[];
  outputs: BlockPort[];
  source: {
    owner: string;
    repository: string;
    token: string; // write-only: sent once, never persisted
    path: string;
  };
  dependencies: Dependency[];
}

export type BlockBody = CodeBlockBody | GithubBlockBody;

export interface Definition {
  id: number;
  name: string;
  version: string;
  body: BlockBody;
  description: string | null;
  help: string | null;
}

export interface NewDefinition {
  name: string;
  version: string;
  body: BlockBody;
  description: string | null;
  help: string | null;
}

export interface UpdateDefinition {
  id: number;
  name: string | null;
  version: string | null;
  body: string | null;
  body_type: string | null;
  description: string | null;
  help: string | null;
}

export interface BlockJunction {
  block: string | null;
  sink: string | null;
  source: string | null;
  data_type: DataType;
}

export interface BlockConnection {
  from: BlockJunction;
  to: BlockJunction;
}

export interface DeploymentBlock {
  definition_id: number;
  id: number;
}

export interface DeploymentPort {
  id: string;
  data_type: DataType;
}

export interface Deployment {
  id: number;
  name: string;
  version: string;
  connections: BlockConnection[];
  sources: DeploymentPort[];
  sinks: DeploymentPort[];
  blocks: DeploymentBlock[];
}

export interface NewDeployment {
  name: string;
  version: string;
  connections: BlockConnection[];
  sources: DeploymentPort[];
  sinks: DeploymentPort[];
  blocks: DeploymentBlock[];
}