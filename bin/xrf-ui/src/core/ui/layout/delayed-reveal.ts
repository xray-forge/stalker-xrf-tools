import { Theme } from "@mui/material";
import { keyframes, SystemStyleObject } from "@mui/system";

/** How long an operation may run before a loader for it is worth drawing at all. */
export const REVEAL_DELAY_MS: number = 500;

const REVEAL_KEYFRAMES = keyframes`
  from { visibility: hidden; }
  to { visibility: visible; }
`;

/**
 * Holds a loader hidden until the operation behind it has run long enough to be worth reporting.
 *
 * Most commands answer in tens of milliseconds, where a bar that appears and vanishes reads as a
 * glitch rather than as progress. Done as an animation delay rather than a timer so a fast operation
 * costs no extra render, and so the delay restarts with each mount.
 */
export const DELAYED_REVEAL_SX: SystemStyleObject<Theme> = {
  visibility: "hidden",
  animationName: `${REVEAL_KEYFRAMES}`,
  animationDuration: "0s",
  animationDelay: `${REVEAL_DELAY_MS}ms`,
  animationFillMode: "forwards",
};
