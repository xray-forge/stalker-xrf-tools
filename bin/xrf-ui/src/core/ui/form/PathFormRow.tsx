import { ReactElement } from "react";

import { FilePickerInput } from "@/core/ui/form/file-picker/FilePickerInput";
import { FormRow } from "@/core/ui/form/FormRow";
import { IPathField } from "@/core/ui/form/use-path-field";

export interface IPathFormRowProps {
  label: string;
  description?: string;
  isRequired?: boolean;
  isDisabled?: boolean;
  placeholder?: string;
  field: IPathField;
}

/**
 * A labelled, validated, remembered path - the whole row in one element.
 */
export function PathFormRow({
  label,
  description,
  isRequired = true,
  isDisabled,
  placeholder,
  field,
}: IPathFormRowProps): ReactElement {
  return (
    <FormRow label={label} description={description} isRequired={isRequired} error={field.error}>
      <FilePickerInput
        placeholder={placeholder}
        value={field.value}
        isDisabled={isDisabled}
        isInvalid={Boolean(field.error)}
        onSelect={field.select}
        onClear={field.clear}
      />
    </FormRow>
  );
}
