import { Box, Card, CardActionArea, Tooltip, Typography } from "@mui/material";
import { ReactElement } from "react";

import { IApplication } from "@/core/router/application";

export interface IApplicationCardProps {
  application: IApplication;
  isEnabled: boolean;
  onOpen: (application: IApplication) => void;
}

/**
 * One application on the home grid.
 */
export function ApplicationCard({ application, isEnabled, onOpen }: IApplicationCardProps): ReactElement {
  const card: ReactElement = (
    <Card sx={{ display: "flex", flexDirection: "column", opacity: isEnabled ? 1 : 0.5 }}>
      <CardActionArea disabled={!isEnabled} sx={{ flexGrow: 1, padding: 2 }} onClick={() => onOpen(application)}>
        <Box sx={{ display: "flex", alignItems: "center", gap: 1, marginBottom: 0.5 }}>
          <Box sx={{ display: "flex", color: "primary.main" }}>{application.icon}</Box>
          <Typography variant={"subtitle2"}>{application.label}</Typography>
        </Box>

        <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
          {application.description}
        </Typography>
      </CardActionArea>
    </Card>
  );

  // A disabled control cannot receive the tooltip's events, so the card is wrapped rather than
  // targeted directly.
  return isEnabled ? (
    card
  ) : (
    <Tooltip describeChild title={"Not implemented yet. Developer mode opens it anyway."}>
      <span>{card}</span>
    </Tooltip>
  );
}
