<!--
@component
## 概要
- detailsタグ・summaryタグのように、クリックなどのユーザー操作で開閉に対応する汎用的なアコーディオンコンポーネントです

## 機能
- detailsタグ・summaryタグと同様に、クリックやフォーカスなどのインタラクションに対応します

## Props
- title: 見出しの文言を設定できます 

## Usage
```svelte
<Accordion title='項目'>ここにコンテンツ内容が入ります</Accordion>
```
-->

<script module lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import type { Snippet } from 'svelte';

  export let accordionVariants = cva(
    'w-full border-b-1 border-base-stroke-default'
  );

  export let accordionTitleVariants = cva(
    'relative px-2 py-4 list-none text-base-foreground-default text-base transition cursor-pointer outline-primary focus-visible:outline-primary focus-visible:outline-[0.125rem] focus-visible:outline-offset-[0.125rem] focus-visible:!rounded-sm hover:bg-base-container-accent'
  );

  export let accordionDescriptionVariants = cva(
    'px-2 pb-4 text-base-foreground-default'
  );

  export type AccordionVariants = VariantProps<typeof accordionVariants>;

  export interface AccordionProps extends AccordionVariants {
    /** タイトル */
    title: string;
    /** クラス */
    class?: string;
    children: Snippet<[]>;
  }
</script>

<script lang="ts">
  import { ChevronDown } from '@lucide/svelte';

  let { title, class: className = '', children } = $props();

  const animTiming = { 
    duration: 250,
    easing: 'ease-out'
  };

  let detailsElement;
  let summaryElement;
  let descriptionContainer;

  let isOpen = $state(false);

  let accordionVariantClass = $derived(accordionVariants({
    class: className,
  }));

  function closeAnimKeyframes() {
    return [
      {
        height: descriptionContainer.offsetHeight + "px", // height: "auto"だとうまく計算されないため要素の高さを指定する
        opacity: 1,
      },
      {
        height: 0,
        opacity: 0,
      },
    ]
  }

  function openAnimKeyframes() {
    return [
      {
        height: 0,
        opacity: 0,
      },
      {
        height: descriptionContainer.offsetHeight + "px",
        opacity: 1,
      },
    ]
  }

  function summaryToggleAnim(e) {
    e.preventDefault();
    if (detailsElement.open) {
      const close_anim = descriptionContainer.animate(
        closeAnimKeyframes(),
        animTiming,
      );

      isOpen = false;

      close_anim.onfinish = () => {
        // アニメーションの完了後にopen属性を取り除く
        detailsElement.removeAttribute("open");
      };
    }
    else {
      detailsElement.setAttribute("open", "true");

      isOpen = true;

      descriptionContainer.animate(
        openAnimKeyframes(),
        animTiming,
      );
    }
  }

</script>

<details class={accordionVariantClass} bind:this={detailsElement}>
  <summary class={accordionTitleVariants()} bind:this={summaryElement} onclick={summaryToggleAnim}>
    {title}
    <ChevronDown class='absolute top-4 right-2 transition duration-300 {isOpen ? "rotate-180" : ""}' size="1rem" />
  </summary>
  <div class='overflow-hidden' bind:this={descriptionContainer}>
    <div class={accordionDescriptionVariants()}>
      {@render children()}
    </div>
  </div>
</details>