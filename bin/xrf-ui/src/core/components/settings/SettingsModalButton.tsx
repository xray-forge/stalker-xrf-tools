import { default as SettingsIcon } from "@mui/icons-material/Settings";
import { Dialog, IconButton } from "@mui/material";
import { ReactElement, useState } from "react";

import { SettingsForm } from "@/core/components/settings/SettingsForm";

export function SettingsModalButton(): ReactElement {
  const [isModalOpen, setModalOpen] = useState(false);

  return (
    <>
      <IconButton onClick={() => setModalOpen(true)}>
        <SettingsIcon />
      </IconButton>

      <Dialog open={isModalOpen} onClose={() => setModalOpen(false)}>
        <SettingsForm title={"Settings"} />
      </Dialog>
    </>
  );
}
