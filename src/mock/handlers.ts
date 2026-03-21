import { mockConfig } from './config';
import { mockNodeVersions, mockVersions } from './verison';

import { CommandEnum, LanguageEnum } from '@/core/constants/enum';
import { SearchPayload } from '@/core/types/common';

export const mockHandlers = {
  [CommandEnum.GET_CONFIG_VALUES]: () => mockConfig,
  [CommandEnum.LIST_VERSIONS]: (args?: SearchPayload) => {
    if (args?.language === LanguageEnum.NODE) {
      return mockNodeVersions;
    }
    return mockVersions;
  },
};
