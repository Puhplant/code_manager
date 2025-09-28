<script lang="ts">
  import { createDialog, melt } from '@melt-ui/svelte';
  import { fade, fly } from 'svelte/transition';
  import { X } from 'lucide-svelte';
  import cn from 'clsx';
  import { twMerge } from 'tailwind-merge';
  import { writable } from 'svelte/store';

  interface ModalProps {
    title?: string;
    description?: string;
    size?: 'sm' | 'md' | 'lg' | 'xl';
    class?: string;
    children?: any;
    open: boolean;
    onOpenChange: (open: boolean) => void;
  }

  let { 
    title = '',
    description = '',
    size = 'md',
    class: className = '',
    children,
    open = $bindable(false),
    onOpenChange,
  } : ModalProps = $props();

  let isOpen = writable(false);

  const {
    elements: {
      overlay,
      content,
      title: titleElement,
      description: descriptionElement,
      close,
      portalled,
    },
  } = createDialog({
    forceVisible: true,
    onOpenChange: ({ next }) => {
      open = next;
      onOpenChange(next);
      return next;
    },
    open: isOpen,
  });

  $effect(() => {
    isOpen.set(open);
  });

  const sizeStyles: Record<string, string> = {
    sm: 'w-[20vw] h-[40vh]',
    md: 'w-[40vw] h-[60vh]',
    lg: 'w-[60vw] h-[80vh]',
    xl: 'w-[70vw] h-[90vh]'
  };

  const contentClasses = twMerge(
    cn(
      'fixed left-1/2 top-1/2 z-50 max-h-[85vh] -translate-x-1/2 -translate-y-1/2 rounded-lg bg-card p-6 shadow-lg border border-border flex flex-col',
      sizeStyles[size],
      className
    )
  );
</script>

{#if $isOpen}
  <div use:melt={$portalled}>
    <div
      use:melt={$overlay}
      class="fixed inset-0 z-50 bg-black/50"
      transition:fade={{ duration: 150 }}
    ></div>
    <div
      class={contentClasses}
      transition:fly={{
        duration: 150,
        y: -200,
      }}
      use:melt={$content}
    >
      {#if title}
        <h2 use:melt={$titleElement} class="m-0 text-lg font-semibold text-card-foreground">
          {title}
        </h2>
      {/if}
      {#if description}
        <p use:melt={$descriptionElement} class="mb-5 mt-2 text-sm text-muted-foreground">
          {description}
        </p>
      {/if}

      <div class="text-card-foreground flex-1">
        {@render children?.()}
      </div>

      <button
        use:melt={$close}
        aria-label="close"
        class="cursor-pointer absolute right-4 top-4 inline-flex h-6 w-6 appearance-none items-center justify-center rounded-full p-1 text-muted-foreground hover:bg-accent/50 hover:text-accent-foreground focus:outline-none"
      >
        <X class="size-4" />
      </button>
    </div>
  </div>
{/if}
