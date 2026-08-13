import { Theme } from "@mui/material/styles";
import { SystemStyleObject } from "@mui/system";

export interface BaseComponentProps {
  ["data-testid"]?: string;
  id?: string;
  className?: string;
  sx?: SystemStyleObject<Theme>;
}
