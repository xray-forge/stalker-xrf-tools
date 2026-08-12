import { ReactElement, useMemo } from "react";

import { CodeView } from "@/core/components/code/CodeView";
import { ProjectReadResult } from "@/lib/bindings/xray-archive";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { ESyntaxLanguage, getSyntaxLanguage } from "@/lib/syntax";

interface IArchiveTextPreviewProps extends BaseComponentProps {
  file: ProjectReadResult;
}

export function ArchiveTextPreview({ file }: IArchiveTextPreviewProps): ReactElement {
  const language: ESyntaxLanguage = useMemo(() => getSyntaxLanguage(file.name), [file.name]);

  return (
    <CodeView
      label={`Contents of ${file.name}`}
      content={file.content}
      language={language}
      sx={{ flexGrow: 1, minHeight: 0, backgroundColor: "background.paper" }}
    />
  );
}
