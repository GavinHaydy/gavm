export enum LangEnum {
  EN = 'en',
  ZH = 'zh',
}

export enum CommandEnum {
  UPDATE_CONFIG = 'update_configs',
  GET_CONFIG_VALUES = 'get_config_values',
  LIST_VERSIONS = 'list_versions',
  USE_VERSION = 'activate',
  UNUSE_VERSION = 'deactivate',
  DOWNLOAD_VERSION = 'download_version',
  DOWNLOAD_PATH = 'downloadPath',
  VERSIONS_PATH = 'versionsPath',
  AUTO_ACTIVATE = 'autoActivate',
  PROXY = 'proxy',
  RESET_SETTINGS = 'reset_settings',
}

export enum InstallStatusEnum {
  INSTALLED = 'install',
  UNINSTALLED = 'uninstall',
}

export enum LanguageEnum {
  PYTHON = 'python',
  GO = 'go',
  NODE = 'node',
}

export enum DownloadStatusEnum {
  SUCCESS = 'success',
  ERROR = 'error',
  DOWNLOADING = 'downloading',
}
