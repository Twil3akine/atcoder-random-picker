export type DiffRange = {
    under: number;
    over: number;
}

const rangeKey : string = 'lastDiffRangeUsed';

export function recordLastInput(range : DiffRange): void {
    localStorage.setItem(rangeKey, JSON.stringify(range));
}
export function loadLastInput(): DiffRange | null {
    const data = localStorage.getItem(rangeKey);
    return data ? JSON.parse(data) as DiffRange : null;
}