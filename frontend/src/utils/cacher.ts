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
const historyExclusionKey: string = "pickHistoryExclusionEnabled";
const pickHistoryVersion = 1;

export const MAX_PICK_HISTORY_ENTRIES = 20;

export type PickActivity = Record<string, number>;
export type PickHistoryEntry = Problem & {
  entryId: string;
  pickedAt: string;
};

type PickHistoryStorage = {
  version: typeof pickHistoryVersion;
  entries: PickHistoryEntry[];
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

const savePickHistory = (history: PickHistoryEntry[]): PickHistoryEntry[] => {
  const entries = history.slice(0, MAX_PICK_HISTORY_ENTRIES);
  const storedHistory: PickHistoryStorage = {
    version: pickHistoryVersion,
    entries,
  };

  localStorage.setItem(historyKey, JSON.stringify(storedHistory));

  return entries;
};

export const loadPickHistory = (): PickHistoryEntry[] => {
  const data = localStorage.getItem(historyKey);

  if (!data) {
    return [];
  }

  try {
    const parsed: unknown = JSON.parse(data);
    const isLegacyHistory = Array.isArray(parsed);
    const storedEntries = isLegacyHistory
      ? parsed
      : typeof parsed === "object" &&
          parsed !== null &&
          "version" in parsed &&
          parsed.version === pickHistoryVersion &&
          "entries" in parsed &&
          Array.isArray(parsed.entries)
        ? parsed.entries
        : null;

    if (storedEntries === null) {
      return [];
    }

    const history = storedEntries
      .filter(isPickHistoryEntry)
      .slice(0, MAX_PICK_HISTORY_ENTRIES);

    if (isLegacyHistory) {
      savePickHistory(history);
    }

    return history;
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
  const nextHistory = [entry, ...loadPickHistory()];

  return savePickHistory(nextHistory);
};

export const removePickHistoryEntry = (entryId: string): PickHistoryEntry[] => {
  const nextHistory = loadPickHistory().filter(
    (entry) => entry.entryId !== entryId,
  );

  return savePickHistory(nextHistory);
};

export const clearPickHistory = (): PickHistoryEntry[] => {
  localStorage.removeItem(historyKey);

  return [];
};

export const loadHistoryExclusionEnabled = (): boolean =>
  localStorage.getItem(historyExclusionKey) !== "false";

export const saveHistoryExclusionEnabled = (enabled: boolean): boolean => {
  localStorage.setItem(historyExclusionKey, String(enabled));

  return enabled;
};
