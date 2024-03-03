import { default as FolderIcon } from "@mui/icons-material/Folder";
import { default as SettingsIcon } from "@mui/icons-material/Settings";
import {
  Dialog,
  FormControl,
  Grid,
  IconButton,
  InputAdornment,
  InputLabel,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { ReactElement, useState } from "react";

export function SettingsModalButton(): ReactElement {
  const [isModalOpen, setModalOpen] = useState(false);

  return (
    <>
      <IconButton onClick={() => setModalOpen(true)}>
        <SettingsIcon />
      </IconButton>

      <Dialog open={isModalOpen} onClose={() => setModalOpen(false)}>
        <Grid padding={2} direction={"column"} container>
          <Typography variant={"h6"} component={"h2"}>
            Settings
          </Typography>

          <FormControl sx={{ m: "8px 0" }} size={"small"} variant={"outlined"}>
            <InputLabel size={"small"}>Project</InputLabel>
            <OutlinedInput
              size={"small"}
              type={"text"}
              endAdornment={
                <InputAdornment position={"end"}>
                  <IconButton edge={"end"}>
                    <FolderIcon />
                  </IconButton>
                </InputAdornment>
              }
              label={"Project"}
              readOnly
            />
          </FormControl>

          <FormControl sx={{ m: "8px 0" }} size={"small"} variant={"outlined"}>
            <InputLabel size={"small"}>Configs</InputLabel>
            <OutlinedInput
              size={"small"}
              type={"text"}
              endAdornment={
                <InputAdornment position={"end"}>
                  <IconButton edge={"end"}>
                    <FolderIcon />
                  </IconButton>
                </InputAdornment>
              }
              label={"Configs"}
              readOnly
            />
          </FormControl>
        </Grid>
      </Dialog>
    </>
  );
}
