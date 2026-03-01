import * as tauriCore from '@tauri-apps/api/core';
import { InvokeArgs } from '@tauri-apps/api/core';

import { CommandEnum, InstallStatusEnum } from '@/core/constants/enum';

const isTauri = navigator.userAgent.includes('lvm');
console.log(typeof window, window, isTauri);

/**
 * 安全 invoke
 * 浏览器模式不会报错
 */
export async function safeInvoke<T>(
  command: CommandEnum | InstallStatusEnum,
  args?: InvokeArgs,
): Promise<T> {
  if (!isTauri) {
    console.warn(`[Mock Invoke] ${command}`, args);

    // 这里可以返回 mock 数据
    if (command === CommandEnum.LIST_VERSIONS) {
      return {
        total: 2,
        list: [
          { version: '3.11.8', installed: true, active: true },
          { version: '3.12.0', installed: false, active: false },
        ],
      } as T;
    }

    return Promise.resolve(undefined as T);
  }

  return tauriCore.invoke<T>(command, args);
}

export { isTauri };
