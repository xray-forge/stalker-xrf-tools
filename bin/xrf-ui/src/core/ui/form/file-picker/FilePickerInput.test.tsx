import { describe, expect, it, jest } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";

import { FilePickerInput } from "@/core/ui/form/file-picker/FilePickerInput";
import { renderWithProviders } from "@/fixtures/utils/render";

describe("FilePickerInput", () => {
  it("shows the picked path", () => {
    const { getByDisplayValue } = renderWithProviders(
      <FilePickerInput value={"C:\\gamedata\\config"} onSelect={jest.fn()} />
    );

    expect(getByDisplayValue("C:\\gamedata\\config")).toBeInTheDocument();
  });

  it("says so when nothing is picked yet", () => {
    const { getByPlaceholderText } = renderWithProviders(<FilePickerInput onSelect={jest.fn()} />);

    expect(getByPlaceholderText("Not selected")).toBeInTheDocument();
  });

  it("leaves the field alone so the path can be selected and copied", async () => {
    const onSelect = jest.fn();

    const { getByRole } = renderWithProviders(<FilePickerInput value={"C:\\gamedata"} onSelect={onSelect} />);

    await userEvent.click(getByRole("textbox"));

    // Clicking the text used to reopen the dialog, which made the value impossible to select by hand.
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("selects from the browse button", async () => {
    const onSelect = jest.fn();

    const { getByLabelText } = renderWithProviders(<FilePickerInput value={"C:\\gamedata"} onSelect={onSelect} />);

    await userEvent.click(getByLabelText("Browse"));

    expect(onSelect).toHaveBeenCalledTimes(1);
  });

  it("takes a typed or pasted path when the caller accepts one", async () => {
    const onChange = jest.fn();

    const { getByRole } = renderWithProviders(<FilePickerInput onChange={onChange} onSelect={jest.fn()} />);

    await userEvent.type(getByRole("textbox"), "D:");

    expect(onChange).toHaveBeenCalled();
  });

  it("stays read only when the caller has no way to accept typed input", async () => {
    const { getByRole } = renderWithProviders(<FilePickerInput value={"C:\\gamedata"} onSelect={jest.fn()} />);

    expect(getByRole("textbox")).toHaveAttribute("readonly");
  });

  it("clears without also opening the dialog", async () => {
    const onSelect = jest.fn();
    const onClear = jest.fn();

    const { getByLabelText } = renderWithProviders(
      <FilePickerInput value={"C:\\gamedata"} onSelect={onSelect} onClear={onClear} />
    );

    await userEvent.click(getByLabelText("Clear"));

    // The clear control sits inside the field, whose click opens the picker. Without the guard in the
    // component, clearing would immediately reopen the dialog it just cleared for.
    expect(onClear).toHaveBeenCalledTimes(1);
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("offers nothing to clear when there is no value", () => {
    const { queryByLabelText } = renderWithProviders(<FilePickerInput onSelect={jest.fn()} onClear={jest.fn()} />);

    expect(queryByLabelText("Clear")).not.toBeInTheDocument();
  });

  it("omits the clear control when the caller does not support clearing", () => {
    const { queryByLabelText } = renderWithProviders(<FilePickerInput value={"C:\\gamedata"} onSelect={jest.fn()} />);

    expect(queryByLabelText("Clear")).not.toBeInTheDocument();
  });

  it("does not select while disabled", () => {
    const { getByLabelText, getByRole } = renderWithProviders(
      <FilePickerInput value={"C:\\gamedata"} isDisabled onSelect={jest.fn()} />
    );

    expect(getByLabelText("Browse")).toBeDisabled();
    expect(getByRole("textbox")).toBeDisabled();
  });

  it("describes what the path is for", () => {
    const { getByText } = renderWithProviders(
      <FilePickerInput label={"Configs"} description={"Directory of LTX files to validate"} onSelect={jest.fn()} />
    );

    expect(getByText("Directory of LTX files to validate")).toBeInTheDocument();
  });
});
