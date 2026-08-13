import { default as SearchIcon } from "@mui/icons-material/Search";
import { Box, InputAdornment, TextField, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ChangeEvent, Fragment, ReactElement, useCallback, useMemo, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { APPLICATION_CATALOG } from "@/ApplicationCatalog";
import { EApplicationStatus, IApplicationDescriptor, IApplicationGroup } from "@/core/routing/application";
import { ApplicationCard } from "@/core/routing/components/ApplicationCard";
import { SettingsService } from "@/core/settings/services/settings";
import { EditorLayout } from "@/core/shell/editor/EditorLayout";
import { EditorToolbar } from "@/core/shell/editor/EditorToolbar";

/**
 * Column counts at chosen widths rather than wherever a `minmax` happens to divide.
 */
const GRID_COLUMNS = {
  xs: "repeat(1, minmax(0, 1fr))",
  sm: "repeat(2, minmax(0, 1fr))",
  md: "repeat(3, minmax(0, 1fr))",
  lg: "repeat(4, minmax(0, 1fr))",
  xl: "repeat(5, minmax(0, 1fr))",
} as const;

/**
 * Start page, and the only route between applications.
 */
export function ApplicationRoot(): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { applications, groups } = APPLICATION_CATALOG;

  const settingsService: SettingsService = useInjection(SettingsService);

  const [query, setQuery] = useState<string>("");

  const isDevModeEnabled: boolean = settingsService.isDevModeEnabled;

  const matched: ReadonlyArray<IApplicationDescriptor> = useMemo(() => {
    const needle: string = query.trim().toLowerCase();

    return needle
      ? applications.filter(
          (application: IApplicationDescriptor) =>
            application.label.toLowerCase().includes(needle) || application.description.toLowerCase().includes(needle)
        )
      : applications;
  }, [applications, query]);

  const sections: Array<[IApplicationGroup, Array<IApplicationDescriptor>]> = useMemo(
    () =>
      groups
        .map((group: IApplicationGroup): [IApplicationGroup, Array<IApplicationDescriptor>] => [
          group,
          matched
            .filter((it: IApplicationDescriptor) => it.group === group.id)
            .sort((left: IApplicationDescriptor, right: IApplicationDescriptor) => {
              if (left.status !== right.status) {
                return left.status === EApplicationStatus.READY ? -1 : 1;
              }

              return left.label.localeCompare(right.label);
            }),
        ])
        .filter(([, applications]) => applications.length > 0),
    [groups, matched]
  );

  const onChangeQuery = useCallback((event: ChangeEvent<HTMLInputElement>) => setQuery(event.target.value), []);

  const onOpen = useCallback(
    (application: IApplicationDescriptor) => navigate(application.path, { replace: true }),
    [navigate]
  );

  return (
    <EditorLayout toolbar={<EditorToolbar />}>
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Box sx={{ maxWidth: 1600 }}>
          <TextField
            fullWidth
            size={"small"}
            value={query}
            placeholder={"Filter applications"}
            slotProps={{
              input: {
                startAdornment: (
                  <InputAdornment position={"start"}>
                    <SearchIcon fontSize={"small"} />
                  </InputAdornment>
                ),
              },
              htmlInput: { "aria-label": "Filter applications" },
            }}
            sx={{ marginBottom: 3, maxWidth: 420 }}
            onChange={onChangeQuery}
          />

          {sections.length ? (
            <Box sx={{ display: "grid", gridTemplateColumns: GRID_COLUMNS, columnGap: 2, rowGap: 2 }}>
              {sections.map(([group, applications]: [IApplicationGroup, Array<IApplicationDescriptor>]) => (
                <Fragment key={group.id}>
                  <Box
                    sx={{
                      gridColumn: "1 / -1",
                      display: "flex",
                      alignItems: "center",
                      gap: 1,
                      marginTop: 1,
                      color: "text.secondary",
                    }}
                  >
                    <Box sx={{ display: "flex" }}>{group.icon}</Box>
                    <Typography variant={"subtitle2"}>{group.label}</Typography>
                  </Box>

                  {applications.map((application: IApplicationDescriptor) => (
                    <ApplicationCard
                      key={application.id}
                      application={application}
                      isEnabled={application.status === EApplicationStatus.READY || isDevModeEnabled}
                      onOpen={onOpen}
                    />
                  ))}
                </Fragment>
              ))}
            </Box>
          ) : (
            <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
              Nothing matches {`"${query}"`}.
            </Typography>
          )}
        </Box>
      </Box>
    </EditorLayout>
  );
}
