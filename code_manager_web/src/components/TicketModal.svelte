<script lang="ts">
  import Modal from './Modal.svelte';
  import Input from './Input.svelte';
  import Button from './Button.svelte';
  import Select from './Select.svelte';
  import type { MinTicketResponse, ColumnResponse } from '../types/board';

  interface TicketModalProps {
    open: boolean;
    mode?: 'create' | 'edit';
    ticket?: MinTicketResponse | null;
    boardId?: number;
    columnOptions?: ColumnResponse[];
    onClose: () => void;
    onSubmit?: (data: { title: string; description: string; boardId?: number; ticketId?: number; columnId?: number }) => Promise<void>;
  }

  let { 
    open = $bindable(false),
    mode = 'create',
    ticket = null,
    boardId,
    columnOptions = [],
    onClose,
    onSubmit
  }: TicketModalProps = $props();

  let title = $state('');
  let description = $state('');
  let selectedColumn = $state<{ value: number; label: string } | null>(null);
  let isSubmitting = $state(false);

  $effect(() => {
    if (open) {
      if (mode === 'edit' && ticket) {
        title = ticket.title;
        description = '';
        if (ticket.column_id && columnOptions.length > 0) {
          const column = columnOptions.find(col => col.id === ticket.column_id);
          selectedColumn = column ? { value: column.id, label: column.name } : null;
        } else {
          selectedColumn = null;
        }
      } else {
        title = '';
        description = '';
        selectedColumn = null;
      }
    }
  });

  async function handleSubmit() {
    if (!title.trim()) return;
    
    try {
      isSubmitting = true;
      await onSubmit?.({
        title: title.trim(),
        description: description.trim(),
        boardId,
        ticketId: mode === 'edit' ? ticket?.id : undefined,
        columnId: selectedColumn?.value
      });
      handleClose();
    } catch (error) {
      console.error('Failed to submit ticket:', error);
    } finally {
      isSubmitting = false;
    }
  }

  function handleClose() {
    onClose?.();
    open = false;
  }

  const modalTitle = mode === 'create' ? 'Create New Ticket' : 'Edit Ticket';
  const modalDescription = mode === 'create' ? 'Add a new ticket to your board' : 'Update ticket details';
  const submitButtonText = mode === 'create' ? 'Create Ticket' : 'Update Ticket';
</script>

<Modal 
  title={modalTitle}
  description={modalDescription}
  open={open}
  size="lg"
  onOpenChange={isOpen => !isOpen && handleClose()}
>
  {#snippet children()}
    <div class="space-y-4 flex flex-col h-full">
      <div>
        <label for="ticket-title" class="text-sm font-medium text-foreground mb-2 block">Title</label>
        <Input 
          id="ticket-title"
          placeholder="Enter ticket title..."
          bind:value={title}
          class="w-full"
          disabled={isSubmitting}
          autoComplete="off"
        />
      </div>
      <div class="flex gap-4">
        <div class="flex-1">
          <label for="ticket-description" class="text-sm font-medium text-foreground mb-2 block">Description</label>
          <textarea 
            id="ticket-description"
            placeholder="Enter ticket description..."
            bind:value={description}
            disabled={isSubmitting}
            class="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 resize-none disabled:opacity-50 disabled:cursor-not-allowed"
            rows="4"
          ></textarea>
        </div>
        <div class="w-64">
          <Select
            options={columnOptions.map(col => ({ value: col.id, label: col.name }))}
            selected={selectedColumn}
            onSelectedChange={(value) => {
              if (value) {
                const column = columnOptions.find(col => col.id === value);
                selectedColumn = column ? { value: column.id, label: column.name } : null;
              } else {
                selectedColumn = null;
              }
            }}
            placeholder="Select column..."
            labelText="Column"
          />
        </div>
      </div>
      <div class="flex justify-end gap-3 pt-4 mt-auto">
        <Button 
          variant="outline" 
          onclick={handleClose}
          disabled={isSubmitting}
        >
          Cancel
        </Button>
        <Button 
          onclick={handleSubmit}
          disabled={!title.trim() || isSubmitting}
          loading={isSubmitting}
        >
          {submitButtonText}
        </Button>
      </div>
    </div>
  {/snippet}
</Modal>
