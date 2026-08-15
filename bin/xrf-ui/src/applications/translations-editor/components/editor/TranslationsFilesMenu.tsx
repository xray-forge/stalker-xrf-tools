import { default as CircleIcon } from "@mui/icons-material/Circle";
import { Box, Typography } from "@mui/material";
import { ReactElement, useMemo } from "react";

import { TranslationFile } from "@/core/bindings/xrf-app-translations";
import { ISearchResult, IUseRankedSearch, useRankedSearch } from "@/core/search/lib";
import { EditorSearchHeader } from "@/core/shell/editor/EditorSearchHeader";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/shell/editor/EditorSideMenu";
import { Nullable } from "@/lib/types/general";

/** One file, with enough on it to rank a search and label a row. */
interface IFileEntry {
  name: string;
  entryCount: number;
  isDirty: boolean;
}

export interface ITranslationsFilesMenuProps {
  files: Record<string, TranslationFile>;
  dirtyFiles: ReadonlyArray<string>;
  selected: Nullable<string>;
  onSelect: (file: string) => void;
}

export function TranslationsFilesMenu({
  files,
  dirtyFiles,
  selected,
  onSelect,
}: ITranslationsFilesMenuProps): ReactElement {
  const entries: Array<IFileEntry> = useMemo(
    () =>
      Object.entries(files).map(([name, file]: [string, TranslationFile]) => ({
        name,
        entryCount: Object.keys(file.entries).length,
        isDirty: dirtyFiles.includes(name),
      })),
    [dirtyFiles, files]
  );

  const search: IUseRankedSearch<IFileEntry> = useRankedSearch({
    items: entries,
    toSearchText: (it: IFileEntry) => it.name,
    onSelect: (it: IFileEntry) => onSelect(it.name),
  });

  const visible: Array<IFileEntry> = search.isSearching
    ? search.results.map((result: ISearchResult<IFileEntry>) => result.item)
    : entries;

  const sections: Array<IEditorSideMenuItem> = visible.map((it: IFileEntry) => ({
    label: it.name,
    description: `${it.entryCount} entries`,
    isSelected: it.name === selected,
    icon: it.isDirty ? (
      <CircleIcon aria-label={"Unsaved changes"} sx={{ fontSize: 8, color: "warning.main" }} />
    ) : undefined,
    onClick: () => onSelect(it.name),
  }));

  return (
    <EditorSideMenu
      header={
        <EditorSearchHeader
          title={"Files"}
          count={entries.length}
          query={search.query}
          placeholder={"Filter files"}
          ariaLabel={"Filter translation files"}
          onClear={search.clear}
          onKeyDown={search.onInputKeyDown}
          onQueryChange={search.setQuery}
        />
      }
      sections={sections}
    >
      {visible.length ? null : (
        <Box sx={{ padding: 2, textAlign: "center" }}>
          <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
            No files match {search.query.trim()}.
          </Typography>
        </Box>
      )}
    </EditorSideMenu>
  );
}
