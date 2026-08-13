import { describe, expect, it } from "@jest/globals";

import { formatBytes } from "@/lib/format/memory";

describe("formatBytes", () => {
  it("uses adaptive binary units and compact precision", () => {
    expect(formatBytes(0)).toBe("0 B");
    expect(formatBytes(1536)).toBe("1.5 KB");
    expect(formatBytes(10 * 1024 * 1024)).toBe("10 MB");
    expect(formatBytes(1536 * 1024 * 1024)).toBe("1.5 GB");
  });
});
