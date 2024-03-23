import { Accordion, AccordionDetails, AccordionSummary, Chip, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { IArchiveUnpackResult } from "@/lib/archive";

interface IConfigsVerifyResultProps {
  result: IArchiveUnpackResult;
}

export function ArchivesUnpackResult({ result }: IConfigsVerifyResultProps): ReactElement {
  return (
    <Grid direction={"column"} padding={2} maxWidth={540} container>
      <Grid justifyContent={"center"} gap={1} container>
        <Chip variant={"outlined"} label={`${result.duration / 1000} sec`} />
        <Chip variant={"outlined"} label={`${(result.unpackedSize / 1024 / 1024).toFixed(1)} MB`} />
        <Chip variant={"outlined"} label={result.destination} />
      </Grid>

      <Grid marginTop={2}>
        <Accordion>
          <AccordionSummary>Archives ({result.archives.length})</AccordionSummary>
          <AccordionDetails sx={{ maxHeight: 112, overflowY: "auto" }}>
            {result.archives.map((it, index) => (
              <Grid key={index}>
                <Typography color={"green"}>
                  ({index + 1}) {it}
                </Typography>
              </Grid>
            ))}
          </AccordionDetails>
        </Accordion>
      </Grid>
    </Grid>
  );
}
