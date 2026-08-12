import { useCallback } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

export type TChunkTabChange = (event: unknown, value: string) => void;

/**
 * Which sub-table of a chunk is showing, taken from the route rather than from local state.
 */
export function useChunkTab(basePath: string, tabs: Array<string>, fallback: string): [string, TChunkTabChange] {
  const navigate: NavigateFunction = useNavigate();

  const { pathname } = useLocation();

  const segment: string = pathname.split("/").filter(Boolean).pop() ?? "";
  const activeTab: string = tabs.includes(segment) ? segment : fallback;

  const onChangeTab: TChunkTabChange = useCallback(
    (_: unknown, value: string) => navigate(`${basePath}/${value}`, { replace: true }),
    [basePath, navigate]
  );

  return [activeTab, onChangeTab];
}
