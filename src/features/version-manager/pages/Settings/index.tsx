import { Button, Card, Form, Input, message, Switch } from 'antd';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';

import { safeInvoke } from '@/api/tauri.ts';
import { CommandEnum } from '@/core/constants/enum.ts';

export const Settings = () => {
  const { t } = useTranslation();

  const [form] = Form.useForm();

  useEffect(() => {
    const init = async () => {
      const { versionsPath, downloadPath, autoActivate } = await safeInvoke<
        Record<string, boolean | string>
      >(CommandEnum.GET_CONFIG_VALUES, {
        keys: [CommandEnum.VERSIONS_PATH, CommandEnum.DOWNLOAD_PATH, CommandEnum.AUTO_ACTIVATE],
      });

      form.setFieldsValue({
        req: {
          versionsPath,
          downloadPath,
          autoActivate,
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
        <Form.Item name={['req', 'downloadPath']} label={t('settings.download_path')}>
          <Input />
        </Form.Item>

        <Form.Item name={['req', 'versionsPath']} label={t('settings.versions_path')}>
          <Input />
        </Form.Item>

        <Form.Item
          name={['req', 'autoActivate']}
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
