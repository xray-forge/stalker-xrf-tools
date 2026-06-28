import { ReactElement } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";

export function IconsEditorDescriptionOpenPage(): ReactElement {
  return <PickerForm title={"Provide description file to open"} backPath={"/icons_editor"} />;
}
