import {z} from 'zod';

const labelSchema = z.object({
  label: z.string().min(1),
  remark: z.string(),
});

const deleteLabelSchema = z.object({
  label: z.string().min(1),
});

export {labelSchema, deleteLabelSchema};
