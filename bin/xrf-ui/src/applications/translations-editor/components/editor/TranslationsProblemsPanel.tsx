import { Box, Chip, List, ListItem, ListItemText, Typography } from "@mui/material";
import { ReactElement } from "react";

import { TranslationFinding } from "@/core/bindings/xrf-app-translations";
import { EmptyState } from "@/core/ui/layout/EmptyState";

/** Trims the project root off a subject, which is otherwise most of the line. */
function toLeaf(subject: string): string {
  const separator: number = subject.lastIndexOf("/");

  return separator === -1 ? subject : subject.slice(separator + 1);
}

export interface ITranslationsProblemsPanelProps {
  findings: ReadonlyArray<TranslationFinding>;
}

/**
 * What the reader found but did not refuse to open for.
 */
export function TranslationsProblemsPanel({ findings }: ITranslationsProblemsPanelProps): ReactElement {
  return findings.length ? (
    <Box sx={{ width: "100%", height: "100%", overflowY: "auto" }}>
      <List dense disablePadding>
        {findings.map((finding: TranslationFinding, index: number) => (
          <ListItem key={`${finding.rule}-${finding.subject}-${index}`} alignItems={"flex-start"} divider>
            <ListItemText
              primary={
                <Box sx={{ display: "flex", alignItems: "center", gap: 0.75, minWidth: 0 }}>
                  <Chip size={"small"} variant={"outlined"} label={finding.rule.replace("translations.", "")} />

                  {finding.subject ? (
                    <Typography variant={"caption"} noWrap sx={{ color: "text.secondary" }} title={finding.subject}>
                      {toLeaf(finding.subject)}
                    </Typography>
                  ) : null}
                </Box>
              }
              secondary={finding.message}
            />
          </ListItem>
        ))}
      </List>
    </Box>
  ) : (
    <EmptyState title={"No problems found"} description={"Every file in this project read cleanly."} />
  );
}
