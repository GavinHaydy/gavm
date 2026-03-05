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
  install_status: boolean;
  use_status: boolean;
}

export interface VersionResult {
  total: number;
  list: VersionItem[];
}
