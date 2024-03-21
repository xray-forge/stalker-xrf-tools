import { Accordion, AccordionDetails, AccordionSummary, Chip, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ILtxProjectFormatResult } from "@/lib/ltx";

interface IConfigsVerifyResultProps {
  isCheck: boolean;
  result: ILtxProjectFormatResult;
}

export function ConfigsFormatResult({ isCheck, result }: IConfigsVerifyResultProps): ReactElement {
  return (
    <Grid direction={"column"} padding={2} maxWidth={540} container>
      <Grid justifyContent={"center"} gap={1} container>
        <Chip variant={"outlined"} label={`${result.duration / 1000} sec`} />
        <Chip variant={"outlined"} label={`${result.total_files} files total`} />
      </Grid>

      <Grid justifyContent={"center"} gap={1} marginTop={1} padding={`0 ${16}px`} container>
        <Chip variant={"outlined"} label={`${result.valid_files} file(s) valid`} />
        <Chip
          variant={"outlined"}
          color={result.invalid_files ? (isCheck ? "error" : "warning") : "success"}
          label={isCheck ? `${result.invalid_files} file(s) to format` : `${result.invalid_files} file(s) formatted`}
        />
      </Grid>

      {result.to_format.length ? (
        <Grid marginTop={2}>
          <Accordion>
            <AccordionSummary>
              {isCheck ? `Invalid (${result.to_format.length})` : `Formatted (${result.to_format.length})`}
            </AccordionSummary>
            <AccordionDetails sx={{ maxHeight: 300, overflowY: "auto" }}>
              {result.to_format.map((it, index) => (
                <Grid key={index}>
                  <Typography color={"green"}>
                    ({index + 1}) {it}
                  </Typography>
                </Grid>
              ))}
            </AccordionDetails>
          </Accordion>
        </Grid>
      ) : null}
    </Grid>
  );
}
