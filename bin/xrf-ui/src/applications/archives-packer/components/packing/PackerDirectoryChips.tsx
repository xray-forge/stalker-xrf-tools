import { Chip, Stack } from "@mui/material";
import { ReactElement } from "react";

import { ArchivePackDirectory } from "@/core/bindings/types/xrf-pack";

interface IPackerDirectoryChipsProps {
  directories: Array<ArchivePackDirectory>;
  /** Said after a directory that carries its subdirectories, since the flag is what a rule turns on. */
  recursiveSuffix: string;
}

/** Directory rules as chips, each saying whether it reaches below itself. */
export function PackerDirectoryChips({ directories, recursiveSuffix }: IPackerDirectoryChipsProps): ReactElement {
  return (
    <Stack direction={"row"} spacing={0.5} sx={{ flexWrap: "wrap", gap: 0.5 }}>
      {directories.map((directory, index) => (
        <Chip
          key={index}
          size={"small"}
          label={`${directory.path || "(root)"}${directory.isRecursive ? recursiveSuffix : ""}`}
          variant={"outlined"}
        />
      ))}
    </Stack>
  );
}
