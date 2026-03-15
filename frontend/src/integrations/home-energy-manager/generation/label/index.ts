import {
  createLabelServerFn,
  deleteLabelServerFn,
  getLabelsServerFn,
  updateLabelServerFn,
} from './crud.ts';
import type {Label} from './types';

const getLabels = async () => getLabelsServerFn();

const updateLabel = async (label: Label) =>
  updateLabelServerFn({data: label});

const createLabel = async (label: Label) =>
  createLabelServerFn({data: label});

const deleteLabel = async (label: string) =>
  deleteLabelServerFn({data: {label}});

export {createLabel, deleteLabel, updateLabel, getLabels, type Label};
