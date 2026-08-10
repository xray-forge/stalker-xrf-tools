import { describe, expect, it } from "@jest/globals";

import { APPLICATION_TOOLS, IApplicationTool } from "@/core/components/shell/application-tools";
import { APPLICATION_ROUTES, IApplicationRoute } from "@/core/router/lazy-routers";

describe("APPLICATION_ROUTES", () => {
  it("answers every tool the rail offers", () => {
    const routed: Array<string> = APPLICATION_ROUTES.map((route: IApplicationRoute) => route.path).sort();
    const offered: Array<string> = APPLICATION_TOOLS.map((tool: IApplicationTool) => tool.path).sort();

    // The two lists repeat the same paths, so nothing but this stops a rail icon from navigating to a
    // route that does not exist, or an editor from being reachable only by typing its url.
    expect(routed).toEqual(offered);
  });

  it("gives every route something to render", () => {
    for (const route of APPLICATION_ROUTES) {
      expect(route.Component).toBeTruthy();
    }
  });

  it("declares absolute paths, which is what the suspense key is resolved against", () => {
    for (const route of APPLICATION_ROUTES) {
      expect(route.path.startsWith("/")).toBe(true);
    }
  });
});
