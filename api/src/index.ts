import Fastify from 'fastify';
import apiKeyAuth from './lib/auth';
import { prisma } from './lib/prisma';

const fastify = Fastify({
  logger: true,
});

fastify.register(apiKeyAuth);

fastify.addContentTypeParser(
  'text/json',
  { parseAs: 'string' },
  fastify.getDefaultJsonParser('ignore', 'ignore'),
);

fastify.get('/', async function (_request, reply) {
  const blah = await prisma.commit.findMany({ select: { timestamp: true } });
  // const blah = await prisma.commit.deleteMany();
  console.log(blah);
  reply.send({ hello: 'world' });
});

const start = async () => {
  try {
    await fastify.listen({
      host: '0.0.0.0',
      port: Number(process.env.PORT ?? '10000'),
    });
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

void start();
