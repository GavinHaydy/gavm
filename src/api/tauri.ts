import * as tauriCore from '@tauri-apps/api/core';

const isTauri = navigator.userAgent.includes("lvm")
console.log(typeof window,window,isTauri)

/**
 * 安全 invoke
 * 浏览器模式不会报错
 */
export async function safeInvoke<T = any>(command: string, args?: Record<string, any>): Promise<T> {
  if (!isTauri) {
    console.warn(`[Mock Invoke] ${command}`, args);

    // 这里可以返回 mock 数据
    if (command === 'list_versions') {
      return [
        { version: '3.11.8', installed: true, active: true },
        { version: '3.12.0', installed: false, active: false },
      ] as T;
    }

    return Promise.resolve(undefined as T);
  }

  return tauriCore.invoke<T>(command, args);
}

export { isTauri };
