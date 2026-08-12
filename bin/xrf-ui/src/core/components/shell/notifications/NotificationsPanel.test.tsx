import { describe, expect, it } from "@jest/globals";
import { act, RenderResult } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { Binding, Container } from "@wirestate/core";

import { EApplicationToolId } from "@/core/components/shell/application-tools";
import { NotificationsPanel } from "@/core/components/shell/notifications/NotificationsPanel";
import { NotificationsService } from "@/core/store/notifications";
import { renderWithProviders } from "@/fixtures/utils/render";
import { INotificationPayload } from "@/lib/notifications";

interface IPanelRender {
  render: RenderResult;
  service: NotificationsService;
}

/** Seeded before rendering, which is the ordering the panel actually meets - it opens onto a log. */
function renderPanel(seed: Array<INotificationPayload> = []): IPanelRender {
  const service: NotificationsService = new Container({ bindings: [NotificationsService] }).get(NotificationsService);

  seed.forEach((payload: INotificationPayload) => service.push(payload));

  return {
    render: renderWithProviders(<NotificationsPanel />, {
      bindings: [{ token: NotificationsService, value: service } as Binding],
    }),
    service,
  };
}

describe("NotificationsPanel", () => {
  it("says nothing has happened rather than showing an empty box", () => {
    const { render } = renderPanel();

    expect(render.getByText(/Nothing has been reported yet/)).toBeInTheDocument();
    expect(render.getByRole("button", { name: "Clear all" })).toBeDisabled();
  });

  it("names the tool a record came from the way the rail does", () => {
    const { render } = renderPanel([
      { severity: "success", source: EApplicationToolId.ARCHIVES, title: "Extracted textures" },
    ]);

    expect(render.getByText("Extracted textures")).toBeInTheDocument();
    expect(render.getByText("Archives")).toBeInTheDocument();
  });

  it("shows the newest record first", () => {
    const { render } = renderPanel([
      { severity: "info", source: EApplicationToolId.ARCHIVES, title: "Older" },
      { severity: "info", source: EApplicationToolId.ARCHIVES, title: "Newer" },
    ]);

    const titles: Array<string> = render
      .getAllByText(/Older|Newer/)
      .map((it: HTMLElement) => it.textContent as string);

    expect(titles).toEqual(["Newer", "Older"]);
  });

  it("reads what it was opened onto", () => {
    const { service } = renderPanel([{ severity: "error", source: EApplicationToolId.ARCHIVES, title: "Failed" }]);

    expect(service.unreadCount).toBe(0);
  });

  it("reads what arrives while it is open", () => {
    const { service } = renderPanel();

    act(() => service.push({ severity: "error", source: EApplicationToolId.ARCHIVES, title: "Failed" }));

    // Left unread, the badge counts records the user is looking at and nothing can dismiss it.
    expect(service.unreadCount).toBe(0);
  });

  it("keeps details out of the way until they are asked for", async () => {
    const { render } = renderPanel([
      {
        details: "C:\\out\\system.ltx",
        severity: "error",
        source: EApplicationToolId.ARCHIVES,
        title: "Could not extract",
      },
    ]);

    expect(render.queryByText("C:\\out\\system.ltx")).not.toBeInTheDocument();

    await userEvent.click(render.getByLabelText("Show details"));

    expect(render.getByText("C:\\out\\system.ltx")).toBeInTheDocument();
  });

  it("offers no expander for a record with nothing more to say", () => {
    const { render } = renderPanel([
      { severity: "info", source: EApplicationToolId.ARCHIVES, title: "Nothing to expand" },
    ]);

    expect(render.queryByLabelText("Show details")).not.toBeInTheDocument();
  });

  it("clears the log on request", async () => {
    const { render, service } = renderPanel([
      { severity: "info", source: EApplicationToolId.ARCHIVES, title: "Something" },
    ]);

    await userEvent.click(render.getByRole("button", { name: "Clear all" }));

    expect(service.notifications).toHaveLength(0);
    expect(render.getByText(/Nothing has been reported yet/)).toBeInTheDocument();
  });
});
