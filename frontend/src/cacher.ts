import type { ClosedRange } from './utils/types';

const rangeKey : string = 'lastDiff';

export function cacheInput(range : ClosedRange): void {
    localStorage.setItem(rangeKey, JSON.stringify(range));
}
export function loadLastInput(): ClosedRange | null {
    const data = localStorage.getItem(rangeKey);
    return data ? JSON.parse(data) as ClosedRange : null;
}