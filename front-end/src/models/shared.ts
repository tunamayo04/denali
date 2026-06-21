export type Filter<T> =
  | { op: 'eq'; value: T }
  | { op: 'gt'; value: T }
  | { op: 'ge'; value: T }
  | { op: 'lt'; value: T }
  | { op: 'le'; value: T }
  | { op: 'range_inclusive'; from: T; to: T }
  | { op: 'range_exclusive'; from: T; to: T }

export const Filter = {
  eq: <T>(value: T): Filter<T> => ({ op: 'eq', value }),
  gt: <T>(value: T): Filter<T> => ({ op: 'gt', value }),
  ge: <T>(value: T): Filter<T> => ({ op: 'ge', value }),
  lt: <T>(value: T): Filter<T> => ({ op: 'lt', value }),
  le: <T>(value: T): Filter<T> => ({ op: 'le', value }),
  rangeInclusive: <T>(from: T, to: T): Filter<T> => ({ op: 'range_inclusive', from, to }),
  rangeExclusive: <T>(from: T, to: T): Filter<T> => ({ op: 'range_exclusive', from, to }),
}

export function isFilter(value: unknown): value is Filter<unknown> {
  return typeof value === 'object' && value !== null && 'op' in value
}

export function serializeFilter<T>(filter: Filter<T>, toStr: (v: T) => string = String): string {
  switch (filter.op) {
    case 'eq':
    case 'gt':
    case 'ge':
    case 'lt':
    case 'le':
      return `${filter.op}:${toStr(filter.value)}`
    case 'range_inclusive':
    case 'range_exclusive':
      return `${filter.op}:${toStr(filter.from)},${toStr(filter.to)}`
  }
}

export type IsoDate = string;
