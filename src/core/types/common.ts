import { InvokeArgs } from '@tauri-apps/api/core';

export interface ISearchPayload {
  language: string;
  page: number;
  pageSize: number;
  keyWord: string;
}

export type SearchPayload = ISearchPayload & InvokeArgs;

export interface VersionItem {
  version: string;
  installStatus: boolean;
  useStatus: boolean;
}

export interface VersionResult {
  total: number;
  list: VersionItem[];
  page: number;
  pageSize: number;
}
