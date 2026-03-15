import {createServerFn} from '@tanstack/react-start';
import {deleteLabelSchema, labelSchema} from './schemas.ts';
import type {Label} from './types';

//const BACKEND_BASE_URL = process.env.BACKEND_BASE_URL;
const BACKEND_BASE_URL = 'http://localhost:8000';

const getLabelsServerFn = createServerFn({method: 'GET'}).handler(
  async () => {
    const res = await fetch(`${BACKEND_BASE_URL}/generation/labels`, {
      method: 'GET',
    });

    if (!res.ok) {
      throw new Error('Failed to get labels');
    }

    return (await res.json()) as Label[];
  },
);

const updateLabelServerFn = createServerFn({method: 'POST'})
  .inputValidator(labelSchema)
  .handler(async ({data}) => {
    const searchParams = new URLSearchParams({remark: data.remark});
    const res = await fetch(
      `${BACKEND_BASE_URL}/generation/labels/${data.label}?${searchParams.toString()}`,
      {
        method: 'PUT',
      },
    );

    if (!res.ok) {
      throw new Error('Failed to update label');
    }
  });

const createLabelServerFn = createServerFn({method: 'POST'})
  .inputValidator(labelSchema)
  .handler(async ({data}) => {
    const res = await fetch(`${BACKEND_BASE_URL}/generation/labels`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });

    if (!res.ok) {
      throw new Error('Failed to create label');
    }
  });

const deleteLabelServerFn = createServerFn({method: 'POST'})
  .inputValidator(deleteLabelSchema)
  .handler(async ({data}) => {
    const res = await fetch(
      `${BACKEND_BASE_URL}/generation/labels/${data.label}`,
      {
        method: 'DELETE',
      },
    );

    if (!res.ok) {
      throw new Error('Failed to delete label');
    }
  });

export {
  getLabelsServerFn,
  updateLabelServerFn,
  createLabelServerFn,
  deleteLabelServerFn,
};
