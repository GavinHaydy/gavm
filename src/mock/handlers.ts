import { mockConfig } from './config';
import { mockNodeVersions, mockVersions } from './version';

import { CommandEnum, LanguageEnum } from '@/core/constants/enum';
import { SearchPayload } from '@/core/types/common';

export const mockHandlers = {
  [CommandEnum.GET_CONFIG_VALUES]: () => mockConfig,
  [CommandEnum.LIST_VERSIONS]: (args?: SearchPayload) => {
    const sourceData = args?.language === LanguageEnum.NODE ? mockNodeVersions : mockVersions;

    // 如果有过滤条件，进行过滤 (0: all, 1: installed, 2: uninstalled)
    if (args?.installStatus && args.installStatus !== 0) {
      const targetStatus = args.installStatus === 1;
      const filteredList = sourceData.data.list.filter(item => item.installStatus === targetStatus);
      return {
        ...sourceData,
        data: {
          ...sourceData.data,
          list: filteredList,
          total: filteredList.length,
        },
      };
    }

    return sourceData;
  },
};
