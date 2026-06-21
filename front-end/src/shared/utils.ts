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

export const formatDateDifference = (date: Date) => {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  const hours = Math.floor(diff / (1000 * 60 * 60))

  if (days <= 0) {
    return `${hours} hour${hours > 1 ? 's' : ''} ago`
  }
  return `${days} day${days > 1 ? 's' : ''} ago`
}
