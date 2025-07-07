import * as z from 'zod';

export const analyzerSchema = z.object({
  token: z.string().min(1),
  username: z.string().min(1),
});
