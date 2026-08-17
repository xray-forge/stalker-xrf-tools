import { describe, expect, it } from "@jest/globals";

import { ABSENT_VALUE, formatNumber } from "@/lib/format/number";

describe("formatNumber", () => {
  it("renders a fixed number of decimals", () => {
    expect(formatNumber(1.23456, 3)).toBe("1.235");
    expect(formatNumber(-0.5, 2)).toBe("-0.50");
    expect(formatNumber(0, 0)).toBe("0");
  });

  it("marks an absent value rather than showing a zero", () => {
    // A non-finite rust float serialises to null, so this is the ipc contract rather than a defensive check.
    expect(formatNumber(null, 3)).toBe(ABSENT_VALUE);
  });

  it("marks a non-finite value, which no fixed rendering can express", () => {
    expect(formatNumber(Number.NaN, 3)).toBe(ABSENT_VALUE);
    expect(formatNumber(Number.POSITIVE_INFINITY, 3)).toBe(ABSENT_VALUE);
    expect(formatNumber(Number.NEGATIVE_INFINITY, 3)).toBe(ABSENT_VALUE);
  });

  it("accepts a caller's own placeholder", () => {
    expect(formatNumber(null, 3, "unknown")).toBe("unknown");
    expect(formatNumber(1, 1, "unknown")).toBe("1.0");
  });
});
