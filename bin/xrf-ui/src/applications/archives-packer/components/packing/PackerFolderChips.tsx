import { Chip, Stack } from "@mui/material";
import { ReactElement } from "react";

import { ArchivePackFolder } from "@/core/bindings/xrf-archive";

interface IPackerFolderChipsProps {
  folders: Array<ArchivePackFolder>;
  /** Said after a folder that carries its subfolders, since the flag is what a rule turns on. */
  recursiveSuffix: string;
}

/** Folder rules as chips, each saying whether it reaches below itself. */
export function PackerFolderChips({ folders, recursiveSuffix }: IPackerFolderChipsProps): ReactElement {
  return (
    <Stack direction={"row"} spacing={0.5} sx={{ flexWrap: "wrap", gap: 0.5 }}>
      {folders.map((folder, index) => (
        <Chip
          key={index}
          size={"small"}
          label={`${folder.path || "(root)"}${folder.isRecursive ? recursiveSuffix : ""}`}
          variant={"outlined"}
        />
      ))}
    </Stack>
  );
}
