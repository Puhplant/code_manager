<script lang="ts">
  import Button from '../components/Button.svelte';
  import Card from '../components/Card.svelte';
  import Select from '../components/Select.svelte';
  import Tabs from '../components/Tabs.svelte';
  import Toast, { showError } from '../components/Toast.svelte';
  import { get, post } from '../utils/request';
  import type { Board, BoardTicketsResponse, BacklogTicketsResponse, MinTicketResponse, BoardResponse } from '../types/board';

  type SelectOption = { value: number; label: string };

  let boards = $state<Board[]>([]);
  let boardTickets = $state<BoardTicketsResponse | null>(null);
  let backlogTickets = $state<BacklogTicketsResponse | null>(null);
  let selectedBoard = $state<Board | null>(null);
  let isLoading = $state(false);
  let activeTab = $state('board');

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

  $effect(() => {
    fetchBoards();
  });
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 p-6">
  <div class="max-w-7xl mx-auto">
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-white mb-2">Code Manager</h1>
      <p class="text-slate-300">Manage your development projects and tasks</p>
    </div>

    <div class="mb-6">
      <Select
        options={boards.map(board => ({ value: board.id, label: board.name }))}
        selected={selectedBoard ? { value: selectedBoard.id, label: selectedBoard.name } : null}
        placeholder="Select a board..."
        onSelectedChange={(board: SelectOption | null) => {
          if (board && typeof board.value === 'number') {
            const selectedBoard = boards.find(b => b.id === board.value);
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

      <div class="p-6">
        {#if activeTab === 'board' && boardTickets}
          <div class="grid grid-cols-1 md:grid-cols-{boardTickets.columns.length} gap-6">
            {#each boardTickets.columns as column}
              <div class="space-y-4">
                <h3 class="text-lg font-semibold text-white">Column {column.column_id}</h3>
                <div class="space-y-3">
                  {#each column.tickets as ticket}
                    <Card>
                      <div class="p-4">
                        <div class="flex items-start justify-between mb-2">
                          <h4 class="font-medium text-white">{ticket.title}</h4>
                        </div>
                        <div class="text-xs text-slate-400">
                          Position: {ticket.position || 'N/A'}
                        </div>
                      </div>
                    </Card>
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        {:else if activeTab === 'backlog' && backlogTickets}
          <div class="space-y-4">
            <h3 class="text-lg font-semibold text-white mb-4">Backlog</h3>
            <div class="space-y-3">
              {#each backlogTickets.tickets as ticket}
                <Card>
                  <div class="p-4">
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
</div>

<Toast /> 