import * as z from 'zod';

export const analyzerSchema = z.object({
  token: z.string().min(1),
  username: z.string().min(1),
  date: z.string().min(1).optional(),
});

export const commitDataSchema = z.object({
  repo: z.string(),
  username: z.string(),
  hash: z.string(),
  timestamp: z
    .number()
    .int()
    .nonnegative()
    .refine(
      (val) => val > 0 && val < Date.now() + 10000000000, // future buffer
      { message: 'Epoch timestamp out of range' },
    )
    .transform((val) => new Date(val * 1000)), // assumes milliseconds
  files: z.number(),
  linesAdded: z.number(),
  linesDeleted: z.number(),
});
