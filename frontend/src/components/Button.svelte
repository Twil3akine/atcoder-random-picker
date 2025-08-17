<!--
@component
## 概要
- buttonタグのように、クリックなどのユーザー操作に対応する汎用的なボタンコンポーネントです
- 色（variant）、サイズ（size）、塗り（tone）を組み合わせて見た目を柔軟に変更できます

## 機能
- buttonタグと同様に、クリックやフォーカスなどのインタラクションに対応します
- 見た目を変更するためのいくつかのスタイル用Propsが追加されています(詳細はPropsセクションを参照)

## Props
- variant: ボタンの意味を指定すると、それに合わせたスタイルになります
- tone: 塗りのスタイルを指定します
- isSquare: trueにすると正方形のボタンになります
- size: サイズを指定します
- disabled: 指定するとグレーアウトされ、クリック不可になります

## Usage
```svelte
<Button variant="primary" tone="solid" size="medium">ボタン</Button>
```
-->

<script module lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  export const buttonVariants = cva(
    'inline-flex items-center justify-center gap-2 rounded-md text-sm leading-tight transition-colors cursor-pointer outline-primary focus-visible:outline-primary focus-visible:outline-[0.125rem] focus-visible:outline-offset-[0.125rem]',
    {
      variants: {
        /** ボタンの使用用途 */
        variant: {
          primary: [],
          secondary: [],
          success: [],
          danger: [],
        },
        /** ボタンのサイズ */
        size: {
          tiny: ['w-[13.5rem] py-2 min-h-7'],
          small: ['px-3 py-2 min-h-9'],
          medium: ['px-4 py-2.5 min-h-10'],
          large: ['px-8 py-3 min-h-11'],
        },
        /** ボタンの塗りの値 */
        tone: {
          solid: [],
          ghost: [],
        },
        /** ボタンを正方形にするか */
        isSquare: {
          true: ['aspect-square'],
          false: [],
        },
        /** 操作できるかどうか */
        disabled: {
          true: ['opacity-50 pointer-events-none'],
          false: [],
        },
      },
      compoundVariants: [
        {
          variant: 'primary',
          tone: 'solid',
          class:
            'border border-primary bg-primary text-primary-on-fill hover:bg-primary/90 hover:border-primary/10 active:bg-primary/80 active:border-primary/80',
        },
        {
          variant: 'primary',
          tone: 'ghost',
          class:
            'border border-transparent text-primary hover:bg-base-container-accent/90 hover:border-base-container-accent/90 active:bg-base-container-accent/80 active:border-base-container-accent/80',
        },
        {
          variant: 'secondary',
          tone: 'solid',
          class:
            'border border-base-stroke-default text-base-foreground-default hover:bg-base-container-accent/90 active:bg-base-container-accent/80',
        },
        {
          variant: 'secondary',
          tone: 'ghost',
          class:
            'border border-transparent text-base-foreground-default hover:bg-base-container-accent/90 hover:border-base-container-accent/90 active:bg-base-container-accent/80 active:border-base-container-accent/80',
        },
        {
          variant: 'success',
          tone: 'solid',
          class:
            'border border-success bg-success text-success-on-fill hover:bg-success/90 hover:border-success/10 active:bg-success/80 active:border-success/80',
        },
        {
          variant: 'success',
          tone: 'ghost',
          class:
            'border border-transparent text-success hover:bg-base-container-accent/90 hover:border-base-container-accent/90 active:bg-base-container-accent/80 active:border-base-container-accent/80',
        },
        {
          variant: 'danger',
          tone: 'solid',
          class:
            'border border-destructive bg-destructive text-destructive-on-fill hover:bg-destructive/90 hover:border-destructive/10 active:bg-destructive/80 active:border-destructive/80',
        },
        {
          variant: 'danger',
          tone: 'ghost',
          class: `
            border border-destructive text-destructive
            hover:bg-destructive hover:text-white
            active:bg-destructive/80 active:text-white
          `,
        },
        {
          isSquare: true,
          size: 'small',
          class: '!p-2.25',
        },
        {
          isSquare: true,
          size: 'medium',
          class: '!p-2.75',
        },
        {
          isSquare: true,
          size: 'large',
          class: '!p-3.25',
        },
      ],
      defaultVariants: {
        variant: 'primary',
        tone: 'solid',
        size: 'medium',
      },
    },
  );

  export type ButtonVariants = VariantProps<typeof buttonVariants>;

  export interface ButtonProps extends ButtonVariants, HTMLButtonAttributes {
    /** クラス */
    class?: string;
    children?: Snippet<[]>;
  }
</script>

<script lang="ts">
  let { tone, isSquare, variant, size, class: className = '', children, ...buttonAttributes }: ButtonProps = $props();

  let buttonVariantClass = $derived(buttonVariants({
    tone: tone,
    isSquare: isSquare,
    variant: variant,
    disabled: buttonAttributes.disabled,
    size: size,
    class: className,
  }));
</script>

<!-- svelte-ignore slot_element_deprecated -->
<button class={buttonVariantClass} {...buttonAttributes}>
  <slot />
</button>