import { ReactElement } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";

export function IconsEditorDescriptionPackPage(): ReactElement {
  return <PickerForm title={"Provide description file to pack"} backPath={"/icons_editor"} />;
}
