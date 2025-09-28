<script lang="ts">
  import Button from '../components/Button.svelte';
  import Card from '../components/Card.svelte';
  import Select from '../components/Select.svelte';
  import Tabs from '../components/Tabs.svelte';
  import Toast, { showError, showSuccess } from '../components/Toast.svelte';
  import TicketModal from '../components/TicketModal.svelte';
  import { get, post, put } from '../utils/request';
  import type { 
    Board, 
    BoardTicketsResponse, 
    BacklogTicketsResponse, 
    MinTicketResponse, 
    BoardResponse,
    CreateTicketRequest,
    CreateTicketResponse,
    EditTicketRequest,
    ColumnResponse
  } from '../types/board';
  import { Plus } from 'lucide-svelte';

  let boards = $state<Board[]>([]);
  let boardTickets = $state<BoardTicketsResponse | null>(null);
  let backlogTickets = $state<BacklogTicketsResponse | null>(null);
  let selectedBoard = $state<Board | null>(null);
  let isLoading = $state(false);
  let activeTab = $state('board');
  let isTicketModalOpen = $state(false);
  let ticketModalMode = $state<'create' | 'edit'>('create');
  let selectedTicket = $state<MinTicketResponse | null>(null);
  let columnOptions = $state<ColumnResponse[]>([]);

  async function fetchBoards() {
    try {
      isLoading = true;
      const response = await get<BoardResponse>('/api/boards');
      boards = response.data.boards || [];
      if (boards.length > 0) {
        selectedBoard = boards[0] || null;
        await fetchBoardTickets();
      }
    } catch (error) {
      showError('Failed to fetch boards');
    } finally {
      isLoading = false;
    }
  }

  async function fetchBoardTickets() {
    if (!selectedBoard) return;
    
    try {
      const boardResponse = await get<BoardTicketsResponse>('/api/tickets/board', {
        board_id: selectedBoard.id,
        ticket_type: 'board'
      });
      boardTickets = boardResponse.data;
      
      // Update column options from boardTickets
      if (boardTickets && boardTickets.columns) {
        columnOptions = boardTickets.columns.map(col => col.column);
      } else {
        columnOptions = [];
      }
    } catch (error) {
      showError('Failed to fetch board tickets');
    }
  }

  async function fetchBacklogTickets() {
    if (!selectedBoard) return;
    
    try {
      const backlogResponse = await get<BacklogTicketsResponse>('/api/tickets/board', {
        board_id: selectedBoard.id,
        ticket_type: 'backlog'
      });
      backlogTickets = backlogResponse.data;
    } catch (error) {
      showError('Failed to fetch backlog tickets');
    }
  }

  function selectBoard(board: Board) {
    selectedBoard = board;
    fetchBoardTickets();
  }

  function setActiveTab(tab: string) {
    activeTab = tab;
    if (tab === 'board' && selectedBoard && !boardTickets) {
      fetchBoardTickets();
    } else if (tab === 'backlog' && selectedBoard && !backlogTickets) {
      fetchBacklogTickets();
    }
    return tab;
  }

  function openCreateTicketModal() {
    ticketModalMode = 'create';
    selectedTicket = null;
    isTicketModalOpen = true;
  }

  function openEditTicketModal(ticket: MinTicketResponse) {
    ticketModalMode = 'edit';
    selectedTicket = ticket;
    isTicketModalOpen = true;
  }

  function closeTicketModal() {
    isTicketModalOpen = false;
    selectedTicket = null;
  }

  async function handleTicketSubmit(data: { title: string; description: string; boardId?: number; ticketId?: number; columnId?: number }) {
    if (!selectedBoard) return;
    
    try {
      if (ticketModalMode === 'create') {
        const createRequest: CreateTicketRequest = {
          title: data.title,
          description: data.description || 'No description provided',
          board_id: selectedBoard.id,
          column_id: data.columnId,
          position: undefined
        };
        
        const response = await post<CreateTicketResponse>('/api/tickets', createRequest);
        console.log('Ticket created successfully:', response.data);
        showSuccess('Ticket created successfully!');
      } else {
        const editRequest: EditTicketRequest = {
          title: data.title,
          description: data.description || 'No description provided',
          column_id: data.columnId
        };
        
        await put(`/api/tickets/${data.ticketId}`, editRequest);
        console.log('Ticket updated successfully');
        showSuccess('Ticket updated successfully!');
      }
      
      // Refresh tickets after creation/update
      if (activeTab === 'board') {
        await fetchBoardTickets();
      } else if (activeTab === 'backlog') {
        await fetchBacklogTickets();
      }
    } catch (error) {
      console.error('API Error:', error);
      showError(`Failed to ${ticketModalMode === 'create' ? 'create' : 'update'} ticket`);
    }
  }

  $effect(() => {
    fetchBoards();
  });
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 p-6">
  <div class="max-w-full mx-auto">
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-white mb-2">Code Manager</h1>
      <p class="text-slate-300">Manage your development projects and tasks</p>
    </div>

    <div class="mb-6">
      <Select
        options={boards.map(board => ({ value: board.id, label: board.name }))}
        selected={selectedBoard ? { value: selectedBoard.id, label: selectedBoard.name } : null}
        placeholder="Select a board..."
        onSelectedChange={(boardId: number | undefined) => {
          if (boardId) {
            const selectedBoard = boards.find(b => b.id === boardId);
            if (selectedBoard) {
              selectBoard(selectedBoard);
            }
          }
        }}
      />
    </div>

    {#if selectedBoard}
      <Tabs
        tabs={[
          { id: 'board', label: 'Board' },
          { id: 'backlog', label: 'Backlog' }
        ]}
        defaultValue={'board'}
        value={activeTab}
        onValueChange={setActiveTab}
      >
        {#snippet actions()}
          <Button 
            variant="default" 
            size="sm" 
            onclick={openCreateTicketModal}
            class="flex items-center gap-2"
          >
            <Plus class="w-4 h-4" />
            New Ticket
          </Button>
        {/snippet}

      <div class="p-6">
        {#if activeTab === 'board' && boardTickets}
          <div class="overflow-x-auto">
            <div class="flex gap-6 min-w-max px-2">
              {#each boardTickets.columns as column}
                <div class="flex flex-col h-full w-80 flex-shrink-0">
                  <!-- Column Header -->
                  <div class="bg-slate-800/50 rounded-lg p-4 mb-4 border border-slate-700">
                    <h3 class="text-lg font-semibold text-white text-center">{column.column.name}</h3>
                    <div class="text-sm text-slate-400 text-center mt-1">
                      {column.tickets.length} ticket{column.tickets.length !== 1 ? 's' : ''}
                    </div>
                  </div>
                  
                  <!-- Drop Zone Area -->
                  <div class="flex-1 min-h-[500px] bg-slate-800/20 rounded-lg border-2 border-dashed border-slate-600 p-4 transition-colors hover:border-slate-500 hover:bg-slate-800/30">
                    <div class="space-y-3">
                      {#each column.tickets as ticket}
                        <Card>
                          <div 
                            class="p-4 cursor-pointer hover:bg-slate-800/50 transition-colors rounded-lg" 
                            role="button"
                            tabindex="0"
                            onclick={() => openEditTicketModal(ticket)}
                            onkeydown={(e) => e.key === 'Enter' && openEditTicketModal(ticket)}
                          >
                            <div class="flex items-start justify-between mb-2">
                              <h4 class="font-medium text-white">{ticket.title}</h4>
                            </div>
                            <div class="text-xs text-slate-400">
                              Position: {ticket.position || 'N/A'}
                            </div>
                          </div>
                        </Card>
                      {/each}
                      
                      <!-- Empty state indicator -->
                      {#if column.tickets.length === 0}
                        <div class="text-center py-8 text-slate-500">
                          <div class="text-sm">Drop tickets here</div>
                          <div class="text-xs mt-1">or create a new ticket</div>
                        </div>
                      {/if}
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {:else if activeTab === 'backlog' && backlogTickets}
          <div class="space-y-4">
            <h3 class="text-lg font-semibold text-white mb-4">Backlog</h3>
            <div class="space-y-3">
              {#each backlogTickets.tickets as ticket}
                <Card>
                  <div 
                    class="p-4 cursor-pointer hover:bg-slate-800/50 transition-colors" 
                    role="button"
                    tabindex="0"
                    onclick={() => openEditTicketModal(ticket)}
                    onkeydown={(e) => e.key === 'Enter' && openEditTicketModal(ticket)}
                  >
                    <div class="flex items-start justify-between mb-2">
                      <div>
                        <h4 class="font-medium text-white">{ticket.title}</h4>
                        <div class="flex items-center space-x-2 mt-1">
                          <span class="text-xs px-2 py-1 bg-slate-700 text-slate-300 rounded">
                            {ticket.column_id ? `Column ${ticket.column_id}` : 'No Column'}
                          </span>
                          <span class="text-xs text-slate-400">
                            Position: {ticket.position || 'N/A'}
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                </Card>
              {/each}
            </div>
          </div>
        {/if}
      </div>
      </Tabs>
    {:else if isLoading}
      <div class="text-center py-12">
        <div class="text-white">Loading boards...</div>
      </div>
    {:else}
      <div class="text-center py-12">
        <div class="text-slate-400">No boards available</div>
      </div>
    {/if}
  </div>
  
  <TicketModal 
    open={isTicketModalOpen}
    mode={ticketModalMode}
    ticket={selectedTicket}
    boardId={selectedBoard?.id}
    columnOptions={columnOptions}
    onClose={closeTicketModal}
    onSubmit={handleTicketSubmit}
  />
</div>

<Toast /> 