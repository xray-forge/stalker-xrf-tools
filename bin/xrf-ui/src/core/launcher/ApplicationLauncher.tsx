import { default as SearchOffIcon } from "@mui/icons-material/SearchOff";
import { Box, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationLauncherCard } from "@/core/launcher/ApplicationLauncherCard";
import {
  ApplicationLauncherFilters,
  IApplicationLauncherGroupFilter,
} from "@/core/launcher/ApplicationLauncherFilters";
import { ApplicationLauncherSection } from "@/core/launcher/ApplicationLauncherSection";
import {
  EApplicationGroupId,
  EApplicationStatus,
  IApplicationDescriptor,
  IApplicationGroup,
} from "@/core/routing/application";
import { ISearchResult, IUseRankedSearch, useRankedSearch } from "@/core/search/lib";
import { SettingsService } from "@/core/settings/services/settings";
import { EditorLayout } from "@/core/shell/editor/EditorLayout";
import { EditorToolbar } from "@/core/shell/editor/EditorToolbar";
import { EmptyState } from "@/core/ui/layout/EmptyState";
import { Nullable } from "@/lib/types/general";

/**
 * Column counts at chosen widths rather than wherever a `minmax` happens to divide.
 */
const GRID_COLUMNS = {
  xs: "repeat(1, minmax(0, 1fr))",
  sm: "repeat(2, minmax(0, 1fr))",
  md: "repeat(3, minmax(0, 1fr))",
  lg: "repeat(3, minmax(0, 1fr))",
  xl: "repeat(4, minmax(0, 1fr))",
} as const;

/** One application together with the group it was found under, which search results no longer imply. */
type TCatalogEntry = [IApplicationDescriptor, IApplicationGroup];

interface ILauncherSection {
  group: IApplicationGroup;
  applications: Array<IApplicationDescriptor>;
}

export interface IApplicationLauncherProps {
  applications: ReadonlyArray<IApplicationDescriptor>;
  groups: ReadonlyArray<IApplicationGroup>;
}

/**
 * The searchable home surface for launching applications.
 */
export function ApplicationLauncher({ applications, groups }: IApplicationLauncherProps): ReactElement {
  const settingsService: SettingsService = useInjection(SettingsService);

  const navigate: NavigateFunction = useNavigate();

  const searchInputRef = useRef<Nullable<HTMLInputElement>>(null);

  const [selectedGroupId, setSelectedGroupId] = useState<Nullable<EApplicationGroupId>>(null);

  const sections: Array<ILauncherSection> = useMemo(
    () =>
      groups
        .map(
          (group: IApplicationGroup): ILauncherSection => ({
            group,
            applications: applications.filter((application: IApplicationDescriptor) => application.group === group.id),
          })
        )
        .filter((section: ILauncherSection) => section.applications.length > 0),
    [applications, groups]
  );

  const visibleSections: Array<ILauncherSection> = useMemo(
    () =>
      selectedGroupId ? sections.filter((section: ILauncherSection) => section.group.id === selectedGroupId) : sections,
    [sections, selectedGroupId]
  );

  const searchable: Array<TCatalogEntry> = useMemo(
    () =>
      visibleSections.flatMap(({ group, applications: grouped }: ILauncherSection) =>
        grouped.map((application: IApplicationDescriptor): TCatalogEntry => [application, group])
      ),
    [visibleSections]
  );

  const filters: Array<IApplicationLauncherGroupFilter> = useMemo(
    () => sections.map(({ group, applications: grouped }: ILauncherSection) => ({ group, count: grouped.length })),
    [sections]
  );

  const isEnabled = useCallback(
    (application: IApplicationDescriptor): boolean =>
      application.status === EApplicationStatus.READY || settingsService.isDevModeEnabled,
    [settingsService]
  );

  const onOpen = useCallback(
    (application: IApplicationDescriptor) => {
      navigate(application.path, { replace: true });
    },
    [navigate]
  );

  const onSelectResult = useCallback(
    ([application]: TCatalogEntry) => {
      if (isEnabled(application)) {
        onOpen(application);
      }
    },
    [isEnabled, onOpen]
  );

  const search: IUseRankedSearch<TCatalogEntry> = useRankedSearch({
    items: searchable,
    toSearchText: ([application]: TCatalogEntry) => application.label,
    // The group name matches too, so "icons" finds the six tools that never say it in their own label.
    toSecondaryText: ([application, group]: TCatalogEntry) => `${application.description} ${group.label}`,
    onSelect: onSelectResult,
  });

  useEffect(() => {
    function onWindowKeyDown(event: KeyboardEvent): void {
      const isEditing: boolean =
        event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement;

      if (((event.ctrlKey || event.metaKey) && event.key === "k") || (event.key === "/" && !isEditing)) {
        event.preventDefault();
        searchInputRef.current?.focus();
        searchInputRef.current?.select();
      }
    }

    window.addEventListener("keydown", onWindowKeyDown);

    return () => window.removeEventListener("keydown", onWindowKeyDown);
  }, []);

  return (
    <EditorLayout toolbar={<EditorToolbar />}>
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Box sx={{ width: "100%", maxWidth: 1600, marginX: "auto" }}>
          <Box sx={{ marginBottom: 2 }}>
            <Typography component={"h1"} variant={"h5"}>
              Applications
            </Typography>

            <Typography variant={"body2"} sx={{ color: "text.secondary", marginTop: 0.25 }}>
              {applications.length} tools across {groups.length} groups
            </Typography>
          </Box>

          <Box sx={{ marginBottom: 2.5 }}>
            <ApplicationLauncherFilters
              filters={filters}
              inputRef={searchInputRef}
              query={search.query}
              selectedGroupId={selectedGroupId}
              totalCount={applications.length}
              onClear={search.clear}
              onKeyDown={search.onInputKeyDown}
              onQueryChange={search.setQuery}
              onSelectGroup={setSelectedGroupId}
            />
          </Box>

          {search.isSearching ? (
            search.results.length ? (
              // Dimmed while a newer keystroke is still being filtered, so stale rows do not read as final.
              <Box sx={{ opacity: search.isStale ? 0.6 : 1, transition: "opacity 120ms ease" }}>
                <Typography variant={"caption"} sx={{ display: "block", marginBottom: 1, color: "text.secondary" }}>
                  {search.total} {search.total === 1 ? "match" : "matches"}
                </Typography>

                <Box sx={{ display: "grid", gridTemplateColumns: GRID_COLUMNS, gap: 1.5 }}>
                  {search.results.map(({ item: [application, group] }: ISearchResult<TCatalogEntry>) => (
                    <ApplicationLauncherCard
                      key={application.id}
                      application={application}
                      group={group}
                      isEnabled={isEnabled(application)}
                      onOpen={onOpen}
                    />
                  ))}
                </Box>
              </Box>
            ) : (
              <EmptyState
                icon={<SearchOffIcon sx={{ fontSize: 40, color: "text.secondary", opacity: 0.55 }} />}
                title={"No tools match"}
                description={`Nothing in the catalog matches ${search.query.trim()}.`}
              />
            )
          ) : (
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2.5 }}>
              {visibleSections.map(({ group, applications: grouped }: ILauncherSection) => (
                <Box key={group.id} sx={{ display: "flex", flexDirection: "column", gap: 1.25 }}>
                  <ApplicationLauncherSection group={group} count={grouped.length} />

                  <Box sx={{ display: "grid", gridTemplateColumns: GRID_COLUMNS, gap: 1.5 }}>
                    {grouped.map((application: IApplicationDescriptor) => (
                      <ApplicationLauncherCard
                        key={application.id}
                        application={application}
                        group={group}
                        isEnabled={isEnabled(application)}
                        onOpen={onOpen}
                      />
                    ))}
                  </Box>
                </Box>
              ))}
            </Box>
          )}
        </Box>
      </Box>
    </EditorLayout>
  );
}
