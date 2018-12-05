import { createReadStream, readFileSync } from 'fs';

import { defineType, gql } from '@girin/typelink';
import { TestClient, prepareTestEnv, destroyTestEnv, TestHttpServer, FileUpload } from '@girin/framework';

import { MediaService } from '../src';
import { TestMedia } from './TestMedia';


@defineType(gql`
  type Query {
    getMedia(id: String!): ${TestMedia}!
  }
`)
class Query {

}

@defineType(gql`
  type Mutation {
    echoFileContent(upload: Upload!): String
    createMedia(upload: Upload!): ${TestMedia}!
    deleteMedia(id: String!): String!
  }
`)
class Mutation {
  static async echoFileContent(_source: null, args: { upload: Promise<FileUpload> }) {
    const upload = await args.upload;

    const stream = upload.stream;
    let bufs: string = '';
    stream.on('data', buf => {
      bufs += buf.toString();
    });
    return new Promise((resolve, reject) => {
      stream.once('end', () => resolve(bufs));
      stream.once('error', err => reject(err));
    });
  }

  static async createMedia(_source: null, args: { upload: Promise<FileUpload> }) {
    const mediaModule = MediaService.object();
    return mediaModule.createMediaFromUpload(await args.upload);
  }

  static async deleteMedia(_source: null, args: { id: string }): Promise<string> {
    await MediaService.object().deleteMedia(args.id);
    return args.id;
  }
}

describe('media', () => {

  let client: TestClient;
  beforeAll(async () => {
    await prepareTestEnv({ Query, Mutation })
      .load(new MediaService({
        endpoint: '/media',
        mediaConstructor: TestMedia,
      }))
      .run();
    client = TestHttpServer.object().getClient();
  });
  afterAll(destroyTestEnv);

  const fileContent = readFileSync('./LICENSE').toString();
  it('works', async () => {
    const res = await client.sendQueryWithFiles({
      query: `
        mutation($upload: Upload!) {
          echoFileContent(upload: $upload)
        }
      `,
      map: { 0: ['variables.upload'] },
      variables: {
        upload: null,
      },
      files: [
        createReadStream('./LICENSE'),
      ],
    });

    expect(res.data.echoFileContent).toBe(fileContent);
  });

  it('works with basic CRUD actions', async () => {
    const { data, errors } = await client.sendQueryWithFiles({
      query: `
        mutation($upload: Upload!) {
          createMedia(upload: $upload) {
            id
            url
            filename
            originalFilename
            uuid
            size
            uploadedAt
          }
        }
      `,
      map: { 0: ['variables.upload'] },
      variables: {
        upload: null,
      },
      files: [
        createReadStream('./LICENSE'),
      ],
    });

    // { createMedia:
    //   { id: 'UxcN56fCZYVJxauI',
    //     url: '/media/UxcN56fCZYVJxauI',
    //     filename: 'LICENSE_ef103170-f853-11e8-8321-1bcfd5df385c',
    //     originalFilename: 'LICENSE',
    //     uuid: 'ef103170-f853-11e8-8321-1bcfd5df385c',
    //     size: 1085,
    //     uploadedAt: '2018-12-05T06:06:41.162Z' } }
    expect(errors).toBeUndefined();
    const { createMedia } = data;
    expect(typeof createMedia.id).toBe('string');
    expect(typeof createMedia.url).toBe('string');
    expect(typeof createMedia.size).toBe('number');
    expect(typeof createMedia.uploadedAt).toBe('string');


    let { res: resMaybe200, body } = await client.request('GET', createMedia.url);
    expect(resMaybe200.statusCode).toBe(200);
    expect(body.toString()).toEqual(fileContent);
    expect(resMaybe200.headers.etag).toBe(data.createMedia.uuid);

    const { res: headRes } = await client.request('HEAD', createMedia.url);
    expect(headRes.statusCode).toBe(200);
    expect(headRes.headers.etag).toBe(data.createMedia.uuid);

    const deleteMediaMutation = await client.sendQuery({
      query: `
        mutation($id: String!) {
          deleteMedia(id: $id)
        }
      `,
      variables: { id: createMedia.id },
    });
    expect(deleteMediaMutation.errors).toBeUndefined();
    expect(deleteMediaMutation.data).toEqual({
      deleteMedia: createMedia.id,
    });

    let { res: resMaybe404 } = await client.request('GET', createMedia.url);

    expect(resMaybe404.statusCode).toBe(404);
  });
});
