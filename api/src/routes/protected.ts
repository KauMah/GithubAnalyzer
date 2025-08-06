import { prisma } from '../lib/prisma';
import { analyzerSchema, commitDataSchema } from '../schema';
import { spawn } from 'child_process';
import { type FastifyPluginCallback } from 'fastify';

const protectedRoutes: FastifyPluginCallback = (fastify) => {
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

  fastify.post('/', { schema }, async function (request, reply) {
    try {
      const body = request.body;
      const { token, username } = analyzerSchema.parse(body);
      const date = await prisma.commit.findFirst({
        where: {
          username,
        },
        orderBy: {
          timestamp: 'desc',
        },
        select: { timestamp: true },
      });

      const parsedDate = new Date(
        date?.timestamp ?? '2010-01-01',
      ).toISOString();
      console.log(parsedDate);

      const gha = spawn('src/github_analyzer', [username, token, parsedDate]);
      let totalCount = 0;

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
        let insertedCount = 0;
        prisma.commit
          .createMany({
            data: createInput,
            skipDuplicates: true,
          })
          .then((data) => {
            insertedCount += data.count;
          })
          .catch((err) => console.error(err))
          .finally(() => {
            totalCount += insertedCount;
          });
      });

      gha.stderr.on('error', (err) => {
        throw new Error(`Error running GithubAnalyzer: ${err}`);
      });

      gha.on('close', (code) => {
        console.log(`inserted ${totalCount} rows`);
        reply.send({
          success: true,
          data: code,
        });

        gha.on('spawn', () => console.log('I started'));
      });
    } catch (err) {
      console.error(err);
      reply.send({
        error: err,
      });
    } finally {
      return reply;
    }
  });
};

export default protectedRoutes;
