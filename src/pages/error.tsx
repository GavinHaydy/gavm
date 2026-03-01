import { Result, Button } from 'antd';
import { useTranslation } from 'react-i18next';
import { useRouteError, isRouteErrorResponse } from 'react-router-dom';
import { useNavigate } from 'react-router-dom';

export const ErrorPage = () => {
  const error = useRouteError();
  const navigate = useNavigate();
  const { t } = useTranslation();

  let title = t('error.title');
  let subTitle = t('error.subtitle');

  if (isRouteErrorResponse(error)) {
    title = `${error.status}`;
    subTitle = error.statusText;
  } else if (error instanceof Error) {
    subTitle = error.message;
  }

  return (
    <Result
      status="error"
      title={title}
      subTitle={subTitle}
      extra={
        <Button type="primary" onClick={() => navigate('/')}>
          {t('error.backHome')}
        </Button>
      }
    />
  );
};
