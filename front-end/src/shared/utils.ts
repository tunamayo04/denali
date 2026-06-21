// The API sends dates as date-only strings (YYYY-MM-DD). Passing those to
// `new Date()` parses them as UTC midnight, which then renders as the previous
// day in timezones west of UTC. Build the Date from the date parts instead so
// it lands on local midnight and the displayed day matches the data.
export const parseLocalDate = (value: string | Date): Date => {
  if (value instanceof Date) {
    return value
  }

  const [year, month, day] = value.slice(0, 10).split('-').map(Number)
  return new Date(year ?? 0, ((month ?? 0) - 1) % 12, day)
}

export const formatCurrency = (value: number, markPositive: boolean = false): string => {
  const amount = `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`

  if (value > 0 && markPositive) {
    return `+${amount}`
  }
  else {
    return amount.replace('-', '')
  }
}
