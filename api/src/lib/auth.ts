import { type FastifyPluginCallback } from 'fastify';
import crypto from 'crypto';
import * as z from 'zod';
import protectedRoutes from '../routes/protected';

const headerSchema = z.string({
  required_error:
    "Auth Headers 'x-api-signature' and 'x-api-timestamp' are required",
});

const apiKeyAuth: FastifyPluginCallback = (fastify) => {
  fastify.addHook('preValidation', async (request, reply) => {
    const sig = headerSchema.parse(request.headers['x-api-signature']);
    const ts = headerSchema.parse(request.headers['x-api-timestamp']);
    const rawBody = request.body;
    if (!validateHmac(JSON.stringify(rawBody), ts, sig)) {
      return reply.code(403).send({ error: 'Forbidden: Invalid HMAC' });
    }
  });

  fastify.register(protectedRoutes);
};

const validateHmac = (body: string, timestamp: string, signature: string) => {
  const now = Math.floor(Date.now() / 1000);
  const drift = Math.abs(now - parseInt(timestamp));
  if (drift > 600) return false;

  const expected = crypto
    .createHmac('sha256', process.env.API_SECRET_KEY ?? '')
    .update(`${timestamp}.${body}`)
    .digest('hex');

  return crypto.timingSafeEqual(Buffer.from(expected), Buffer.from(signature));
};

export default apiKeyAuth;
