import { InvokeArgs } from '@tauri-apps/api/core';

import { InstallStatusFilterEnum } from '@/core/constants/enum';

export interface ISearchPayload {
  language: string;
  page: number;
  pageSize: number;
  keyWord: string;
  installStatus?: number; // 0: all, 1: installed, 2: uninstalled
}

export type SearchPayload = ISearchPayload & InvokeArgs;

export interface SearchFormValues {
  keyword?: string;
  installStatus?: InstallStatusFilterEnum;
}

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
