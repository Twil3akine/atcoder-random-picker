<script lang="ts">
  import { onMount } from "svelte";

  const clientId = import.meta.env.VITE_ADSENSE_CLIENT?.trim();
  const isConfigured = Boolean(clientId);
  const isPreviewEnabled =
    import.meta.env.DEV && import.meta.env.VITE_ADSENSE_PREVIEW === "true";
  const scriptId = "google-adsense-script";

  const railSides = ["left", "right"] as const;
  let isMobilePreviewOpen = $state(true);
  let openRailPreviews = $state<Record<(typeof railSides)[number], boolean>>({
    left: true,
    right: true,
  });

  onMount(() => {
    if (!isConfigured || document.getElementById(scriptId)) {
      return;
    }

    const script = document.createElement("script");
    script.id = scriptId;
    script.async = true;
    script.crossOrigin = "anonymous";
    script.src = `https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=${encodeURIComponent(clientId!)}`;
    document.head.appendChild(script);
  });
</script>

{#if isPreviewEnabled}
  <!-- Tablet and mobile: bottom anchor preview -->
  {#if isMobilePreviewOpen}
    <div
      class="pointer-events-none fixed inset-x-0 bottom-0 z-50 px-3 pb-[max(0.75rem,env(safe-area-inset-bottom))] xl:hidden"
    >
      <aside
        class="pointer-events-auto relative mx-auto flex min-h-[5.5rem] w-full max-w-[22rem] items-center gap-3 overflow-visible rounded-xl border border-primary/20 bg-gradient-to-br from-primary/10 via-base-container-default to-success/10 px-4 py-3 shadow-lg"
        aria-label="下部固定広告プレビュー"
      >
        <button
          type="button"
          class="absolute -top-3 right-2 flex size-7 cursor-pointer items-center justify-center rounded-full border border-base-stroke-default bg-base-container-default !text-sm leading-none text-base-foreground-muted shadow-sm"
          aria-label="下部広告プレビューを閉じる"
          onclick={() => (isMobilePreviewOpen = false)}
        >
          ×
        </button>

        <div class="min-w-0 flex-1">
          <span class="!text-[0.625rem] font-semibold tracking-wider text-primary">
            SAMPLE AD
          </span>
          <p class="truncate !text-sm font-semibold text-base-foreground-default">
            毎日1問、アルゴリズム習慣
          </p>
          <p class="truncate !text-[0.6875rem] text-base-foreground-muted">
            下部アンカー広告のプレビューです
          </p>
        </div>

        <span
          class="inline-flex min-h-9 shrink-0 items-center rounded-md bg-primary px-3 py-2 !text-[0.6875rem] font-medium text-primary-on-fill"
        >
          詳しく見る
        </span>
      </aside>
    </div>
  {/if}

  <!-- Wide desktop: left and right side rail previews -->
  {#if openRailPreviews.left || openRailPreviews.right}
    <div class="hidden xl:block" aria-label="左右レール広告プレビュー">
      {#each railSides as side}
        {#if openRailPreviews[side]}
          <aside
            class={`fixed top-1/2 z-30 flex h-[28rem] w-40 -translate-y-1/2 flex-col overflow-hidden rounded-xl border border-primary/20 bg-gradient-to-b from-primary/10 via-base-container-default to-success/10 p-4 shadow-md ${
              side === "left" ? "left-6" : "right-6"
            }`}
            aria-label={`${side === "left" ? "左" : "右"}レール広告プレビュー`}
          >
            <button
              type="button"
              class="absolute right-2 top-2 flex size-7 cursor-pointer items-center justify-center rounded-full border border-base-stroke-default bg-base-container-default/90 !text-sm leading-none text-base-foreground-muted"
              aria-label={`${side === "left" ? "左" : "右"}広告プレビューを閉じる`}
              onclick={() => (openRailPreviews[side] = false)}
            >
              ×
            </button>

            <span
              class="mt-10 self-start rounded-full bg-primary/10 px-2 py-1 !text-[0.625rem] font-semibold tracking-wider text-primary"
            >
              SAMPLE AD
            </span>
            <p class="mt-5 !text-lg font-semibold leading-snug text-base-foreground-default">
              毎日1問、<br />続ける力に。
            </p>
            <p class="mt-3 !text-xs leading-relaxed text-base-foreground-muted">
              PCの左右に固定されるサイドレール広告のプレビューです。
            </p>

            <div class="mt-auto">
              <div class="mb-5 h-24 rounded-full bg-primary/10"></div>
              <span
                class="flex min-h-10 items-center justify-center rounded-md bg-primary px-3 py-2 !text-xs font-medium text-primary-on-fill"
              >
                詳しく見る
              </span>
            </div>
          </aside>
        {/if}
      {/each}
    </div>
  {/if}
{/if}
