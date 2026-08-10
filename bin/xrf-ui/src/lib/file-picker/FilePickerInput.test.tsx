import { describe, expect, it, jest } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";

import { renderWithProviders } from "@/fixtures/render";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";

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

  it("selects from the field itself, not only from the button", async () => {
    const onSelect = jest.fn();

    const { getByRole } = renderWithProviders(<FilePickerInput value={"C:\\gamedata"} onSelect={onSelect} />);

    await userEvent.click(getByRole("textbox"));

    expect(onSelect).toHaveBeenCalledTimes(1);
  });

  it("selects from the choose button", async () => {
    const onSelect = jest.fn();

    const { getByLabelText } = renderWithProviders(<FilePickerInput value={"C:\\gamedata"} onSelect={onSelect} />);

    await userEvent.click(getByLabelText("Choose"));

    expect(onSelect).toHaveBeenCalledTimes(1);
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

  it("does not select while disabled", async () => {
    const onSelect = jest.fn();

    const { getByRole } = renderWithProviders(
      <FilePickerInput value={"C:\\gamedata"} isDisabled onSelect={onSelect} />
    );

    await userEvent.click(getByRole("textbox"));

    expect(onSelect).not.toHaveBeenCalled();
  });

  it("describes what the path is for", () => {
    const { getByText } = renderWithProviders(
      <FilePickerInput description={"Directory of LTX files to validate"} onSelect={jest.fn()} />
    );

    expect(getByText("Directory of LTX files to validate")).toBeInTheDocument();
  });
});
