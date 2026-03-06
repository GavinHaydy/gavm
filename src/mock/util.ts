export function mockResponse<T>(data: T, delay = 300): Promise<T> {
  return new Promise(resolve => {
    setTimeout(() => {
      resolve(data);
    }, delay);
  });
}
