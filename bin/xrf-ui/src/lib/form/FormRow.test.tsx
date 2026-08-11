import { describe, expect, it } from "@jest/globals";

import { renderWithProviders } from "@/fixtures/utils/render";
import { FormRow } from "@/lib/form/FormRow";

describe("FormRow", () => {
  it("labels and explains the value", () => {
    const { getByText } = renderWithProviders(
      <FormRow label={"Configs directory"} description={"Directory of LTX files to validate"}>
        <input />
      </FormRow>
    );

    expect(getByText("Configs directory")).toBeInTheDocument();
    expect(getByText("Directory of LTX files to validate")).toBeInTheDocument();
  });

  it("marks a required field so a disabled action is self explanatory", () => {
    const { getByText } = renderWithProviders(
      <FormRow label={"Source"} isRequired>
        <input />
      </FormRow>
    );

    expect(getByText("*")).toBeInTheDocument();
  });

  it("shows a validation message when the value is wrong", () => {
    const { getByText } = renderWithProviders(
      <FormRow label={"Source"} error={"Path does not exist"}>
        <input />
      </FormRow>
    );

    expect(getByText("Path does not exist")).toBeInTheDocument();
  });

  it("omits the message when there is nothing wrong", () => {
    const { queryByText } = renderWithProviders(
      <FormRow label={"Source"}>
        <input />
      </FormRow>
    );

    expect(queryByText("Path does not exist")).not.toBeInTheDocument();
  });
});
