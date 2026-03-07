import { Button, Card, Form, Input, message, Switch } from 'antd';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';

import { safeInvoke } from '@/api/tauri.ts';
import { CommandEnum } from '@/core/constants/enum.ts';
import { keysToCamel } from '@/shared/utils/common';

export const Settings = () => {
  const { t } = useTranslation();

  const [form] = Form.useForm();

  useEffect(() => {
    const init = async () => {
      const configData = await safeInvoke<Record<string, boolean | string>>(
        CommandEnum.GET_CONFIG_VALUES,
        {
          keys: [CommandEnum.VERSIONS_PATH, CommandEnum.DOWNLOAD_PATH, CommandEnum.AUTO_ACTIVATE],
        },
      );

      const { versionsPath, downloadPath, autoActivate } = keysToCamel(configData);

      form.setFieldsValue({
        newValues: {
          download_path: downloadPath,
          versions_path: versionsPath,
          auto_activate: autoActivate,
        },
      });
    };

    void init();
  }, []);

  const handleSave = async (values: Record<string, unknown>) => {
    try {
      await safeInvoke(CommandEnum.UPDATE_CONFIG, values);
      message.success(t('settings.success'));
    } catch (error) {
      message.error((error as Error).message);
    }
  };

  return (
    <Card title={t('settings.title')}>
      <Form form={form} layout="vertical" onFinish={handleSave}>
        <Form.Item name={['newValues', 'download_path']} label={t('settings.download_path')}>
          <Input />
        </Form.Item>

        <Form.Item name={['newValues', 'versions_path']} label={t('settings.versions_path')}>
          <Input />
        </Form.Item>

        <Form.Item
          name={['newValues', 'auto_activate']}
          label={t('settings.auto_activate')}
          valuePropName="checked"
        >
          <Switch />
        </Form.Item>

        <Button type="primary" htmlType="submit">
          {t('settings.save')}
        </Button>
      </Form>
    </Card>
  );
};
