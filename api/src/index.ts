import Fastify from 'fastify';
import { analyzerSchema } from './schema';

const fastify = Fastify({
  logger: true,
});

fastify.get('/', function (_request, reply) {
  reply.send({ hello: 'world' });
});

const analysisBodySchema = {
  type: 'object',
  required: ['username', 'token'],
  properties: {
    username: { type: 'string' },
    token: { type: 'string' },
  },
};

const schema = {
  body: analysisBodySchema,
};

fastify.post('/', { schema }, function (request, reply) {
  const { token, username } = analyzerSchema.parse(request.body);
  try {
    reply.send({
      resp: `the token is ${token} and the username is ${username}`,
    });
  } catch (err) {
    console.error(err);
    reply.send({
      error: err,
    });
  }
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
