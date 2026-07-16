import { beforeEach, describe, expect, test } from "bun:test";
import {
  MAX_PICK_HISTORY_ENTRIES,
  MAX_SAVED_PROBLEMS,
  clearPickHistory,
  clearSavedProblems,
  loadHistoryExclusionEnabled,
  loadPickHistory,
  loadSavedProblems,
  recordPickHistory,
  removePickHistoryEntry,
  removeSavedProblem,
  saveProblem,
  saveHistoryExclusionEnabled,
  type PickHistoryEntry,
  type SavedProblem,
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

const savedProblem = (index: number): SavedProblem => ({
  ...problem(index),
  savedAt: new Date(2026, 0, index + 1).toISOString(),
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

describe("saved problems", () => {
  test("saves unique problems in newest-first order", () => {
    saveProblem(problem(1), new Date(2026, 0, 1));
    saveProblem(problem(2), new Date(2026, 0, 2));
    saveProblem(problem(1), new Date(2026, 0, 3));

    const savedProblems = loadSavedProblems();

    expect(savedProblems.map((saved) => saved.id)).toEqual([
      "abc002_a",
      "abc001_a",
    ]);
    expect(savedProblems[1]?.savedAt).toBe(
      new Date(2026, 0, 1).toISOString(),
    );
  });

  test("refuses the two-hundred-fifty-sixth problem without deleting existing items", () => {
    for (let index = 0; index < MAX_SAVED_PROBLEMS; index += 1) {
      saveProblem(problem(index), new Date(2026, 0, index + 1));
    }

    const beforeLimit = loadSavedProblems();
    const afterLimit = saveProblem(problem(MAX_SAVED_PROBLEMS));

    expect(beforeLimit).toHaveLength(MAX_SAVED_PROBLEMS);
    expect(afterLimit).toEqual(beforeLimit);
    expect(afterLimit.some((saved) => saved.id === "abc255_a")).toBe(false);
  });

  test("removes one saved problem or clears the complete list", () => {
    saveProblem(problem(1));
    saveProblem(problem(2));

    expect(removeSavedProblem("abc002_a").map((saved) => saved.id)).toEqual([
      "abc001_a",
    ]);
    expect(clearSavedProblems()).toEqual([]);
    expect(loadSavedProblems()).toEqual([]);
  });

  test("filters invalid items and supports an unknown difficulty", () => {
    const unknownDifficulty = {
      ...savedProblem(1),
      difficulty: null,
    };
    localStorage.setItem(
      "savedProblems",
      JSON.stringify({
        version: 1,
        items: [unknownDifficulty, unknownDifficulty, { id: "invalid" }],
      }),
    );

    expect(loadSavedProblems()).toEqual([unknownDifficulty]);
  });

  test("ignores malformed and unsupported saved problem data", () => {
    localStorage.setItem("savedProblems", "not-json");
    expect(loadSavedProblems()).toEqual([]);

    localStorage.setItem(
      "savedProblems",
      JSON.stringify({ version: 999, items: [savedProblem(1)] }),
    );
    expect(loadSavedProblems()).toEqual([]);
  });
});
