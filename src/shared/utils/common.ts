// snake_case -> camelCase
export const keysToCamel = (obj: Record<string, boolean | string>) => {
  const result: Record<string, boolean | string> = {};

  const toCamel = (str: string) => {
    return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
  };

  for (const key in obj) {
    const camelKey = toCamel(key);
    result[camelKey] = obj[key];
  }

  return result;
};
