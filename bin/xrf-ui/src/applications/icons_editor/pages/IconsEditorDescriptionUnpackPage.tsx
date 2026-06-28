import { ReactElement } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";

export function IconsEditorDescriptionUnpackPage(): ReactElement {
  return <PickerForm title={"Provide description file to unpack"} backPath={"/icons_editor"} />;
}
