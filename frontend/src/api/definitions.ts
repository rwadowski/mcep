import type { Definition, NewDefinition, UpdateDefinition } from '../types';

const BASE = '/api/v1/definition';

export async function getDefinitions(): Promise<Definition[]> {
  const res = await fetch(BASE);
  if (!res.ok) throw new Error('Failed to fetch definitions');
  return res.json();
}

export async function getDefinition(id: number): Promise<Definition> {
  const res = await fetch(`${BASE}/${id}`);
  if (!res.ok) throw new Error('Definition not found');
  return res.json();
}

export async function createDefinition(data: NewDefinition): Promise<Definition> {
  const res = await fetch(BASE, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to create definition');
  return res.json();
}

export async function updateDefinition(data: UpdateDefinition): Promise<Definition> {
  const res = await fetch(BASE, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error('Failed to update definition');
  return res.json();
}

export async function deleteDefinition(id: number): Promise<void> {
  const res = await fetch(`${BASE}/${id}`, { method: 'DELETE' });
  if (!res.ok) throw new Error('Failed to delete definition');
}