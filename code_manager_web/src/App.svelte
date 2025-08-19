<script lang="ts">
  import { Router, type RouteConfig } from '@mateothegreat/svelte5-router';
  import Login from './pages/Login.svelte';

  const routes: RouteConfig[] = [
    { path: '/login', component: Login },
  ];

</script>

<Router routes={routes} />
