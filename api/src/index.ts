import Fastify from 'fastify';
import { analyzerSchema } from './schema';
import { spawn } from 'node:child_process';

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
  try {
    const { token, username } = analyzerSchema.parse(request.body);
    const gha = spawn('src/github_analyzer', [username, token]);
    // const gha = spawn('ls');

    gha.stdout.on('data', (data: string) => {
      console.log('one data', String(data));
    });

    gha.stderr.on('error', (err) => {
      throw new Error(`Error running GithubAnalzyer: ${err}`);
    });

    gha.on('close', (code) => {
      reply.send({
        resp: `the token is ${token} and the username is ${username}`,
        code,
      });

      gha.on('spawn', () => console.log('I started'));
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
