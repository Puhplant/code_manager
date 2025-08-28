<script lang="ts">
  import { Router, type RouteConfig } from '@mateothegreat/svelte5-router';
  import Login from './pages/Login.svelte';
  import Home from './pages/Home.svelte';

  const routes: RouteConfig[] = [
    { path: '/', component: Home },
    { path: '/login', component: Login },
  ];

</script>

<Router routes={routes} />
