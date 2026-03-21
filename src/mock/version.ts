export const mockVersions = {
  code: 200,
  msg: 'success',
  data: {
    total: 8,
    list: [
      { version: '3.11.3', installStatus: true, useStatus: false },
      { version: '3.10.7', installStatus: true, useStatus: true },
      { version: '3.12.1', installStatus: false, useStatus: false },
      { version: '3.13.0', installStatus: true, useStatus: true },
      { version: '3.11.8', installStatus: true, useStatus: true },
      { version: '3.12.5', installStatus: false, useStatus: false },
      { version: '3.10.2', installStatus: true, useStatus: false },
      { version: '3.13.3', installStatus: true, useStatus: false },
    ],
    page: 0,
    pageSize: 10,
  },
};

export const mockNodeVersions = {
  code: 200,
  msg: 'success',
  data: {
    total: 10,
    list: [
      { version: 'v22.11.0', installStatus: false, useStatus: false },
      { version: 'v22.10.0', installStatus: true, useStatus: true },
      { version: 'v20.18.0', installStatus: true, useStatus: false },
      { version: 'v20.17.0', installStatus: false, useStatus: false },
      { version: 'v18.20.4', installStatus: true, useStatus: false },
      { version: 'v18.20.3', installStatus: false, useStatus: false },
      { version: 'v20.16.0', installStatus: true, useStatus: false },
      { version: 'v22.9.0', installStatus: false, useStatus: false },
      { version: 'v18.19.0', installStatus: true, useStatus: false },
      { version: 'v20.15.0', installStatus: false, useStatus: false },
    ],
    page: 0,
    pageSize: 10,
  },
};
