import { Box, Card, CardActionArea, Theme, Tooltip, Typography } from "@mui/material";
import { ReactElement, useCallback } from "react";

import { EApplicationStatus, IApplicationDescriptor, IApplicationGroup } from "@/core/routing/application";

export interface IApplicationLauncherCardProps {
  application: IApplicationDescriptor;
  group: IApplicationGroup;
  isEnabled: boolean;
  onOpen: (application: IApplicationDescriptor) => void;
}

/**
 * One application on the root catalog grid.
 *
 * Pointing at a card warms whatever chunk opening it would need. Intent runs a few hundred milliseconds
 * ahead of the click, which is longer than the load takes, so the split applications open as if they
 * were not split. Statically imported ones have no `preload` and nothing to do here.
 */
export function ApplicationLauncherCard({
  application,
  group,
  isEnabled,
  onOpen,
}: IApplicationLauncherCardProps): ReactElement {
  const isPlanned: boolean = application.status === EApplicationStatus.PLANNED;

  const onWarm = useCallback(() => {
    if (isEnabled) {
      // Nothing awaits this: the point is only that the fetch has started before the click.
      void application.preload?.();
    }
  }, [application, isEnabled]);

  const content: ReactElement = (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        gap: 0.5,
        height: "100%",
        padding: 1.25,
      }}
    >
      <Box sx={{ display: "flex", alignItems: "center", gap: 0.75, minWidth: 0 }}>
        <Box
          aria-hidden={true}
          sx={(theme: Theme) => ({
            display: "flex",
            flexShrink: 0,
            color: group.accent.light,
            "& .MuiSvgIcon-root": { fontSize: 18 },
            ...theme.applyStyles("dark", { color: group.accent.dark }),
          })}
        >
          {application.icon}
        </Box>

        <Typography
          variant={"subtitle2"}
          sx={{
            display: "-webkit-box",
            flexGrow: 1,
            minWidth: 0,
            color: "text.primary",
            WebkitBoxOrient: "vertical",
            WebkitLineClamp: 2,
            overflow: "hidden",
            lineHeight: 1.3,
          }}
        >
          {application.label}
        </Typography>

        {isPlanned ? (
          <Typography
            component={"span"}
            variant={"caption"}
            sx={{
              flexShrink: 0,
              alignSelf: "flex-start",
              paddingX: 0.75,
              color: "text.secondary",
              border: "1px solid",
              borderColor: "divider",
              borderRadius: 1,
              fontSize: "0.625rem",
              fontWeight: 600,
              lineHeight: "17px",
            }}
          >
            Planned
          </Typography>
        ) : null}
      </Box>

      <Typography
        variant={"body2"}
        sx={{
          display: "-webkit-box",
          // Two lines whether or not this one needs them, so cards keep a shared baseline across sections.
          minHeight: 32,
          color: "text.secondary",
          WebkitBoxOrient: "vertical",
          WebkitLineClamp: 2,
          overflow: "hidden",
          lineHeight: 1.35,
        }}
      >
        {application.description}
      </Typography>
    </Box>
  );

  return (
    <Card
      sx={{
        height: "100%",
        backgroundColor: "background.paper",
        transition: "background-color 140ms ease, border-color 140ms ease",
        ...(isEnabled
          ? {
              "&:hover": {
                backgroundColor: "action.hover",
                borderColor: "primary.main",
              },
            }
          : { opacity: 0.6 }),
      }}
    >
      {isEnabled ? (
        <CardActionArea
          aria-label={application.label}
          sx={{
            display: "block",
            height: "100%",
            "&.Mui-focusVisible": {
              outline: "2px solid",
              outlineColor: "primary.main",
              outlineOffset: -2,
            },
          }}
          onFocus={onWarm}
          onMouseEnter={onWarm}
          onClick={() => onOpen(application)}
        >
          {content}
        </CardActionArea>
      ) : (
        <Tooltip describeChild title={"Not implemented yet"}>
          <Box sx={{ height: "100%", cursor: "not-allowed" }}>{content}</Box>
        </Tooltip>
      )}
    </Card>
  );
}
