// src/layouts/BasicLayout/Sider.tsx
import { MenuFoldOutlined, MenuUnfoldOutlined, SettingOutlined } from '@ant-design/icons';
import { Menu, Button, Select, Tooltip, Popover } from 'antd';
import React, { useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { useLocation, useNavigate } from 'react-router-dom';
import { LangEnum } from '@/core/constants/enum';
import i18n from '@/features/i18n';
import { setMode } from '@/features/theme/themeSlice';
import { IconFont } from '@/shared/components/IconFont';
import { saveTheme } from '@/shared/utils/tauriStore';
import { type RootState } from '@/store';
import { routes } from '@/app/routes'; // 导入路由配置

interface ISiderProps {
  collapsed: boolean;
  onCollapse: (status: boolean) => void;
}

// 内置图标映射表（处理 SettingOutlined 这类 antd 图标）
const builtinIcons: Record<string, React.ReactNode> = {
  SettingOutlined: <SettingOutlined />,
};

export const Sider: React.FC<ISiderProps> = ({ collapsed, onCollapse }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const dispatch = useDispatch();
  const { t } = useTranslation();
  const mode = useSelector((state: RootState) => state.theme.mode);
  const [language, setLanguage] = React.useState<string>('zh');

  // 从路由配置生成菜单项
  const menuItems = useMemo(() => {
    // 递归提取菜单
    const extractMenus = (routeList: any[], parentPath = ''): any[] => {
      return routeList
        .filter(route => route.meta && !route.meta.hideInMenu) // 只显示有 meta 且未隐藏的
        .map(route => {
          // 处理路径（支持相对路径拼接）
          const fullPath = route.path.startsWith('/')
            ? route.path
            : `${parentPath}/${route.path}`.replace(/\/+/g, '/');

          // 处理图标
          let icon = null;
          if (route.meta.icon) {
            if (builtinIcons[route.meta.icon]) {
              icon = builtinIcons[route.meta.icon];
            } else {
              icon = <IconFont type={route.meta.icon} />;
            }
          }

          const item: any = {
            key: fullPath,
            label: t(route.meta.label),
            icon,
          };

          // 递归处理子路由（如果有）
          if (route.children) {
            const children = extractMenus(route.children, fullPath);
            if (children.length > 0) {
              item.children = children;
            }
          }

          return item;
        });
    };

    // 从 BasicLayout 的子路由中提取（routes[0] 是 layout 路由）
    const layoutRoute = routes[0];
    return layoutRoute?.children ? extractMenus(layoutRoute.children) : [];
  }, [t]);

  // 主题、语言切换等逻辑保持不变...
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
    <div style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      {/* 菜单 - 现在使用动态生成的 menuItems */}
      <Menu
        mode="inline"
        selectedKeys={[location.pathname]}
        onClick={e => navigate(e.key)}
        items={menuItems}
      />

      <div style={{ flex: 1 }} />

      {/* 底部操作区代码保持不变... */}
      <div style={{ padding: 16 }}>
        {collapsed ? (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 12, alignItems: 'center' }}>
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
              <Button type="text" shape="circle" style={{ width: 40, height: 40 }}>
                {language === LangEnum.ZH ? '中' : 'EN'}
              </Button>
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
