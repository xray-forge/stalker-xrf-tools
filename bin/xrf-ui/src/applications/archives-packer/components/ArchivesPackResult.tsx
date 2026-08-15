import { GridColDef } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { ArchivePackResult } from "@/core/bindings/xrf-archive";
import { CommandResult, ICommandResultStat } from "@/core/ui/command-result/CommandResult";
import { CommandResultFindings } from "@/core/ui/command-result/CommandResultFindings";
import { formatDuration } from "@/lib/format/duration";
import { bytesToMegabytes } from "@/lib/format/memory";

interface IArchivesPackResultProps {
  result: ArchivePackResult;
}

export function ArchivesPackResult({ result }: IArchivesPackResultProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [{ field: "volume", headerName: "Volume", flex: 1, minWidth: 320, cellClassName: "monospace" }],
    []
  );

  const rows: Array<{ volume: string }> = useMemo(
    () => result.volumes.map((volume) => ({ volume })),
    [result.volumes]
  );

  const stats: Array<ICommandResultStat> = useMemo(
    () => [
      { label: "volumes", value: result.volumes.length },
      { label: "packed", value: result.filesTotal },
      { label: "compressed", value: result.filesCompressed },
      { label: "stored", value: result.filesStored },
      // Aliased and skipped are the two counts that explain a surprising size or a missing file.
      { label: "aliased", value: result.filesAliased },
      { label: "skipped", value: result.filesSkipped },
      { label: "source", value: `${bytesToMegabytes(result.sizeSource).toFixed(1)} MB` },
      { label: "written", value: `${bytesToMegabytes(result.sizeWritten).toFixed(1)} MB` },
      { label: "elapsed", value: formatDuration(result.duration) },
    ],
    [result]
  );

  return (
    <CommandResult
      headline={`Packed ${result.filesTotal} file(s) into ${result.volumes.length} volume(s)`}
      tone={"success"}
      stats={stats}
    >
      <CommandResultFindings<{ volume: string }>
        rows={rows}
        columns={columns}
        getRowId={(row) => row.volume}
        getSearchText={(row) => row.volume}
        emptyLabel={"No volumes were written."}
        searchPlaceholder={"Filter by volume"}
      />
    </CommandResult>
  );
}
