<script lang="ts">
  import type { PickHistoryEntry } from "../utils/cacher";

  type Props = {
    history: PickHistoryEntry[];
    onRemove: (entryId: string) => void;
    onClear: () => void;
  };

  let { history, onRemove, onClear }: Props = $props();

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
        {history.length} / 50
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
                  {entry.contest_id.toUpperCase()} · Diff {entry.difficulty === null
                    ? "不明"
                    : Math.floor(entry.difficulty)} ·
                  {formatPickedAt(entry.pickedAt)}
                </p>
              </div>

              <button
                type="button"
                class="flex min-h-10 shrink-0 cursor-pointer items-center rounded-md px-2 !text-xs text-base-foreground-muted hover:bg-base-container-muted hover:text-base-foreground-default"
                aria-label={`${entry.name}を履歴から削除`}
                onclick={() => onRemove(entry.entryId)}
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
