<script lang="ts">
  import { onMount } from "svelte";

  const clientId = import.meta.env.VITE_ADSENSE_CLIENT?.trim();
  const slotId = import.meta.env.VITE_ADSENSE_SLOT?.trim();
  const isConfigured = Boolean(clientId && slotId);
  const scriptId = "google-adsense-script";

  onMount(() => {
    if (!isConfigured) {
      return;
    }

    if (!document.getElementById(scriptId)) {
      const script = document.createElement("script");
      script.id = scriptId;
      script.async = true;
      script.crossOrigin = "anonymous";
      script.src = `https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=${encodeURIComponent(clientId!)}`;
      document.head.appendChild(script);
    }

    try {
      window.adsbygoogle = window.adsbygoogle ?? [];
      window.adsbygoogle.push({});
    } catch (error) {
      console.warn("AdSense ad could not be initialized.", error);
    }
  });
</script>

{#if isConfigured || import.meta.env.DEV}
  <aside class="mt-8 w-full border-t border-base-stroke-default pt-4" aria-label="広告">
    <p class="mb-2 text-center !text-xs text-base-foreground-muted">
      {isConfigured ? "広告" : "広告プレビュー"}
    </p>

    {#if isConfigured}
      <ins
        class="adsbygoogle block min-h-[5.625rem] w-full overflow-hidden"
        data-ad-client={clientId}
        data-ad-slot={slotId}
        data-ad-format="auto"
        data-full-width-responsive="true"
      ></ins>
    {:else}
      <div
        class="relative flex min-h-[10rem] w-full overflow-hidden rounded-xl border border-primary/20 bg-gradient-to-br from-primary/10 via-base-container-default to-success/10 p-5 shadow-sm sm:min-h-[7.5rem] sm:items-center sm:px-6"
      >
        <div
          class="pointer-events-none absolute -right-10 -top-12 size-36 rounded-full bg-primary/10"
        ></div>
        <div
          class="pointer-events-none absolute -bottom-12 right-16 size-28 rounded-full bg-success/10"
        ></div>

        <div
          class="relative z-10 flex w-full flex-col gap-4 sm:flex-row sm:items-center sm:justify-between"
        >
          <div class="min-w-0">
            <span
              class="mb-2 inline-flex rounded-full bg-primary/10 px-2.5 py-1 !text-[0.6875rem] font-semibold tracking-wider text-primary"
            >
              SAMPLE AD
            </span>
            <p class="!text-lg font-semibold leading-snug text-base-foreground-default">
              アルゴリズムを、毎日の習慣に。
            </p>
            <p class="mt-1 !text-xs leading-relaxed text-base-foreground-muted">
              これは広告掲載位置を確認するための仮バナーです。
            </p>
          </div>

          <span
            class="inline-flex min-h-10 shrink-0 items-center justify-center self-start rounded-md bg-primary px-4 py-2 !text-xs font-medium text-primary-on-fill sm:self-center"
          >
            詳しく見る →
          </span>
        </div>
      </div>
    {/if}
  </aside>
{/if}
