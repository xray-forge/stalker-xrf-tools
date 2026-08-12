import { Box, Card, CardActionArea, Tooltip, Typography } from "@mui/material";
import { ReactElement, useCallback } from "react";

import { IApplicationDescriptor } from "@/core/router/application";

export interface IApplicationCardProps {
  application: IApplicationDescriptor;
  isEnabled: boolean;
  onOpen: (application: IApplicationDescriptor) => void;
}

/**
 * One application on the home grid.
 *
 * Pointing at a card warms whatever chunk opening it would need. Intent runs a few hundred milliseconds
 * ahead of the click, which is longer than the load takes, so the split applications open as if they
 * were not split. Statically imported ones have no `preload` and nothing to do here.
 */
export function ApplicationCard({ application, isEnabled, onOpen }: IApplicationCardProps): ReactElement {
  const onWarm = useCallback(() => {
    if (isEnabled) {
      // Nothing awaits this: the point is only that the fetch has started before the click.
      void application.preload?.();
    }
  }, [application, isEnabled]);

  const card: ReactElement = (
    <Card sx={{ display: "flex", flexDirection: "column", opacity: isEnabled ? 1 : 0.5 }}>
      <CardActionArea
        disabled={!isEnabled}
        sx={{ flexGrow: 1, padding: 2 }}
        onFocus={onWarm}
        onMouseEnter={onWarm}
        onClick={() => onOpen(application)}
      >
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
