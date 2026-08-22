import { Alert, Stack } from "@mui/material";
import { ReactElement } from "react";

import { PackerFolderList } from "@/applications/archives-packer/components/controls/PackerFolderList";
import { PackerStringList } from "@/applications/archives-packer/components/controls/PackerStringList";
import { isWholeDirectory } from "@/applications/archives-packer/lib/pack-config";
import { ArchivePackConfig } from "@/core/bindings/types/xrf-pack";
import { FormRow } from "@/core/ui/form/FormRow";

interface IPackerSelectionSectionProps {
  config: ArchivePackConfig;
  isDisabled?: boolean;
  onChange: (patch: Partial<ArchivePackConfig>) => void;
}

/**
 * What goes into the archive: the sections an xrCompress configuration carries.
 */
export function PackerSelectionSection({ config, isDisabled, onChange }: IPackerSelectionSectionProps): ReactElement {
  return (
    <Stack spacing={2}>
      {isWholeDirectory(config) ? (
        <Alert severity={"info"}>
          Nothing is selected, so the whole source directory is packed. Add a folder or a file to narrow it.
        </Alert>
      ) : null}

      <FormRow
        label={"Included folders"}
        description={"Folders to pack, relative to the source. Recursive folders take their subfolders too"}
      >
        <PackerFolderList
          folders={config.includeFolders}
          isDisabled={isDisabled}
          addLabel={"Add folder"}
          emptyLabel={"No folders listed."}
          recursiveLabel={"Recursive"}
          onChange={(includeFolders) => onChange({ includeFolders })}
        />
      </FormRow>

      <FormRow label={"Included files"} description={"Individual files to pack, named relative to the source"}>
        <PackerStringList
          values={config.includeFiles}
          isDisabled={isDisabled}
          addLabel={"Add file"}
          emptyLabel={"No files listed."}
          placeholder={"shaders.xr"}
          onChange={(includeFiles) => onChange({ includeFiles })}
        />
      </FormRow>

      <FormRow
        label={"Excluded folders"}
        description={"Folders to leave out. A prefix match drops everything beneath it, otherwise only the exact path"}
      >
        <PackerFolderList
          folders={config.excludeFolders}
          isDisabled={isDisabled}
          addLabel={"Add exclusion"}
          emptyLabel={"No exclusions."}
          recursiveLabel={"Prefix"}
          onChange={(excludeFolders) => onChange({ excludeFolders })}
        />
      </FormRow>

      <FormRow label={"Excluded extensions"} description={"Patterns matched against a file extension, such as *.txt"}>
        <PackerStringList
          values={config.excludeExtensions}
          isDisabled={isDisabled}
          addLabel={"Add pattern"}
          emptyLabel={"No patterns."}
          placeholder={"*.txt"}
          onChange={(excludeExtensions) => onChange({ excludeExtensions })}
        />
      </FormRow>
    </Stack>
  );
}
