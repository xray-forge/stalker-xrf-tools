import { SxProps, Theme } from "@mui/material/styles";

export interface BaseComponentProps {
  ["data-testid"]?: string;
  id?: string;
  className?: string;
  sx?: SxProps<Theme>;
}
