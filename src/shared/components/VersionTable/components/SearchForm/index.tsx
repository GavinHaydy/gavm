import { Button, Form, Input, Select } from 'antd';
import React from 'react';
import { useTranslation } from 'react-i18next';

import { InstallStatusFilterEnum } from '@/core/constants/enum';
import type { SearchFormValues } from '@/core/types/common';

import './index.css';

interface SearchFormProps {
  onSearch: (values: SearchFormValues) => void;
  onReset?: () => void;
  initialValues?: SearchFormValues;
  loading?: boolean;
}

export const SearchForm: React.FC<SearchFormProps> = ({
  onSearch,
  onReset,
  initialValues,
  loading,
}) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();

  const handleSearch = () => {
    const values = form.getFieldsValue();
    onSearch(values);
  };

  const handleReset = () => {
    form.resetFields();
    onReset?.();
  };

  const installStatusOptions = [
    { value: InstallStatusFilterEnum.ALL, label: t('search.install_status_all') },
    { value: InstallStatusFilterEnum.INSTALLED, label: t('search.install_status_installed') },
    { value: InstallStatusFilterEnum.UNINSTALLED, label: t('search.install_status_uninstalled') },
  ];

  return (
    <Form form={form} className="search-form" initialValues={initialValues} layout="inline">
      <Form.Item name="keyword" label={t('search.keyword')}>
        <Input placeholder={t('search.placeholder')} />
      </Form.Item>
      <Form.Item name="installStatus" label={t('search.install_status')}>
        <Select options={installStatusOptions} style={{ width: 120 }} />
      </Form.Item>
      <Form.Item>
        <div className="search-form-button-group">
          <Button onClick={handleReset}>{t('search.reset')}</Button>
          <Button type="primary" onClick={handleSearch} loading={loading}>
            {t('search.button')}
          </Button>
        </div>
      </Form.Item>
    </Form>
  );
};
