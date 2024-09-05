import { DialogFilter, FileResponse, open } from "@tauri-apps/plugin-dialog";
import { Dispatch, MouseEvent, SetStateAction, useCallback, useState } from "react";

import { Optional } from "@/core/types/general";

export function usePathState({
  title = "Provide path",
  filters = null as Optional<Array<DialogFilter>>,
  isDisabled = false,
  isDirectory = false,
}): [Optional<string>, Dispatch<SetStateAction<Optional<string>>>, (event: MouseEvent<HTMLElement>) => void] {
  const [pathState, setPathState] = useState<Optional<string>>(null);

  const onSelectPath = useCallback(
    async (event: MouseEvent<HTMLElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (isDisabled) {
        return;
      }

      const pathResponse: Optional<FileResponse> = await open({
        title,
        filters: filters ? filters : undefined,
        directory: isDirectory,
      });

      if (pathResponse) {
        setPathState(pathResponse.path);
      }
    },
    [title, isDirectory, isDisabled]
  );

  return [pathState, setPathState, onSelectPath];
}
