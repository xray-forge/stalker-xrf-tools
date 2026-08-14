import { describe, expect, it } from "@jest/globals";

import { FormRow } from "@/core/ui/form/FormRow";
import { renderWithProviders } from "@/fixtures/utils/render";

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

  it("marks the optional field rather than every required one", () => {
    const required = renderWithProviders(
      <FormRow label={"Source"} isRequired>
        <input />
      </FormRow>
    );

    expect(required.queryByText("Optional")).not.toBeInTheDocument();

    required.unmount();

    const optional = renderWithProviders(
      <FormRow label={"Source"} isRequired={false}>
        <input />
      </FormRow>
    );

    expect(optional.getByText("Optional")).toBeInTheDocument();
  });

  it("ties the label to the control, so the field is not announced as unlabelled", () => {
    const { getByLabelText } = renderWithProviders(
      <FormRow label={"Configs directory"} controlId={"configs-directory"}>
        <input id={"configs-directory"} />
      </FormRow>
    );

    expect(getByLabelText("Configs directory")).toBeInTheDocument();
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
