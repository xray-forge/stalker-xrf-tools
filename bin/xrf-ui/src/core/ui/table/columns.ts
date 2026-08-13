import { GridColDef, GridValidRowModel, GridValueGetter } from "@mui/x-data-grid";

/** Applied by the theme to render a cell in monospace. */
const MONOSPACE_CLASS: string = "monospace";

/** Vector components are read for magnitude and sign, not for their tail digits. */
const VECTOR_PRECISION: number = 2;

interface IVectorLike {
  x: number;
  y: number;
  z: number;
}

function isVectorLike(value: unknown): value is IVectorLike {
  return typeof value === "object" && value !== null && "x" in value && "y" in value && "z" in value;
}

/**
 * A plain numeric or string column.
 *
 * Exists so a table declares every column through the same helper, rather than most of them through an
 * object literal and the interesting ones through a helper.
 */
export function textColumn(field: string, headerName: string, width?: number): GridColDef {
  return width === undefined ? { field, headerName } : { field, headerName, width };
}

/**
 * An identifier, path or section: compared by eye, so rendered in monospace.
 */
export function identifierColumn(field: string, headerName: string, width?: number): GridColDef {
  return {
    ...textColumn(field, headerName, width),
    cellClassName: MONOSPACE_CLASS,
  };
}

/**
 * A position or direction, as `x, y, z`.
 *
 * These used to be `JSON.stringify`d into the cell, which spent the width on punctuation and field
 * names that are identical in every row.
 */
export function vectorColumn(field: string, headerName: string, width: number = 170): GridColDef {
  return {
    field,
    headerName,
    width,
    cellClassName: MONOSPACE_CLASS,
    sortable: false,
    valueGetter: ((value: unknown) =>
      isVectorLike(value)
        ? `${value.x.toFixed(VECTOR_PRECISION)}, ${value.y.toFixed(VECTOR_PRECISION)}, ${value.z.toFixed(
            VECTOR_PRECISION
          )}`
        : null) as GridValueGetter<GridValidRowModel>,
  };
}

/**
 * A bit field, as hex.
 *
 * Decimal flags cannot be read as flags; hex at least groups the bits the engine sets together.
 */
export function flagsColumn(field: string, headerName: string, width: number = 110): GridColDef {
  return {
    field,
    headerName,
    width,
    cellClassName: MONOSPACE_CLASS,
    valueGetter: ((value: unknown) =>
      typeof value === "number" ? `0x${value.toString(16).toUpperCase()}` : null) as GridValueGetter<GridValidRowModel>,
  };
}

/**
 * A fixed length numeric tuple, such as a vertex type, as a compact list.
 */
export function tupleColumn(field: string, headerName: string, width: number = 150): GridColDef {
  return {
    field,
    headerName,
    width,
    cellClassName: MONOSPACE_CLASS,
    sortable: false,
    valueGetter: ((value: unknown) =>
      Array.isArray(value) ? value.join(", ") : null) as GridValueGetter<GridValidRowModel>,
  };
}

/**
 * A distance or other float, at fixed precision.
 */
export function decimalColumn(field: string, headerName: string, width: number = 110): GridColDef {
  return {
    field,
    headerName,
    width,
    cellClassName: MONOSPACE_CLASS,
    valueGetter: ((value: unknown) =>
      typeof value === "number" ? value.toFixed(VECTOR_PRECISION) : null) as GridValueGetter<GridValidRowModel>,
  };
}
