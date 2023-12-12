// Not a string, number, boolean, null, undefined, or array
export function isOnlyObject(value: any): value is Record<string, any> {
  return (
    typeof value === "object" &&
    !Array.isArray(value) &&
    value !== null &&
    !(value instanceof Date)
  );
}
