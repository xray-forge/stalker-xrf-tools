import { Accordion, AccordionDetails, AccordionSummary, Box, Chip, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { IArchiveUnpackResult } from "@/lib/archive";

interface IConfigsVerifyResultProps {
  result: IArchiveUnpackResult;
}

export function ArchivesUnpackResult({ result }: IConfigsVerifyResultProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", padding: 2, maxWidth: 540 }}>
      <Grid container sx={{ justifyContent: "center", gap: 1 }}>
        <Chip variant={"outlined"} label={`${result.duration / 1000} sec`} />
        <Chip variant={"outlined"} label={`${(result.unpackedSize / 1024 / 1024).toFixed(1)} MB`} />
        <Chip variant={"outlined"} label={result.destination} />
      </Grid>

      <Box sx={{ marginTop: 2 }}>
        <Accordion>
          <AccordionSummary>Archives ({result.archives.length})</AccordionSummary>
          <AccordionDetails sx={{ maxHeight: 112, overflowY: "auto" }}>
            {result.archives.map((it, index) => (
              <Box key={index}>
                <Typography color={"green"}>
                  ({index + 1}) {it}
                </Typography>
              </Box>
            ))}
          </AccordionDetails>
        </Accordion>
      </Box>
    </Box>
  );
}
