import type { ClosedRange } from './types';

const rangeKey : string = 'lastDiff';

export const cacheInput = (range : ClosedRange): void => {
  localStorage.setItem(rangeKey, JSON.stringify(range));
}
export const loadLastInput = (): ClosedRange | null => {
  const data = localStorage.getItem(rangeKey);
  return data ? JSON.parse(data) as ClosedRange : null;
}