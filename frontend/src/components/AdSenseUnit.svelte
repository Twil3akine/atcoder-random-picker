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
    <p class="mb-2 text-center !text-xs text-base-foreground-muted">広告</p>

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
        class="flex min-h-[5.625rem] w-full items-center justify-center rounded-md border border-dashed border-base-stroke-default bg-base-container-accent px-4 text-center !text-xs text-base-foreground-muted"
      >
        AdSense preview（環境変数を設定すると広告が表示されます）
      </div>
    {/if}
  </aside>
{/if}
