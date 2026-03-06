import * as tauriCore from '@tauri-apps/api/core';
import { InvokeArgs } from '@tauri-apps/api/core';

import { isMock } from '@/core/config/env';
import { CommandEnum, InstallStatusEnum } from '@/core/constants/enum';
import { mockHandlers } from '@/mock/handlers';
import { mockResponse } from '@/mock/util';

const isTauri = navigator.userAgent.includes('lvm');

/**
 * 安全 invoke
 * 浏览器模式不会报错
 */
export async function safeInvoke<T>(
  command: CommandEnum | InstallStatusEnum,
  args?: InvokeArgs,
): Promise<T> {
  if (isMock && command in mockHandlers) {
    const handler = mockHandlers[command as keyof typeof mockHandlers];

    return handler ? (mockResponse(handler()) as Promise<T>) : (undefined as T);
  }

  return !isTauri ? Promise.resolve(undefined as T) : tauriCore.invoke<T>(command, args);
}

export { isTauri };
