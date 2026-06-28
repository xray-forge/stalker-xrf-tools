import { ReactElement } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";

export function IconsEditorEquipmentUnpackPage(): ReactElement {
  return <PickerForm title={"Provide equipment paths to unpack"} backPath={"/icons_editor"} />;
}
