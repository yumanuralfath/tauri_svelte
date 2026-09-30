<script lang="ts">
  import { getUser } from "$lib/auth-store.svelte";

  const user = getUser();
</script>

<svelte:head>
  <title>Dashboard</title>
</svelte:head>

<main>
  <h1>Dashboard</h1>
  <p>login successfully , Welcome {user?.username}!</p>
</main>
