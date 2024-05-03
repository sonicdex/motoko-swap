import bigDecimal from "js-big-decimal";

export function getPrettyDecimals(amount: bigint | number, tokenDecimals: number, roundingDecimals = 3) {
  const decimals = getZerosFromDecimals(tokenDecimals);
  const value = new bigDecimal(Number(Number(amount) / decimals).toFixed(tokenDecimals)).getPrettyValue(3, ",");
  return value;
}

export function getZerosFromDecimals(decimals: number) {
  let zeros = "1";
  for (let i = 0; i < decimals; i++) {
    zeros += "0";
  }
  return Number(zeros);
}
