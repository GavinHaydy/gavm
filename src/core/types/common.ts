import { InvokeArgs } from '@tauri-apps/api/core';

export interface ISearchPayload {
  language: string;
  page: number;
  pageSize: number;
  keyWord: string;
}

export type SearchPayload = ISearchPayload & InvokeArgs;
