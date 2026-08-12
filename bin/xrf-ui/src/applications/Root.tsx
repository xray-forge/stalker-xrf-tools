import { default as SearchIcon } from "@mui/icons-material/Search";
import { Box, InputAdornment, TextField, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ChangeEvent, ReactElement, useCallback, useMemo, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { ApplicationCard } from "@/core/components/navigation/ApplicationCard";
import { EApplicationStatus, IApplicationDescriptor, IApplicationGroup } from "@/core/router/application";
import { APPLICATION_GROUPS, APPLICATIONS } from "@/core/router/applications";
import { SettingsService } from "@/core/store/settings";

/**
 * Start page, and the only route between applications.
 *
 * It lists all of them rather than eight categories that each led to another list: five of those
 * categories held two entries or fewer, so the level in between was a click and nothing else. The
 * filter is here because this page now carries the traffic that a permanently visible rail list used
 * to.
 *
 * It carries the same toolbar as every other route, including this one - otherwise entering an
 * application from here shifted the content down by the toolbar's height.
 */
export function Root(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const settingsService: SettingsService = useInjection(SettingsService);

  const [query, setQuery] = useState<string>("");

  const isDevModeEnabled: boolean = settingsService.isDevModeEnabled;

  const matched: Array<IApplicationDescriptor> = useMemo(() => {
    const needle: string = query.trim().toLowerCase();

    return needle
      ? APPLICATIONS.filter(
          (application: IApplicationDescriptor) =>
            application.label.toLowerCase().includes(needle) || application.description.toLowerCase().includes(needle)
        )
      : APPLICATIONS;
  }, [query]);

  const onChangeQuery = useCallback((event: ChangeEvent<HTMLInputElement>) => setQuery(event.target.value), []);

  const onOpen = useCallback(
    (application: IApplicationDescriptor) => navigate(application.path, { replace: true }),
    [navigate]
  );

  return (
    <EditorLayout toolbar={<EditorToolbar />}>
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Typography variant={"body2"} sx={{ color: "text.secondary", marginBottom: 2 }}>
          Inspect and edit S.T.A.L.K.E.R. gamedata. Pick an application below.
        </Typography>

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

        {matched.length ? (
          APPLICATION_GROUPS.map((group: IApplicationGroup) => {
            const applications: Array<IApplicationDescriptor> = matched.filter(
              (it: IApplicationDescriptor) => it.group === group.id
            );

            if (!applications.length) {
              return null;
            }

            return (
              <Box key={group.id} sx={{ marginBottom: 3 }}>
                <Box sx={{ display: "flex", alignItems: "center", gap: 1, marginBottom: 1 }}>
                  <Box sx={{ display: "flex", color: "text.secondary" }}>{group.icon}</Box>
                  <Typography variant={"subtitle2"} sx={{ color: "text.secondary" }}>
                    {group.label}
                  </Typography>
                </Box>

                <Box sx={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(240px, 1fr))", gap: 2 }}>
                  {applications.map((application: IApplicationDescriptor) => (
                    <ApplicationCard
                      key={application.id}
                      application={application}
                      isEnabled={application.status === EApplicationStatus.READY || isDevModeEnabled}
                      onOpen={onOpen}
                    />
                  ))}
                </Box>
              </Box>
            );
          })
        ) : (
          <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
            Nothing matches {`"${query}"`}.
          </Typography>
        )}
      </Box>
    </EditorLayout>
  );
}
