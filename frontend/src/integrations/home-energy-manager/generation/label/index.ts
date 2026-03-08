import type {Label} from './types';

const BASE_URL = 'http://localhost:8000/generation/labels';

export const getLabels = async () => {
  const res = await fetch(BASE_URL, {method: 'GET'});

  if (!res.ok) {
    throw new Error('Failed to get labels');
  }

  return (await res.json()) as Label[];
};

export const updateLabel = async ({label, remark}: Label) => {
  const res = await fetch(`${BASE_URL}/${label}?remark=${remark}`, {
    method: 'PUT',
  });

  if (!res.ok) {
    throw new Error('Failed to update label');
  }
};

export const createLabel = async (label: Label) => {
  const res = await fetch(BASE_URL, {
    method: 'POST',
    body: JSON.stringify(label),
  });

  if (!res.ok) {
    throw new Error('Failed to create label');
  }
};

export const deleteLabel = async (label: string) => {
  const res = await fetch(`${BASE_URL}/${label}`, {
    method: 'DELETE',
  });

  if (!res.ok) {
    throw new Error('Failed to delete label');
  }
};
