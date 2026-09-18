import type { Deployment, NewDeployment, UpdateDeployment } from '../types';

const BASE = '/api/v1/deployment';

export async function getDeployments(): Promise<Deployment[]> {
  const res = await fetch(BASE);
  if (!res.ok) throw new Error('Failed to fetch deployments');
  return res.json();
}

export async function getDeployment(id: number): Promise<Deployment> {
  const res = await fetch(`${BASE}/${id}`);
  if (!res.ok) throw new Error('Deployment not found');
  return res.json();
}

export async function createDeployment(data: NewDeployment): Promise<Deployment> {
  const res = await fetch(BASE, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to create deployment');
  return res.json();
}

export async function deleteDeployment(id: number): Promise<void> {
  const res = await fetch(`${BASE}/${id}`, { method: 'DELETE' });
  if (!res.ok) throw new Error('Failed to delete deployment');
}

export async function updateDeployment(data: UpdateDeployment): Promise<Deployment> {
  const res = await fetch(BASE, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to update deployment');
  return res.json();
}