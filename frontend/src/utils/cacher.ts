export type CachedInput = {
  min: number;
  max: number;
  selectedContests: string[];
  contest_from: string;
  contest_to: string;
};

const rangeKey: string = 'lastDiff';

export const cacheInput = (input: CachedInput): void => {
  localStorage.setItem(rangeKey, JSON.stringify(input));
};

export const loadLastInput = (): CachedInput | null => {
  const data = localStorage.getItem(rangeKey);

  if (!data) {
    return null;
  }

  const parsed = JSON.parse(data) as Partial<CachedInput>;

  if (typeof parsed.min !== "number" || typeof parsed.max !== "number") {
    return null;
  }

  return {
    min: parsed.min,
    max: parsed.max,
    selectedContests: parsed.selectedContests ?? [],
    contest_from: parsed.contest_from ?? "",
    contest_to: parsed.contest_to ?? "",
  };
};
