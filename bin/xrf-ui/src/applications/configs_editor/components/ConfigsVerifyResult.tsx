import { Accordion, AccordionDetails, AccordionSummary, Chip, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ILtxProjectVerifyResult } from "@/lib/ltx";

interface IConfigsVerifyResultProps {
  result: ILtxProjectVerifyResult;
}

export function ConfigsVerifyResult({ result }: IConfigsVerifyResultProps): ReactElement {
  return (
    <Grid direction={"column"} padding={2} maxWidth={540}>
      <Grid justifyContent={"center"} gap={1} container>
        <Chip variant={"outlined"} label={`${result.duration / 1000} sec`} />
        <Chip variant={"outlined"} label={`${result.total_files} files total`} />
        <Chip variant={"outlined"} label={`${result.total_sections} sections total`} />
      </Grid>

      <Grid justifyContent={"center"} gap={1} marginTop={1} padding={`0 ${16}px`} container>
        <Chip variant={"outlined"} label={`${result.checked_fields} field(s) checked`} />
        <Chip variant={"outlined"} label={`${result.checked_sections} section(s) checked`} />
        <Chip variant={"outlined"} color={"success"} label={`${result.valid_sections} section(s) valid`} />
        <Chip variant={"outlined"} color={"info"} label={`${result.skipped_sections} section(s) skipped`} />
        <Chip
          variant={"outlined"}
          color={result.invalid_sections ? "error" : "success"}
          label={`${result.invalid_sections} section(s) invalid`}
        />
      </Grid>

      {result.errors.length ? (
        <Grid marginTop={2}>
          <Accordion>
            <AccordionSummary>Errors ({result.errors.length})</AccordionSummary>
            <AccordionDetails sx={{ maxHeight: 300, overflowY: "auto" }}>
              {result.errors.map((it, index) => (
                <Grid key={index}>
                  <Typography color={"green"}>
                    ({index + 1}) [{it.section}] {it.field}
                  </Typography>
                  {it.message}
                  <Typography color={"primary"}>{it.at}</Typography>
                </Grid>
              ))}
            </AccordionDetails>
          </Accordion>
        </Grid>
      ) : null}
    </Grid>
  );
}
