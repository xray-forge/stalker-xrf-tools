import { Box, Divider, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export type TCommandResultTone = "success" | "warning" | "error" | "info";

export interface ICommandResultStat {
  label: string;
  value: ReactNode;
  tone?: TCommandResultTone;
}

export interface ICommandResultProps {
  /** The one sentence answer to "how did it go". */
  headline: string;
  tone: TCommandResultTone;
  stats: Array<ICommandResultStat>;
  children?: ReactNode;
}

const TONE_COLORS: Record<TCommandResultTone, string> = {
  success: "success.main",
  warning: "warning.main",
  error: "error.main",
  info: "text.primary",
};

/**
 * Shared presentation for whatever a long running command produced.
 *
 * Tone comes from the palette, never a literal colour. The previous components printed their error
 * headings in hardcoded `green`.
 */
export function CommandResult({ headline, tone, stats, children }: ICommandResultProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", minHeight: 0 }}>
      <Typography variant={"subtitle2"} sx={{ color: TONE_COLORS[tone] }}>
        {headline}
      </Typography>

      <Box sx={{ display: "flex", flexWrap: "wrap", columnGap: 3, rowGap: 0.5, marginTop: 1 }}>
        {stats.map((stat: ICommandResultStat) => (
          <Box key={stat.label} sx={{ display: "flex", alignItems: "baseline", gap: 0.75 }}>
            <Typography variant={"body2"} sx={{ color: stat.tone ? TONE_COLORS[stat.tone] : "text.primary" }}>
              {stat.value}
            </Typography>
            <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
              {stat.label}
            </Typography>
          </Box>
        ))}
      </Box>

      {children ? (
        <>
          <Divider sx={{ marginY: 2 }} />
          {children}
        </>
      ) : null}
    </Box>
  );
}
