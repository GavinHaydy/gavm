import { message } from 'antd';
import { useEffect, useState } from 'react';

import { safeInvoke } from '@/api/tauri';
import { CommandEnum, InstallStatusEnum, LanguageEnum } from '@/core/constants/enum';
import { SearchPayload, VersionItem, VersionResult } from '@/core/types/common.ts';
import { VersionTable } from '@/shared/components/VersionTable';

interface LanguageManagePageProps {
  language: LanguageEnum;
}

export const LanguageManagePage = ({ language }: LanguageManagePageProps) => {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<VersionResult>({
    total: 0,
    list: [],
    page: 0,
    pageSize: 10,
  });
  const [searchPayload, setSearchPayload] = useState<SearchPayload>({
    language,
    page: 0,
    pageSize: 10,
    keyWord: '',
  });

  useEffect(() => {
    setSearchPayload(prevState => ({ ...prevState, language, page: 0 }));
  }, [language]);

  useEffect(() => {
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

    void getList();
  }, [searchPayload]);

  const handleSearch = (keyWord: string) => {
    setSearchPayload(prevState => ({ ...prevState, keyWord, page: 0 }));
  };

  const handlePageChange = (page: number, pageSize: number) => {
    setSearchPayload(prevState => ({ ...prevState, page, pageSize }));
  };

  const handleVersionAction = async (
    command: CommandEnum | InstallStatusEnum,
    record: VersionItem,
  ) => {
    try {
      setLoading(true);
      await safeInvoke(command, { language, version: record.version });
      setSearchPayload(prevState => ({ ...prevState, page: 0 })); // Reset to page 0 after action
    } catch (error) {
      message.error((error as Error).message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <VersionTable
      loading={loading}
      data={data}
      handleVersionAction={handleVersionAction}
      onSearch={handleSearch}
      handlePageChange={handlePageChange}
    />
  );
};
