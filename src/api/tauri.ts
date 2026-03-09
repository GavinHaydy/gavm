import * as tauriCore from '@tauri-apps/api/core';
import { InvokeArgs } from '@tauri-apps/api/core';

import { isMock } from '@/core/config/env';
import { CommandEnum, InstallStatusEnum } from '@/core/constants/enum';
import { mockHandlers } from '@/mock/handlers';
import { mockResponse } from '@/mock/util';

const isTauri = navigator.userAgent.includes('lvm');

interface IApiResponse<T> {
  data: T;
  msg: string;
  code: number;
}

const defaultErrorResponse: IApiResponse<undefined> = {
  code: 500,
  msg: 'Internal Error',
  data: undefined,
};

/**
 * 安全 invoke
 * 浏览器模式不会报错
 */
export async function safeInvoke<T>(
  command: CommandEnum | InstallStatusEnum,
  args?: InvokeArgs,
): Promise<T> {
  let res: IApiResponse<T>;

  if (isMock && command in mockHandlers) {
    const handler = mockHandlers[command as keyof typeof mockHandlers];

    res = handler
      ? ((await mockResponse<T>(handler() as unknown as T)) as IApiResponse<T>)
      : (defaultErrorResponse as IApiResponse<T>);
  } else if (!isTauri) {
    res = defaultErrorResponse as IApiResponse<T>;
  } else {
    res = await tauriCore.invoke<IApiResponse<T>>(command, args);
  }

  if (res?.code !== 200) {
    throw new Error(res.msg);
  }

  return res?.data;
}
export { isTauri };
