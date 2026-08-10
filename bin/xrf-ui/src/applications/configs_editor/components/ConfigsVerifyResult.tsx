import { GridColDef } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { CommandResult, ICommandResultStat } from "@/core/components/result/CommandResult";
import { CommandResultFindings } from "@/core/components/result/CommandResultFindings";
import { ILtxProjectVerifyError, ILtxProjectVerifyResult } from "@/lib/ltx";
import { formatDuration } from "@/lib/result";

interface IConfigsVerifyResultProps {
  result: ILtxProjectVerifyResult;
}

export function ConfigsVerifyResult({ result }: IConfigsVerifyResultProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "section", headerName: "Section", width: 180, cellClassName: "monospace" },
      { field: "field", headerName: "Field", width: 150, cellClassName: "monospace" },
      { field: "message", headerName: "Problem", flex: 1, minWidth: 220 },
      { field: "at", headerName: "Location", width: 220, cellClassName: "monospace" },
    ],
    []
  );

  const stats: Array<ICommandResultStat> = useMemo(
    () => [
      { label: "files", value: result.totalFiles },
      { label: "sections checked", value: result.checkedSections },
      { label: "fields checked", value: result.checkedFields },
      { label: "valid", value: result.validSections, tone: "success" },
      { label: "skipped", value: result.skippedSections },
      { label: "invalid", value: result.invalidSections, tone: result.invalidSections ? "error" : "success" },
      { label: "elapsed", value: formatDuration(result.duration) },
    ],
    [result]
  );

  return (
    <CommandResult
      headline={
        result.errors.length
          ? `${result.errors.length} problem(s) found in ${result.invalidSections} section(s)`
          : "All sections passed validation"
      }
      tone={result.errors.length ? "error" : "success"}
      stats={stats}
    >
      <CommandResultFindings<ILtxProjectVerifyError>
        rows={result.errors}
        columns={columns}
        getRowId={(row) => `${row.at}:${row.section}:${row.field}`}
        getSearchText={(row) => `${row.section} ${row.field} ${row.message} ${row.at}`}
        emptyLabel={"Nothing to report."}
        searchPlaceholder={"Filter by section, field or file"}
      />
    </CommandResult>
  );
}
