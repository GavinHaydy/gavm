import { useCallback, useEffect, useState } from 'react';

import { safeInvoke } from '@/api/tauri';
import { CommandEnum, InstallStatusEnum, LanguageEnum } from '@/core/constants/enum';
import { SearchPayload } from '@/core/types/common.ts';
import { VersionTable, VersionItem, VersionResult } from '@/shared/components/VersionTable';

export const PythonManagePage = () => {
  const [searchPayload, setSearchPayload] = useState<SearchPayload>({
    language: LanguageEnum.PYTHON,
    page: 0,
    pageSize: 10,
    keyWord: '',
  });
  const [data, setData] = useState<VersionResult>({
    total: 0,
    list: [],
  });

  const getList = useCallback(async () => {
    const result = await safeInvoke<VersionResult>(CommandEnum.LIST_VERSIONS, searchPayload);
    setData(result);
  }, [searchPayload]);

  useEffect(() => {
    void getList();
  }, [getList]);

  const handleSearch = async (keyWord: string) => {
    setSearchPayload(prevState => ({ ...prevState, keyWord: keyWord }));
  };

  const handleInstallToggle = async (record: VersionItem) => {
    const command = record.install_status
      ? InstallStatusEnum.UNINSTALLED
      : InstallStatusEnum.INSTALLED;

    await safeInvoke(command, {
      language: LanguageEnum.PYTHON,
      version: record.version,
    });

    await getList();
  };

  const handleUseToggle = async (record: VersionItem) => {
    await safeInvoke(CommandEnum.USE_VERSION, {
      language: LanguageEnum.PYTHON,
      version: record.version,
    });

    await getList();
  };

  return (
    <VersionTable
      data={data}
      onInstallToggle={handleInstallToggle}
      onSearch={handleSearch}
      onUseToggle={handleUseToggle}
    />
  );
};
