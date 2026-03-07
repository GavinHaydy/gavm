import { message } from 'antd';
import { useEffect, useState } from 'react';

import { safeInvoke } from '@/api/tauri';
import { CommandEnum, InstallStatusEnum, LanguageEnum } from '@/core/constants/enum';
import { SearchPayload, VersionItem, VersionResult } from '@/core/types/common.ts';
import { VersionTable } from '@/shared/components/VersionTable';

export const PythonManagePage = () => {
  const [loading, setLoading] = useState(false);
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

  const getList = async () => {
    try {
      setLoading(true);
      const data = await safeInvoke<VersionResult>(CommandEnum.LIST_VERSIONS, searchPayload);
      setData(data);
    } catch (error) {
      message.error((error as Error).message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void getList();
  }, [searchPayload]);

  const handleSearch = (keyWord: string) => {
    setSearchPayload(prevState => ({ ...prevState, keyWord: keyWord }));
  };

  const handleVersionAction = async (
    command: CommandEnum | InstallStatusEnum,
    record: VersionItem,
  ) => {
    await safeInvoke(command, {
      language: LanguageEnum.PYTHON,
      version: record.version,
    });

    await getList();
  };

  return (
    <VersionTable
      loading={loading}
      data={data}
      handleVersionAction={handleVersionAction}
      onSearch={handleSearch}
    />
  );
};
