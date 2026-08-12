import { Box, Theme } from "@mui/material";
import { SystemStyleObject } from "@mui/system";
import { ReactElement, useMemo } from "react";

import { SyntaxContent } from "@/core/components/code/SyntaxContent";
import { ESyntaxLanguage } from "@/lib/syntax";

export interface ICodeViewProps {
  content: string;
  language: ESyntaxLanguage;
  /**
   * Number of the first rendered line.
   *
   * Not always 1: an excerpt lifted out of a file is far more useful when its gutter still says where
   * in that file it came from.
   */
  firstLine?: number;
  label?: string;
  sx?: SystemStyleObject<Theme>;
}

const CODE_LINE_HEIGHT: number = 1.6;

/**
 * Source text with a line gutter, coloured by its grammar.
 */
export function CodeView({ content, language, firstLine = 1, label, sx }: ICodeViewProps): ReactElement {
  const lineNumbers: string = useMemo(() => {
    const count: number = Math.max(1, content.split("\n").length);

    return Array.from({ length: count }, (_, index: number) => firstLine + index).join("\n");
  }, [content, firstLine]);

  return (
    <Box aria-label={label} sx={[{ display: "flex", minWidth: 0, overflow: "auto" }, sx ?? {}]}>
      <Box
        aria-hidden={true}
        component={"pre"}
        className={"monospace"}
        sx={{
          flexShrink: 0,
          margin: 0,
          padding: 1.5,
          borderRight: 1,
          borderColor: "divider",
          color: "text.secondary",
          lineHeight: CODE_LINE_HEIGHT,
          textAlign: "right",
          userSelect: "none",
        }}
      >
        {lineNumbers}
      </Box>

      <Box
        component={"pre"}
        className={"monospace"}
        sx={{
          minWidth: "max-content",
          minHeight: "100%",
          margin: 0,
          padding: 1.5,
          color: "text.primary",
          lineHeight: CODE_LINE_HEIGHT,
          tabSize: 2,
          whiteSpace: "pre",
        }}
      >
        <SyntaxContent content={content} language={language} />
      </Box>
    </Box>
  );
}
