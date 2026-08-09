import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivesFileContent } from "@/applications/archive_editor/components/editor/ArchivesFileContent";
import { ArchivesMenu } from "@/applications/archive_editor/components/editor/ArchivesMenu";
import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";

export function ArchivesEditor(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const archiveCount: number = archivesService.project.value?.archives.length ?? 0;
  const fileCount: number = Object.keys(archivesService.project.value?.files ?? {}).length;

  useEditorStatus([`${archiveCount} archives`, `${fileCount} files`]);

  return (
    <EditorLayout
      toolbar={<EditorToolbar onBack={archivesService.closeArchivesProject} />}
      menu={<ArchivesMenu />}
    >
      <ArchivesFileContent />
    </EditorLayout>
  );
}
