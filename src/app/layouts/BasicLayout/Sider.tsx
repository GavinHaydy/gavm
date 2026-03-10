// src/layouts/BasicLayout/Sider.tsx
import { MenuFoldOutlined, MenuUnfoldOutlined, SettingOutlined } from '@ant-design/icons';
import { Menu, Button, Select, MenuProps, Tooltip, Popover } from 'antd';
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

interface ISiderProps {
  collapsed: boolean;
  onCollapse: (status: boolean) => void;
}

export const Sider: React.FC<ISiderProps> = ({ collapsed, onCollapse }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const dispatch = useDispatch();
  const { t } = useTranslation();

  const [selectedKey, setSelectedKey] = useState<string>(location.pathname);
  const mode = useSelector((state: RootState) => state.theme.mode);
  const [language, setLanguage] = useState<string>('zh');

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
    { label: t('nav.settings'), key: '/settings', icon: <SettingOutlined /> },
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
    <div
      style={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      {/* 菜单 */}
      <Menu mode="inline" selectedKeys={[selectedKey]} onClick={handleMenuClick} items={items} />

      <div style={{ flex: 1 }} />

      {/* 底部操作区 */}
      <div style={{ padding: 16 }}>
        {collapsed ? (
          /* 折叠状态：纵向 */
          <div
            style={{
              display: 'flex',
              flexDirection: 'column',
              gap: 12,
              alignItems: 'center',
            }}
          >
            <Tooltip title={mode === 'dark' ? t('theme.light') : t('theme.dark')} placement="right">
              <Button
                type="text"
                shape="circle"
                style={{ width: 40, height: 40 }}
                icon={mode === 'dark' ? '☀️' : '🌙'}
                onClick={toggleTheme}
              />
            </Tooltip>

            <Popover
              content={
                <Select
                  value={language}
                  onChange={handleLanguageChange}
                  style={{ width: 120 }}
                  options={[
                    { value: LangEnum.ZH, label: t('lang.zh') },
                    { value: LangEnum.EN, label: t('lang.en') },
                  ]}
                />
              }
              placement="right"
            >
              <Button
                type="text"
                shape="circle"
                style={{ width: 40, height: 40 }}
                icon={<span>{language === LangEnum.ZH ? '中' : 'EN'}</span>}
              />
            </Popover>

            <Tooltip title={t('expand')} placement="right">
              <Button
                type="text"
                shape="circle"
                style={{ width: 40, height: 40 }}
                icon={<MenuUnfoldOutlined />}
                onClick={() => onCollapse(!collapsed)}
              />
            </Tooltip>
          </div>
        ) : (
          /* 展开状态：横向 */
          <div
            style={{
              display: 'flex',
              gap: 12,
              alignItems: 'center',
              justifyContent: 'space-between',
            }}
          >
            <Tooltip title={mode === 'dark' ? t('theme.light') : t('theme.dark')}>
              <Button type="text" icon={mode === 'dark' ? '☀️' : '🌙'} onClick={toggleTheme} />
            </Tooltip>

            <Tooltip title={t('lang.switch')}>
              <Select
                value={language}
                onChange={handleLanguageChange}
                style={{ width: 100 }}
                options={[
                  { value: LangEnum.ZH, label: t('lang.zh') },
                  { value: LangEnum.EN, label: t('lang.en') },
                ]}
              />
            </Tooltip>

            <Tooltip title={collapsed ? t('expand') : t('collapse')}>
              <Button
                type="text"
                icon={<MenuFoldOutlined />}
                onClick={() => onCollapse(!collapsed)}
              />
            </Tooltip>
          </div>
        )}
      </div>
    </div>
  );
};
