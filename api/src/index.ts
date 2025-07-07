import Fastify from 'fastify';

const fastify = Fastify({
  logger: true,
});

fastify.get('/', function (_request, reply) {
  reply.send({ hello: 'world' });
});

fastify.post('/', function (_request, _reply) {});

const start = async () => {
  try {
    await fastify.listen({ port: Number(process.env.PORT ?? '3000') });
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

void start();
