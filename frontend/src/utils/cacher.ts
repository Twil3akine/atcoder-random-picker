import type { Problem } from "./types";

export type CachedInput = {
  min: number | "";
  max: number | "";
  selectedContests: string[];
  contest_from: string;
  contest_to: string;
};

const rangeKey: string = 'lastDiff';
const activityKey: string = 'pickActivity';
const historyKey: string = "pickHistory";
const maxHistoryEntries = 50;

export type PickActivity = Record<string, number>;
export type PickHistoryEntry = Problem & {
  entryId: string;
  pickedAt: string;
};

export const cacheInput = (input: CachedInput): void => {
  localStorage.setItem(rangeKey, JSON.stringify(input));
};

export const loadLastInput = (): CachedInput | null => {
  const data = localStorage.getItem(rangeKey);

  if (!data) {
    return null;
  }

  const parsed = JSON.parse(data) as Partial<CachedInput>;

  if (
    (typeof parsed.min !== "number" && parsed.min !== "") ||
    (typeof parsed.max !== "number" && parsed.max !== "")
  ) {
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

const isPickHistoryEntry = (value: unknown): value is PickHistoryEntry => {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const entry = value as Partial<PickHistoryEntry>;

  return (
    typeof entry.entryId === "string" &&
    typeof entry.pickedAt === "string" &&
    !Number.isNaN(Date.parse(entry.pickedAt)) &&
    typeof entry.id === "string" &&
    typeof entry.contest_id === "string" &&
    typeof entry.name === "string" &&
    (entry.difficulty === null ||
      (typeof entry.difficulty === "number" &&
        Number.isFinite(entry.difficulty)))
  );
};

export const loadPickHistory = (): PickHistoryEntry[] => {
  const data = localStorage.getItem(historyKey);

  if (!data) {
    return [];
  }

  try {
    const parsed: unknown = JSON.parse(data);

    if (!Array.isArray(parsed)) {
      return [];
    }

    return parsed.filter(isPickHistoryEntry).slice(0, maxHistoryEntries);
  } catch {
    return [];
  }
};

export const recordPickHistory = (
  problem: Problem,
  date = new Date(),
): PickHistoryEntry[] => {
  const entry: PickHistoryEntry = {
    ...problem,
    entryId: `${date.getTime()}-${Math.random().toString(36).slice(2)}`,
    pickedAt: date.toISOString(),
  };
  const nextHistory = [entry, ...loadPickHistory()].slice(
    0,
    maxHistoryEntries,
  );

  localStorage.setItem(historyKey, JSON.stringify(nextHistory));

  return nextHistory;
};

export const removePickHistoryEntry = (entryId: string): PickHistoryEntry[] => {
  const nextHistory = loadPickHistory().filter(
    (entry) => entry.entryId !== entryId,
  );

  localStorage.setItem(historyKey, JSON.stringify(nextHistory));

  return nextHistory;
};

export const clearPickHistory = (): PickHistoryEntry[] => {
  localStorage.removeItem(historyKey);

  return [];
};
