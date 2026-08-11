import { describe, expect, it } from "@jest/globals";
import { RenderResult, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { Injectable } from "@wirestate/core";

import { EquipmentSpriteEditorMenu } from "@/applications/icons-editor/components/equipment-editor/EquipmentSpriteEditorMenu";
import { EquipmentService, IEquipmentPngDescriptor } from "@/applications/icons-editor/store/equipment";
import { Nullable } from "@/core/types/general";
import { renderWithProviders } from "@/fixtures/render";

const SPRITE: IEquipmentPngDescriptor = {
  ltxPath: "C:\\game\\system.ltx",
  descriptors: [],
  path: "C:\\game\\equipment.dds",
  name: "equipment.dds",
  blob: new Blob(),
  image: new Image(),
};

const seed: { repackSourcePath: Nullable<string> } = { repackSourcePath: null };

/** The instance the container built for the current render, so a test can watch what it is asked to do. */
let rendered: Nullable<EquipmentService> = null;

function captureRendered(service: EquipmentService): void {
  rendered = service;
}

/**
 * A service that starts in the state under test.
 *
 * Subclassed rather than bound as a prepared instance because the container only provisions objects it
 * constructed itself, and `useInjection` goes through provisioning.
 */
@Injectable()
class TestEquipmentService extends EquipmentService {
  public constructor() {
    super();

    this.spriteImage = this.spriteImage.asUpdated(SPRITE);
    this.repackSourcePath = seed.repackSourcePath;

    captureRendered(this);
  }
}

function renderMenu(repackSourcePath: Nullable<string>): RenderResult {
  seed.repackSourcePath = repackSourcePath;

  return renderWithProviders(<EquipmentSpriteEditorMenu />, {
    bindings: [{ token: EquipmentService, type: "Instance", value: TestEquipmentService }],
  });
}

describe("EquipmentSpriteEditorMenu", () => {
  it("withholds repacking when there is nothing to rebuild from", () => {
    const { getByRole } = renderMenu(null);

    // Previously the command was offered, then failed after the click, in the console.
    expect(getByRole("button", { name: /Repack sprite/ })).toHaveAttribute("aria-disabled", "true");
    expect(getByRole("button", { name: /Repack sprite/ })).toHaveTextContent("No unpacked icons beside the sprite");
  });

  it("names both paths before overwriting anything", async () => {
    const { getByRole, findByText } = renderMenu("C:\\game\\equipment");

    await userEvent.click(getByRole("button", { name: /Repack sprite/ }));

    // The dialog has to say what is read and what is destroyed, not just that something will happen.
    expect(await findByText("C:\\game\\equipment")).toBeInTheDocument();
    expect(await findByText("C:\\game\\equipment.dds")).toBeInTheDocument();
    expect(getByRole("button", { name: "Repack" })).toBeInTheDocument();
  });

  it("does not repack when the confirmation is dismissed", async () => {
    const { getByRole, queryByRole } = renderMenu("C:\\game\\equipment");

    await userEvent.click(getByRole("button", { name: /Repack sprite/ }));
    await userEvent.click(getByRole("button", { name: "Cancel" }));

    // The dialog leaves on a transition, so its absence has to be waited for rather than asserted
    // straight after the click.
    await waitFor(() => expect(queryByRole("button", { name: "Repack" })).not.toBeInTheDocument());

    // Asserted through the service rather than a spy: `@BoundAction()` makes the method non-writable,
    // and untouched state is the stronger claim anyway. A repack that ran against the mocked backend
    // would have left either a timestamp or an error behind.
    expect((rendered as EquipmentService).repackedAt).toBeNull();
    expect((rendered as EquipmentService).spriteImage.error).toBeNull();
    expect((rendered as EquipmentService).spriteImage.isLoading).toBe(false);
  });
});
