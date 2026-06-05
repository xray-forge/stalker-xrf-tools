import { Accordion, AccordionDetails, AccordionSummary, Box, Chip, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ILtxProjectVerifyResult } from "@/lib/ltx";

interface IConfigsVerifyResultProps {
  result: ILtxProjectVerifyResult;
}

export function ConfigsVerifyResult({ result }: IConfigsVerifyResultProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", padding: 2, maxWidth: 540 }}>
      <Grid container sx={{ justifyContent: "center", gap: 1 }}>
        <Chip variant={"outlined"} label={`${result.duration / 1000} sec`} />
        <Chip variant={"outlined"} label={`${result.totalFiles} files total`} />
        <Chip variant={"outlined"} label={`${result.totalSections} sections total`} />
      </Grid>

      <Grid container sx={{ justifyContent: "center", gap: 1, marginTop: 1, padding: `0 ${16}px` }}>
        <Chip variant={"outlined"} label={`${result.checkedFields} field(s) checked`} />
        <Chip variant={"outlined"} label={`${result.checkedSections} section(s) checked`} />
        <Chip variant={"outlined"} color={"success"} label={`${result.validSections} section(s) valid`} />
        <Chip variant={"outlined"} color={"info"} label={`${result.skippedSections} section(s) skipped`} />
        <Chip
          variant={"outlined"}
          color={result.invalidSections ? "error" : "success"}
          label={`${result.invalidSections} section(s) invalid`}
        />
      </Grid>

      {result.errors.length ? (
        <Box sx={{ marginTop: 2 }}>
          <Accordion>
            <AccordionSummary>Errors ({result.errors.length})</AccordionSummary>
            <AccordionDetails sx={{ maxHeight: 300, overflowY: "auto" }}>
              {result.errors.map((it, index) => (
                <Box key={index}>
                  <Typography color={"green"}>
                    ({index + 1}) [{it.section}] {it.field}
                  </Typography>
                  {it.message}
                  <Typography color={"primary"}>{it.at}</Typography>
                </Box>
              ))}
            </AccordionDetails>
          </Accordion>
        </Box>
      ) : null}
    </Box>
  );
}
