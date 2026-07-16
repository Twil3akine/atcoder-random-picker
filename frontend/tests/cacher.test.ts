import { beforeEach, describe, expect, test } from "bun:test";
import {
  MAX_PICK_HISTORY_ENTRIES,
  clearPickHistory,
  loadHistoryExclusionEnabled,
  loadPickHistory,
  recordPickHistory,
  removePickHistoryEntry,
  saveHistoryExclusionEnabled,
  type PickHistoryEntry,
} from "../src/utils/cacher";
import type { Problem } from "../src/utils/types";

class MemoryStorage implements Storage {
  private values = new Map<string, string>();

  get length(): number {
    return this.values.size;
  }

  clear(): void {
    this.values.clear();
  }

  getItem(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.values.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.values.delete(key);
  }

  setItem(key: string, value: string): void {
    this.values.set(key, value);
  }
}

const problem = (index: number): Problem => ({
  id: `abc${String(index).padStart(3, "0")}_a`,
  contest_id: `abc${String(index).padStart(3, "0")}`,
  name: `Problem ${index}`,
  difficulty: 100 + index,
});

const historyEntry = (index: number): PickHistoryEntry => ({
  ...problem(index),
  entryId: `entry-${index}`,
  pickedAt: new Date(2026, 0, index + 1).toISOString(),
});

beforeEach(() => {
  Object.defineProperty(globalThis, "localStorage", {
    configurable: true,
    value: new MemoryStorage(),
  });
});

describe("pick history", () => {
  test("keeps only the latest twenty entries", () => {
    for (let index = 0; index <= MAX_PICK_HISTORY_ENTRIES; index += 1) {
      recordPickHistory(problem(index), new Date(2026, 0, index + 1));
    }

    const history = loadPickHistory();

    expect(history).toHaveLength(MAX_PICK_HISTORY_ENTRIES);
    expect(history[0]?.id).toBe("abc020_a");
    expect(history.at(-1)?.id).toBe("abc001_a");
  });

  test("migrates the legacy array format and trims it to twenty entries", () => {
    const legacyHistory = Array.from({ length: 25 }, (_, index) =>
      historyEntry(index),
    );
    localStorage.setItem("pickHistory", JSON.stringify(legacyHistory));

    const history = loadPickHistory();
    const stored = JSON.parse(localStorage.getItem("pickHistory") ?? "null");

    expect(history).toHaveLength(MAX_PICK_HISTORY_ENTRIES);
    expect(stored.version).toBe(1);
    expect(stored.entries).toHaveLength(MAX_PICK_HISTORY_ENTRIES);
  });

  test("ignores malformed and unsupported history data", () => {
    localStorage.setItem("pickHistory", "not-json");
    expect(loadPickHistory()).toEqual([]);

    localStorage.setItem(
      "pickHistory",
      JSON.stringify({ version: 999, entries: [historyEntry(1)] }),
    );
    expect(loadPickHistory()).toEqual([]);
  });

  test("removes one entry or clears the complete history", () => {
    const first = recordPickHistory(problem(1), new Date(2026, 0, 1));
    const second = recordPickHistory(problem(2), new Date(2026, 0, 2));

    expect(removePickHistoryEntry(second[0]!.entryId)).toEqual([first[0]]);
    expect(clearPickHistory()).toEqual([]);
    expect(loadPickHistory()).toEqual([]);
  });
});

describe("history exclusion setting", () => {
  test("is enabled by default and persists both values", () => {
    expect(loadHistoryExclusionEnabled()).toBe(true);

    saveHistoryExclusionEnabled(false);
    expect(loadHistoryExclusionEnabled()).toBe(false);

    saveHistoryExclusionEnabled(true);
    expect(loadHistoryExclusionEnabled()).toBe(true);
  });
});
