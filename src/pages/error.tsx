import { useRouteError, isRouteErrorResponse } from "react-router-dom";
import { Result, Button } from "antd";
import { useNavigate } from "react-router-dom";

export const ErrorPage = () => {
  const error = useRouteError();
  const navigate = useNavigate();

  let title = "Something went wrong";
  let subTitle = "Unknown error";

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
        <Button type="primary" onClick={() => navigate("/")}>
          Back Home
        </Button>
      }
    />
  );
};