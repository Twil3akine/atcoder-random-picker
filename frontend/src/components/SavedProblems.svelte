<script lang="ts">
  import {
    MAX_SAVED_PROBLEMS,
    type SavedProblem,
  } from "../utils/cacher";

  type Props = {
    savedProblems: SavedProblem[];
    onRemove: (problemId: string) => void;
    onClear: () => void;
  };

  let { savedProblems, onRemove, onClear }: Props = $props();

  const dateFormatter = new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });

  const formatSavedAt = (savedAt: string): string =>
    dateFormatter.format(new Date(savedAt));

  const clearList = (): void => {
    if (window.confirm("保存リストをすべて削除しますか？")) {
      onClear();
    }
  };
</script>

<section class="mt-4 w-full" aria-labelledby="saved-problems-heading">
  <details class="rounded-lg border border-base-stroke-default bg-base-container-default">
    <summary
      class="flex min-h-12 cursor-pointer items-center justify-between gap-3 px-4 py-3"
    >
      <span id="saved-problems-heading" class="!text-[1rem] font-medium">
        Saved Problems
      </span>
      <span class="flex items-center gap-2 !text-sm text-base-foreground-muted">
        {savedProblems.length} / {MAX_SAVED_PROBLEMS}
        <span class="details-chevron" aria-hidden="true">›</span>
      </span>
    </summary>

    <div class="border-t border-base-stroke-default px-3 py-3 sm:px-4">
      {#if savedProblems.length === 0}
        <p class="py-5 text-center !text-sm text-base-foreground-muted">
          後で解きたい問題を保存できます。
        </p>
      {:else}
        <div class="mb-3 flex justify-end">
          <button
            type="button"
            class="min-h-11 cursor-pointer rounded-md px-3 py-2 !text-sm text-destructive hover:bg-destructive-on-fill"
            onclick={clearList}
          >
            すべて削除
          </button>
        </div>

        <ol class="max-h-96 space-y-2 overflow-y-auto pr-1">
          {#each savedProblems as problem (problem.id)}
            <li
              class="flex items-start gap-3 rounded-md border border-base-stroke-default p-3"
            >
              <div class="min-w-0 flex-1">
                <a
                  class="block truncate !text-sm font-medium text-primary underline-offset-4 hover:underline"
                  href={`https://atcoder.jp/contests/${problem.contest_id}/tasks/${problem.id}`}
                  target="_blank"
                  rel="noreferrer"
                >
                  {problem.name}
                </a>
                <p class="mt-1 !text-xs leading-relaxed text-base-foreground-muted">
                  {problem.contest_id.toUpperCase()} · Diff {problem.difficulty === null
                    ? "不明"
                    : Math.floor(problem.difficulty)} ·
                  {formatSavedAt(problem.savedAt)}
                </p>
              </div>

              <button
                type="button"
                class="flex min-h-11 shrink-0 cursor-pointer items-center rounded-md px-2 !text-xs text-base-foreground-muted hover:bg-base-container-muted hover:text-base-foreground-default"
                aria-label={`${problem.name}を保存リストから削除`}
                onclick={() => onRemove(problem.id)}
              >
                削除
              </button>
            </li>
          {/each}
        </ol>
      {/if}
    </div>
  </details>
</section>
