import { Button, Grid, Input } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

export function Application() {
  const [greetMessage, setGreetMessage] = useState("");
  const [name, setName] = useState("");

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      sx={{ width: "100%", height: "100%" }}
    >
      <h1>Welcome to Tauri!</h1>

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

      <p>{greetMessage}</p>
    </Grid>
  );
}
