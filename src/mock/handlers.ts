import { mockConfig } from './config';
import { mockVersions } from './verison';

import { CommandEnum } from '@/core/constants/enum';

export const mockHandlers = {
  [CommandEnum.GET_CONFIG_VALUES]: () => mockConfig,
  [CommandEnum.LIST_VERSIONS]: () => mockVersions,
};
