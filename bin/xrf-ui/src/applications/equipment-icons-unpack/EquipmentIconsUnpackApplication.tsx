import { ReactElement } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";

export function EquipmentIconsUnpackApplication(): ReactElement {
  return <PickerForm title={"Provide equipment paths to unpack"} backPath={"/"} />;
}
