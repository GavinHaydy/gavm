// src/layouts/BasicLayout/Sider.tsx
import { SettingTwoTone } from '@ant-design/icons';
import { Menu, Button, Select, MenuProps } from 'antd';
import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { useLocation, useNavigate } from 'react-router-dom';

import { LangEnum } from '@/core/constants/enum';
import i18n from '@/features/i18n';
import { setMode } from '@/features/theme/themeSlice';
import { IconFont } from '@/shared/components/IconFont';
import { saveTheme } from '@/shared/utils/tauriStore';
import { type RootState } from '@/store';

export const Sider: React.FC<{ collapsed: boolean; onCollapse: () => void }> = ({ collapsed }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const dispatch = useDispatch();
  const [selectedKey, setSelectedKey] = useState<string>(location.pathname);
  const mode = useSelector((state: RootState) => state.theme.mode);
  const [language, setLanguage] = useState<string>('zh');
  const { t } = useTranslation();

  useEffect(() => {
    setSelectedKey(location.pathname);
  }, [location.pathname]);

  const items = [
    { label: t('nav.go'), key: '/go', icon: <IconFont type="icon-golang" /> },
    { label: t('nav.java'), key: '/java', icon: <IconFont type="icon-java" /> },
    { label: t('nav.js'), key: '/js', icon: <IconFont type="icon-JavaScript" /> },
    { label: t('nav.python'), key: '/python', icon: <IconFont type="icon-python" /> },
    { label: t('nav.rust'), key: '/rust', icon: <IconFont type="icon-rust" /> },
    { label: t('nav.v'), key: '/v', icon: <IconFont type="icon-vlang" /> },
    { label: t('nav.zig'), key: '/zig', icon: <IconFont type="icon-zig" /> },
    { label: t('nav.settings'), key: '/settings', icon: <SettingTwoTone /> },
    { label: t('nav.downloader'), key: '/downloader', icon: <IconFont type="icon-downloader" /> },
  ];

  const handleMenuClick: MenuProps['onClick'] = async e => {
    await navigate(e.key);
  };

  const toggleTheme = async () => {
    const newMode = mode === 'light' ? 'dark' : 'light';

    dispatch(setMode(newMode));
    await saveTheme(newMode);
  };

  const handleLanguageChange = async (value: string) => {
    setLanguage(value);
    await i18n.changeLanguage(value);
  };

  return (
    <div style={{ width: collapsed ? 80 : 200, transition: 'width 0.3s' }}>
      <Menu
        mode="inline"
        selectedKeys={[selectedKey]}
        onClick={handleMenuClick}
        items={items.map(item => ({
          key: item.key,
          icon: item.icon,
          label: collapsed ? null : item.label,
        }))}
      />
      <div style={{ padding: '16px', textAlign: 'center' }}>
        <Button onClick={toggleTheme} block>
          {mode === 'dark' ? t('theme.light') : t('theme.dark')}
        </Button>
        <Select
          value={language}
          onChange={handleLanguageChange}
          style={{ width: '100%', marginTop: '16px' }}
          options={[
            { value: LangEnum.ZH, label: t('lang.zh') },
            { value: LangEnum.EN, label: t('lang.en') },
          ]}
        />
      </div>
    </div>
  );
};
