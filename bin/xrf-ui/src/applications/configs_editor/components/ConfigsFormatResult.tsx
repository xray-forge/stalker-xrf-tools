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
        <Chip variant={"outlined"} label={`${result.totalFiles} files total`} />
      </Grid>

      <Grid justifyContent={"center"} gap={1} marginTop={1} padding={`0 ${16}px`} container>
        <Chip variant={"outlined"} label={`${result.validFiles} file(s) valid`} />
        <Chip
          variant={"outlined"}
          color={result.invalidFiles ? (isCheck ? "error" : "warning") : "success"}
          label={isCheck ? `${result.invalidFiles} file(s) to format` : `${result.invalidFiles} file(s) formatted`}
        />
      </Grid>

      {result.toFormat.length ? (
        <Grid marginTop={2}>
          <Accordion>
            <AccordionSummary>
              {isCheck ? `Invalid (${result.toFormat.length})` : `Formatted (${result.toFormat.length})`}
            </AccordionSummary>
            <AccordionDetails sx={{ maxHeight: 300, overflowY: "auto" }}>
              {result.toFormat.map((it, index) => (
                <Grid key={index}>
                  <Typography color={"green"}>
                    ({index + 1}) ({it})
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
