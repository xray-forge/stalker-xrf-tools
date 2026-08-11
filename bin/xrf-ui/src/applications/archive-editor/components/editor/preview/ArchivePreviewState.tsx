import { default as DescriptionOutlinedIcon } from "@mui/icons-material/DescriptionOutlined";
import { Typography } from "@mui/material";
import { ReactElement } from "react";

import { CenteredColumn } from "@/core/components/layout/CenteredColumn";
import { BaseComponentProps } from "@/lib/dom/element-types";

interface IArchivePreviewStateProps extends BaseComponentProps {
  description: string;
  title: string;
}

export function ArchivePreviewState({ description, title }: IArchivePreviewStateProps): ReactElement {
  return (
    <CenteredColumn sx={{ padding: 3, textAlign: "center" }}>
      <DescriptionOutlinedIcon sx={{ fontSize: 40, color: "text.secondary", opacity: 0.55 }} />

      <Typography variant={"subtitle1"}>{title}</Typography>

      <Typography variant={"body2"} sx={{ maxWidth: 440, color: "text.secondary" }}>
        {description}
      </Typography>
    </CenteredColumn>
  );
}
