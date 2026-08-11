import { Box } from "@mui/material";
import { ReactElement, useMemo } from "react";

import { ARCHIVE_EDITOR_MONOSPACE_FONT } from "@/applications/archive-editor/components/editor/archive-editor.styles";
import { SyntaxContent } from "@/core/components/code/SyntaxContent";
import { IArchiveFileReadResult } from "@/lib/archive";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { ESyntaxLanguage, getSyntaxLanguage } from "@/lib/syntax";

interface IArchiveTextPreviewProps extends BaseComponentProps {
  file: IArchiveFileReadResult;
}

export function ArchiveTextPreview({ file }: IArchiveTextPreviewProps): ReactElement {
  const lineNumbers: string = useMemo(() => {
    const count: number = Math.max(1, file.content.split("\n").length);

    return Array.from({ length: count }, (_, index: number) => index + 1).join("\n");
  }, [file.content]);

  const language: ESyntaxLanguage = useMemo(() => getSyntaxLanguage(file.name), [file.name]);

  return (
    <Box
      aria-label={`Contents of ${file.name}`}
      sx={{
        display: "flex",
        flexGrow: 1,
        minWidth: 0,
        minHeight: 0,
        overflow: "auto",
        backgroundColor: "background.default",
      }}
    >
      <Box
        aria-hidden={true}
        component={"pre"}
        sx={{
          left: 0,
          zIndex: 1,
          flexShrink: 0,
          margin: 0,
          padding: 1.5,
          borderRight: 1,
          borderColor: "divider",
          color: "text.secondary",
          fontFamily: ARCHIVE_EDITOR_MONOSPACE_FONT,
          fontSize: "0.75rem",
          lineHeight: 1.6,
          textAlign: "right",
          userSelect: "none",
        }}
      >
        {lineNumbers}
      </Box>

      <Box
        component={"pre"}
        sx={{
          minWidth: "max-content",
          minHeight: "100%",
          margin: 0,
          padding: 1.5,
          color: "text.primary",
          fontFamily: ARCHIVE_EDITOR_MONOSPACE_FONT,
          fontSize: "0.75rem",
          lineHeight: 1.6,
          tabSize: 2,
          whiteSpace: "pre",
        }}
      >
        <SyntaxContent content={file.content} language={language} />
      </Box>
    </Box>
  );
}
