<script module lang="ts">
  import { createToaster, melt } from '@melt-ui/svelte';
  import { fly, fade } from 'svelte/transition';
  import { flip } from 'svelte/animate';

  interface ToastData {
    title: string;
    description: string;
    color: string;
  }

  const {
    elements: { content, title, description, close },
    helpers,
    states: { toasts },
    actions: { portal }
  } = createToaster<ToastData>({
    closeDelay: 5000,
  });

  export function showToast(data: ToastData) {
    helpers.addToast({ data });
  }

  export function showError(message: string) {
    helpers.addToast({
      data: {
        title: 'Error',
        description: message,
        color: 'red'
      }
    });
  }

  export function showSuccess(message: string) {
    helpers.addToast({
      data: {
        title: 'Success',
        description: message,
        color: 'green'
      }
    });
  }
</script>

<div use:portal class="fixed top-4 left-1/2 transform -translate-x-1/2">
  {#each $toasts as { id, data } (id)}
    <div use:melt={$content(id)} in:fly={{ y: '-100%', duration: 300 }} out:fade={{ duration: 200 }} animate:flip={{ duration: 500 }}>
      <div class="bg-card border-l-6 border-destructive rounded-lg shadow-2xl p-6 min-w-[400px] max-w-md mb-2">
        <div class="flex items-start space-x-4">
          <div class="flex-shrink-0">
            <div class="w-6 h-6 bg-destructive/10 rounded-full flex items-center justify-center">
              <svg class="w-4 h-4 text-destructive" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
              </svg>
            </div>
          </div>
          <div class="flex-1 min-w-0">
            <h3 use:melt={$title(id)} class="text-base font-semibold text-card-foreground mb-1">
              {data.title}
            </h3>
            <div use:melt={$description(id)} class="text-sm text-muted-foreground leading-relaxed">
              {data.description}
            </div>
          </div>
          <button use:melt={$close(id)} class="flex-shrink-0 text-muted-foreground hover:text-color-foreground transition-colors duration-200 p-1 rounded-full hover:bg-color-accent cursor-pointer" aria-label="close notification">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  {/each}
</div> 