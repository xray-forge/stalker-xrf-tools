import { MouseEvent } from "react";

export function stopPropagation(event: MouseEvent<HTMLElement>): void {
  event.stopPropagation();
}

export function preventDefault(event: MouseEvent<HTMLElement>): void {
  event.preventDefault();
}
