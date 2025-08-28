<script lang="ts">
  import cn from 'clsx';
  import { createTabs, melt } from '@melt-ui/svelte';
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';

  interface Tab {
    id: string;
    label: string;
  }

  interface TabsProps {
    tabs: Tab[];
    children?: any;
    defaultValue?: string;
    onValueChange: (value: string) => string;
    class?: string;
    value: string;
  }

  const { tabs, children, defaultValue, value, onValueChange, class: className = '' }: TabsProps = $props();



  const {
    elements: { root, list, trigger },
  } = createTabs({
    defaultValue: defaultValue,
    onValueChange: ({ next }) => onValueChange(next),
  });

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });
</script>

<div
  use:melt={$root}
  class={cn(
    'flex flex-col overflow-hidden rounded-lg shadow-lg border border-input bg-transparent data-[orientation=vertical]:flex-row w-full',
    className,
  )}
>
  <div
    use:melt={$list}
    class="flex shrink-0 overflow-x-auto bg-transparent border-b border-input data-[orientation=vertical]:flex-col data-[orientation=vertical]:border-r data-[orientation=vertical]:border-b-0"
    aria-label="Manage your account"
  >
    {#each tabs as triggerItem}
      <button 
        use:melt={$trigger(triggerItem.id)} 
        class="cursor-pointer relative px-4 py-3 text-sm font-medium text-foreground hover:text-foreground transition-colors border-none border-transparent hover:border-input data-[state=active]:text-foreground data-[state=active]:bg-muted/10 data-[state=active]:border-primary"
      >
        {triggerItem.label}
        {#if value === triggerItem.id}
          <div
            in:send={{ key: 'trigger' }}
            out:receive={{ key: 'trigger' }}
            class="absolute bottom-1 left-1/2 h-1 w-8 -translate-x-1/2 rounded-full bg-primary"
          ></div>
        {/if}
      </button>
    {/each}
  </div>
    <div class="grow bg-transparent text-foreground p-5">
      {@render children?.()}
    </div>
</div>