export const mockVersions = {
  code: 200,
  msg: 'success',
  data: {
    total: 8,
    list: [
      { version: '3.11.3', installed: true, active: false },
      { version: '3.10.7', installed: true, active: true },
      { version: '3.12.1', installed: false, active: false },
      { version: '3.13.0', installed: true, active: true },
      { version: '3.11.8', installed: true, active: true },
      { version: '3.12.5', installed: false, active: false },
      { version: '3.10.2', installed: true, active: false },
      { version: '3.13.3', installed: true, active: false },
    ],
    page: 0,
    pageSize: 10,
  },
};
