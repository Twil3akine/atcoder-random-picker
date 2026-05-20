export type CachedInput = {
  min: number;
  max: number;
  selectedContests: string[];
  contest_from: string;
  contest_to: string;
};

const rangeKey: string = 'lastDiff';
const activityKey: string = 'pickActivity';

export type PickActivity = Record<string, number>;

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

const getDateKey = (date: Date): string => {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");

  return `${year}-${month}-${day}`;
};

export const loadPickActivity = (): PickActivity => {
  const data = localStorage.getItem(activityKey);

  if (!data) {
    return {};
  }

  const parsed = JSON.parse(data) as PickActivity;

  return Object.fromEntries(
    Object.entries(parsed).filter(
      ([, count]) => Number.isInteger(count) && count > 0,
    ),
  );
};

export const recordPickActivity = (date = new Date()): PickActivity => {
  const activity = loadPickActivity();
  const dateKey = getDateKey(date);
  const nextActivity = {
    ...activity,
    [dateKey]: (activity[dateKey] ?? 0) + 1,
  };

  localStorage.setItem(activityKey, JSON.stringify(nextActivity));

  return nextActivity;
};
