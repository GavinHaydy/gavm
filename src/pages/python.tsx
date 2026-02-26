import React, { useEffect, useState } from 'react';
import { VersionTable, VersionItem, VersionResult } from '@/pages/components/VersionTable';
import { safeInvoke } from '@/api/tauri';

export const PythonPage = () => {
  const [data, setData] = useState<VersionResult>({
    total: 0,
    list: [],
  });

  useEffect(() => {
    load();
  }, []);

  const load = async () => {
    const result = await safeInvoke<VersionResult>('list_versions', {
      language: 'python',
      page: 0,
      pageSize: 10,
    });
    console.log(result.total);
    setData(result);
  };

  const handleInstallToggle = async (record: VersionItem) => {
    if (!record.install_status) {
      await safeInvoke('install', {
        language: 'python',
        version: record.version,
      });
    } else {
      await safeInvoke('uninstall', {
        language: 'python',
        version: record.version,
      });
    }

    load();
  };

  const handleUseToggle = async (record: VersionItem) => {
    await safeInvoke('use_version', {
      language: 'python',
      version: record.version,
    });

    load();
  };

  return (
    <VersionTable data={data} onInstallToggle={handleInstallToggle} onUseToggle={handleUseToggle} />
  );
};
