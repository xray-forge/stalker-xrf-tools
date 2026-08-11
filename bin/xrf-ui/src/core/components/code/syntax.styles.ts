import { Theme } from "@mui/material";

import { ESyntaxToken } from "@/lib/syntax";

/**
 * Token colours for a dark surface.
 *
 * Hues are picked for the roles rather than for a language: the name of a thing (`SECTION`, `KEY`) sits
 * in the blues, its value (`STRING`, `NUMBER`) in the warms, and the parts that are not the data at all
 * (`COMMENT`, `OPERATOR`) recede.
 */
const DARK_SYNTAX_COLORS: Record<ESyntaxToken, string> = {
  [ESyntaxToken.PLAIN]: "inherit",
  [ESyntaxToken.COMMENT]: "#6a9955",
  [ESyntaxToken.STRING]: "#ce9178",
  [ESyntaxToken.NUMBER]: "#b5cea8",
  [ESyntaxToken.KEYWORD]: "#569cd6",
  [ESyntaxToken.TYPE]: "#4ec9b0",
  [ESyntaxToken.DIRECTIVE]: "#c586c0",
  [ESyntaxToken.SECTION]: "#dcdcaa",
  [ESyntaxToken.KEY]: "#9cdcfe",
  [ESyntaxToken.OPERATOR]: "#909090",
};

/** The same roles darkened to hold contrast against a light surface. */
const LIGHT_SYNTAX_COLORS: Record<ESyntaxToken, string> = {
  [ESyntaxToken.PLAIN]: "inherit",
  [ESyntaxToken.COMMENT]: "#3f7d20",
  [ESyntaxToken.STRING]: "#a31515",
  [ESyntaxToken.NUMBER]: "#116644",
  [ESyntaxToken.KEYWORD]: "#0000c0",
  [ESyntaxToken.TYPE]: "#1d7a86",
  [ESyntaxToken.DIRECTIVE]: "#8f0e9e",
  [ESyntaxToken.SECTION]: "#7a5c00",
  [ESyntaxToken.KEY]: "#04517a",
  [ESyntaxToken.OPERATOR]: "#666666",
};

/**
 * Token colours matching the surface the preview is drawn on.
 *
 * @param theme - Active theme, read for its light or dark mode.
 * @returns A colour per token, where `PLAIN` inherits so most of a file needs no colour applied at all.
 */
export function getSyntaxColors(theme: Theme): Record<ESyntaxToken, string> {
  return theme.palette.mode === "light" ? LIGHT_SYNTAX_COLORS : DARK_SYNTAX_COLORS;
}
