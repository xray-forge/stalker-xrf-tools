import { Box, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ISpawnRowSelection, SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { EmptyState } from "@/core/components/layout/EmptyState";
import { Nullable } from "@/core/types/general";

import { formatSpawnRowDetailsValue } from "./SpawnRowDetailsPanel.utils";

export interface ISpawnRowDetailsPanelProps {
  spawnFileService: SpawnFileService;
}

/**
 * Everything about the selected row.
 *
 * The tables keep their columns terse because this exists: a spawn record has more fields than fit on a
 * screen, and most of them are only wanted once you have found the row you care about.
 */
export function SpawnRowDetailsPanel({ spawnFileService }: ISpawnRowDetailsPanelProps): ReactElement {
  const selection: Nullable<ISpawnRowSelection> = spawnFileService.selectedRow;

  if (!selection) {
    return <EmptyState title={"Nothing selected"} description={"Pick a row in any chunk table to inspect it here."} />;
  }

  const entries: Array<[string, unknown]> = Object.entries(selection.row);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", minHeight: 0 }}>
      <Box sx={{ paddingX: 1.5, paddingY: 1, borderBottom: 1, borderColor: "divider" }}>
        <Typography variant={"subtitle2"}>{selection.source}</Typography>
      </Box>

      <Box sx={{ flexGrow: 1, minHeight: 0, overflowY: "auto" }}>
        {entries.map(([key, value]: [string, unknown]) => (
          <Box key={key} sx={{ paddingX: 1.5, paddingY: 0.75, borderBottom: 1, borderColor: "divider" }}>
            <Typography variant={"caption"} sx={{ display: "block", color: "text.secondary" }}>
              {key}
            </Typography>

            <Typography
              variant={"body2"}
              sx={{ fontFamily: "'Cascadia Mono', 'Consolas', monospace", overflowWrap: "anywhere" }}
            >
              {formatSpawnRowDetailsValue(value)}
            </Typography>
          </Box>
        ))}
      </Box>
    </Box>
  );
}
