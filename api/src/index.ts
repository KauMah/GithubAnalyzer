import Fastify from 'fastify';
import { analyzerSchema, commitDataSchema } from './schema';
import { spawn } from 'node:child_process';
import { prisma } from './lib/prisma';

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

    gha.stdout.on('data', (data: string) => {
      const lines = String(data).trim().split('\n');
      const createInput = lines.map((line) => {
        const [
          repo,
          username,
          hash,
          timestamp,
          files,
          linesAdded,
          linesDeleted,
        ] = line.split(',');
        return commitDataSchema.parse({
          repo,
          username,
          hash,
          timestamp: Number(timestamp),
          files: Number(files),
          linesAdded: Number(linesAdded),
          linesDeleted: Number(linesDeleted),
        });
      });
      // console.log(createInput);
      prisma.commit
        .createMany({
          data: createInput,
          skipDuplicates: true,
        })
        .then((data) => console.log(`Inserted ${data.count} rows`))
        .catch((err) => console.error(err));

      // for (const line of lines) {
      //     line.split(',');
      //     prisma.commit.createMany({
      //       data: {
      //         repo,
      //         username,
      //         hash,
      //         timestamp,
      //         files,
      //         linesAdded,
      //         linesDeleted
      //       }
      //     })
      // }
    });

    gha.stderr.on('error', (err) => {
      throw new Error(`Error running GithubAnalyzer: ${err}`);
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
