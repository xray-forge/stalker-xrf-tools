import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivesFileContent } from "@/applications/archive_editor/components/editor/ArchivesFileContent";
import { ArchivesMenu } from "@/applications/archive_editor/components/editor/ArchivesMenu";
import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export function ArchivesEditor(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const fileCount: number = Object.keys(archivesService.project.value?.files ?? {}).length;

  return (
    <EditorLayout
      toolbar={
        <EditorToolbar
          title={"Archives editor"}
          subtitle={`${fileCount} files`}
          onBack={archivesService.closeArchivesProject}
        />
      }
      menu={<ArchivesMenu />}
    >
      <ArchivesFileContent />
    </EditorLayout>
  );
}
