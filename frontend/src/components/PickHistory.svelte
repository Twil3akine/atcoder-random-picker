<script lang="ts">
  import {
    MAX_PICK_HISTORY_ENTRIES,
    type PickHistoryEntry,
  } from "../utils/cacher";

  type Props = {
    history: PickHistoryEntry[];
    savedProblemIds: Set<string>;
    savedListIsFull: boolean;
    onSave: (problem: PickHistoryEntry) => void;
    onRemove: (entryId: string) => void;
    onClear: () => void;
  };

  let {
    history,
    savedProblemIds,
    savedListIsFull,
    onSave,
    onRemove,
    onClear,
  }: Props = $props();

  const dateFormatter = new Intl.DateTimeFormat("ja-JP", {
    month: "numeric",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });

  const formatPickedAt = (pickedAt: string): string =>
    dateFormatter.format(new Date(pickedAt));

  const clearHistory = (): void => {
    if (window.confirm("選出履歴をすべて削除しますか？")) {
      onClear();
    }
  };

  const saveLabel = (problemId: string): string => {
    if (savedProblemIds.has(problemId)) {
      return "保存済み";
    }

    return savedListIsFull ? "上限到達" : "保存";
  };
</script>

<section class="mt-8 w-full" aria-labelledby="pick-history-heading">
  <details class="rounded-lg border border-base-stroke-default bg-base-container-default">
    <summary
      class="flex min-h-12 cursor-pointer items-center justify-between gap-3 px-4 py-3"
    >
      <span id="pick-history-heading" class="!text-[1rem] font-medium">
        Pick History
      </span>
      <span class="flex items-center gap-2 !text-sm text-base-foreground-muted">
        {history.length} / {MAX_PICK_HISTORY_ENTRIES}
        <span class="details-chevron" aria-hidden="true">›</span>
      </span>
    </summary>

    <div class="border-t border-base-stroke-default px-3 py-3 sm:px-4">
      {#if history.length === 0}
        <p class="py-5 text-center !text-sm text-base-foreground-muted">
          選出した問題がここに表示されます。
        </p>
      {:else}
        <div class="mb-3 flex justify-end">
          <button
            type="button"
            class="min-h-10 cursor-pointer rounded-md px-3 py-2 !text-sm text-destructive hover:bg-destructive-on-fill"
            onclick={clearHistory}
          >
            すべて削除
          </button>
        </div>

        <ol class="max-h-80 space-y-2 overflow-y-auto pr-1">
          {#each history as entry (entry.entryId)}
            <li
              class="flex items-start gap-3 rounded-md border border-base-stroke-default p-3"
            >
              <div class="min-w-0 flex-1">
                <a
                  class="block truncate !text-sm font-medium text-primary underline-offset-4 hover:underline"
                  href={`https://atcoder.jp/contests/${entry.contest_id}/tasks/${entry.id}`}
                  target="_blank"
                  rel="noreferrer"
                >
                  {entry.name}
                </a>
                <p class="mt-1 !text-xs text-base-foreground-muted">
                  {entry.contest_id.toUpperCase()} · {formatPickedAt(entry.pickedAt)}
                </p>
              </div>

              <div class="flex shrink-0 flex-col gap-1 sm:flex-row">
                <button
                  type="button"
                  class="flex min-h-11 cursor-pointer items-center rounded-md px-2 !text-xs text-primary hover:bg-primary/10 disabled:cursor-default disabled:opacity-50"
                  aria-label={`${entry.name}を保存`}
                  disabled={savedProblemIds.has(entry.id) || savedListIsFull}
                  onclick={() => onSave(entry)}
                >
                  {saveLabel(entry.id)}
                </button>
                <button
                  type="button"
                  class="flex min-h-11 cursor-pointer items-center rounded-md px-2 !text-xs text-base-foreground-muted hover:bg-base-container-muted hover:text-base-foreground-default"
                  aria-label={`${entry.name}を履歴から削除`}
                  onclick={() => onRemove(entry.entryId)}
                >
                  削除
                </button>
              </div>
            </li>
          {/each}
        </ol>
      {/if}
    </div>
  </details>
</section>
