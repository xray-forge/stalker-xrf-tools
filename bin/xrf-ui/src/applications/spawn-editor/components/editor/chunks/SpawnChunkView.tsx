import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { EmptyState } from "@/core/components/layout/EmptyState";
import { Nullable } from "@/core/types/general";
import { Loadable } from "@/lib/loadable";
import { useMountEffect } from "@/lib/react";

export interface ISpawnChunkViewProps<T> {
  chunk: Loadable<Nullable<T>>;
  /** Asked for on mount. The service ignores a chunk it already holds. */
  onLoad: () => Promise<void>;
  render: (value: T) => ReactNode;
}

/**
 * The frame every spawn chunk renders into.
 */
export function SpawnChunkView<T>({ chunk, onLoad, render }: ISpawnChunkViewProps<T>): ReactElement {
  useMountEffect(() => void onLoad());

  if (chunk.isLoading) {
    return <DelayedProgress />;
  }

  if (chunk.error) {
    return <EmptyState title={"Could not read this chunk"} description={String(chunk.error)} />;
  }

  if (!chunk.value) {
    return <EmptyState title={"Nothing to show"} description={"Open a spawn file to read its chunks."} />;
  }

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        width: "100%",
        height: "100%",
        minHeight: 0,
        padding: 2,
        flexWrap: "nowrap",
      }}
    >
      {render(chunk.value)}
    </Box>
  );
}
