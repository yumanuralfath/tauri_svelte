<script lang="ts">
  import { getUser } from "$lib/auth-store.svelte";

  const user = getUser();
</script>

<svelte:head>
  <title>Dashboard</title>
</svelte:head>

<main
  class="min-h-screen bg-[#f5f6fa] px-5 py-8 text-[#191c20] antialiased sm:px-8 sm:py-10"
>
  <section class="mx-auto w-full max-w-5xl">
    <header class="mb-8 flex items-center gap-3">
      <span
        class="grid size-11 place-items-center rounded-2xl bg-[#cce8e5] text-lg font-bold text-[#004f4f]"
        aria-hidden="true">K</span
      >
      <div>
        <p class="text-sm font-medium text-[#006a6a]">Ko Insight</p>
        <h1 class="text-2xl font-semibold">Dashboard</h1>
      </div>
    </header>
    <section
      class="rounded-[28px] border border-[#e1e3e8] bg-white p-6 shadow-[0_4px_16px_rgba(25,28,32,0.06)] sm:p-8"
    >
      <p class="text-sm font-medium text-[#006a6a]">Workspace</p>
      <h2 class="mt-2 text-xl font-semibold">Welcome, {user?.username}</h2>
      <p class="mt-2 text-sm leading-6 text-[#43474e]">
        You are signed in to Ko Insight.
      </p>
    </section>
  </section>
</main>
