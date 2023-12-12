export function prettyBytes(num: number) {
  const neg = num < 0;
  if (neg) num = -num;
  if (num < 1) return (neg ? "-" : "") + num + " B";
  const units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  const exponent = Math.min(
    Math.floor(Math.log(num) / Math.log(1000)),
    units.length - 1
  );
  num = +(num / Math.pow(1000, exponent)).toFixed(2);
  const unit = units[exponent];
  return (neg ? "-" : "") + num + " " + unit;
}

export function prettyNumber(num: number) {
  return num.toLocaleString();
}

export function prettyHeading(heading: string) {
  return heading
    .replace(/([A-Z])/g, " $1")
    .replace(/^./, (str) => str.toUpperCase())
    .replace(/_/g, " ");
}
