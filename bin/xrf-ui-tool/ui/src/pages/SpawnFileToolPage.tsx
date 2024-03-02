import { Button, Grid, Input, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { ReactElement, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationHeader } from "@/components/header/ApplicationHeader";

export function SpawnFileToolPage(): ReactElement {
  const [greetMessage, setGreetMessage] = useState("");
  const [name, setName] = useState("");
  const navigate: NavigateFunction = useNavigate();

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      width={"100%"}
      height={"100%"}
    >
      <ApplicationHeader />

      <Typography variant={"h4"}>Spawn file editing page</Typography>

      <form
        className={"row"}
        onSubmit={async (event) => {
          event.preventDefault();
          setGreetMessage(await invoke("greet", { name }));
        }}
      >
        <Input
          id={"greet-input"}
          onChange={(event) => setName(event.currentTarget.value)}
          placeholder={"Enter a name..."}
        />
        <Button type={"submit"} variant={"contained"} sx={{ m: "0 16px" }}>
          Greet
        </Button>
      </form>

      <Typography>{greetMessage}</Typography>

      <Button onClick={() => navigate("/", { replace: true })}>Go back</Button>
    </Grid>
  );
}
