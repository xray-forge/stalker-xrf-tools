import { GridColDef } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { CommandResult, ICommandResultStat } from "@/core/components/result/CommandResult";
import { CommandResultFindings } from "@/core/components/result/CommandResultFindings";
import { ArchiveUnpackResult } from "@/lib/bindings/xray-archive";
import { formatDuration } from "@/lib/result";
import { bytesToMegabytes } from "@/lib/size";

interface IArchivesUnpackResultProps {
  result: ArchiveUnpackResult;
}

export function ArchivesUnpackResult({ result }: IArchivesUnpackResultProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [{ field: "archive", headerName: "Archive", flex: 1, minWidth: 320, cellClassName: "monospace" }],
    []
  );

  const rows: Array<{ archive: string }> = useMemo(
    () => result.archives.map((archive) => ({ archive })),
    [result.archives]
  );

  const stats: Array<ICommandResultStat> = useMemo(
    () => [
      { label: "archives", value: result.archives.length },
      { label: "unpacked", value: `${bytesToMegabytes(result.unpackedSize).toFixed(1)} MB` },
      { label: "prepare", value: formatDuration(result.prepareDuration) },
      { label: "unpack", value: formatDuration(result.unpackDuration) },
      { label: "elapsed", value: formatDuration(result.duration) },
    ],
    [result]
  );

  return (
    <CommandResult
      headline={`Unpacked ${result.archives.length} archive(s) to ${result.destination}`}
      tone={"success"}
      stats={stats}
    >
      <CommandResultFindings<{ archive: string }>
        rows={rows}
        columns={columns}
        getRowId={(row) => row.archive}
        getSearchText={(row) => row.archive}
        emptyLabel={"No archives were unpacked."}
        searchPlaceholder={"Filter by archive"}
      />
    </CommandResult>
  );
}
