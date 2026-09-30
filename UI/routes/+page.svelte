<script lang="ts">
  import { Tabs } from "bits-ui";
  import { authenticate, type AuthRequest, type ApiResponse } from "$lib/auth";
  import { goto } from "$app/navigation";
  import { setUser } from "$lib/auth-store.svelte";

  type AuthMode = AuthRequest["mode"];

  let mode = $state<AuthMode>("signIn");
  let name = $state("");
  let email = $state("");
  let password = $state("");
  let submitting = $state(false);
  let notice = $state<{ text: string; error: boolean } | null>(null);

  function changeMode(nextMode: AuthMode) {
    mode = nextMode;
    notice = null;
  }

  async function submitAuth(event: SubmitEvent) {
    event.preventDefault();
    notice = null;
    submitting = true;

    const request: AuthRequest = {
      mode,
      email: email.trim(),
      password,
      ...(mode === "signUp" ? { name: name.trim() } : {}),
    };

    try {
      let response: ApiResponse = await authenticate(request);
      if (response.success) {
        setUser(response.data.user);

        await goto("/dashboard");
      }
      notice = {
        text: `Authentication request completed`,
        error: false,
      };
    } catch (error) {
      notice = {
        text:
          typeof error === "string"
            ? error
            : "Unable to reach the authentication API.",
        error: true,
      };
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>{mode === "signIn" ? "Sign in" : "Create account"} | Ko Insight</title>
</svelte:head>

<main
  class="grid min-h-screen place-items-center bg-[#f5f6fa] px-5 py-10 text-[#191c20] antialiased sm:px-8"
>
  <section
    class="w-full max-w-md rounded-[28px] border border-[#e1e3e8] bg-white px-6 py-8 shadow-[0_12px_32px_rgba(25,28,32,0.08)] sm:px-10 sm:py-10"
    aria-labelledby="auth-title"
  >
    <header class="mb-8">
      <a
        class="inline-flex items-center gap-3 text-sm font-semibold text-[#004f4f] no-underline"
        href="/"
      >
        <span
          class="grid size-11 place-items-center rounded-2xl bg-[#cce8e5] text-lg font-bold text-[#004f4f]"
          aria-hidden="true">K</span
        >
        Ko Insight
      </a>
      <p class="mb-2 mt-8 text-sm font-medium text-[#006a6a]">
        {mode === "signIn" ? "Welcome back" : "Get started"}
      </p>
      <h1
        id="auth-title"
        class="text-3xl font-semibold leading-tight text-[#191c20]"
      >
        {mode === "signIn" ? "Sign in" : "Create account"}
      </h1>
      <p class="mt-2 text-sm leading-6 text-[#43474e]">
        {mode === "signIn"
          ? "Continue to your Ko Insight workspace."
          : "Create an account to begin your Ko Insight workspace."}
      </p>
    </header>

    <Tabs.Root
      value={mode}
      onValueChange={(value) => changeMode(value as AuthMode)}
      class="w-full"
    >
      <Tabs.List
        class="mb-7 grid grid-cols-2 gap-1 rounded-full bg-[#e9edf2] p-1"
        aria-label="Authentication mode"
      >
        <Tabs.Trigger
          value="signIn"
          class="min-h-11 rounded-full px-3 text-sm font-medium text-[#43474e] outline-none transition-colors hover:bg-white/60 focus-visible:ring-2 focus-visible:ring-[#006a6a] focus-visible:ring-offset-2 data-[state=active]:bg-[#cce8e5] data-[state=active]:text-[#004f4f]"
          >Sign in</Tabs.Trigger
        >
        <Tabs.Trigger
          value="signUp"
          class="min-h-11 rounded-full px-3 text-sm font-medium text-[#43474e] outline-none transition-colors hover:bg-white/60 focus-visible:ring-2 focus-visible:ring-[#006a6a] focus-visible:ring-offset-2 data-[state=active]:bg-[#cce8e5] data-[state=active]:text-[#004f4f]"
          >Create account</Tabs.Trigger
        >
      </Tabs.List>

      <Tabs.Content value={mode}>
        <form class="grid gap-5" onsubmit={submitAuth}>
          {#if mode === "signUp"}
            <label class="grid gap-2 text-sm font-medium text-[#43474e]">
              Name
              <input
                class="min-h-14 w-full rounded-xl border border-[#747981] bg-white px-4 text-base font-normal text-[#191c20] outline-none transition-colors placeholder:text-[#747981] hover:border-[#43474e] focus:border-2 focus:border-[#006a6a]"
                bind:value={name}
                autocomplete="name"
                required
              />
            </label>
          {/if}

          <label class="grid gap-2 text-sm font-medium text-[#43474e]">
            Email
            <input
              class="min-h-14 w-full rounded-xl border border-[#747981] bg-white px-4 text-base font-normal text-[#191c20] outline-none transition-colors placeholder:text-[#747981] hover:border-[#43474e] focus:border-2 focus:border-[#006a6a]"
              bind:value={email}
              type="email"
              autocomplete="email"
              required
            />
          </label>

          <label class="grid gap-2 text-sm font-medium text-[#43474e]">
            Password
            <input
              class="min-h-14 w-full rounded-xl border border-[#747981] bg-white px-4 text-base font-normal text-[#191c20] outline-none transition-colors placeholder:text-[#747981] hover:border-[#43474e] focus:border-2 focus:border-[#006a6a]"
              bind:value={password}
              type="password"
              autocomplete={mode === "signIn"
                ? "current-password"
                : "new-password"}
              minlength="8"
              required
            />
          </label>

          <button
            class="mt-2 min-h-12 rounded-full bg-[#006a6a] px-6 text-sm font-semibold tracking-wide text-white shadow-sm transition-colors hover:bg-[#005757] focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#006a6a] disabled:cursor-wait disabled:opacity-70"
            type="submit"
            disabled={submitting}
          >
            {submitting
              ? "Submitting..."
              : mode === "signIn"
                ? "Sign in"
                : "Create account"}
          </button>
        </form>
      </Tabs.Content>
    </Tabs.Root>

    {#if notice}
      <p
        class:error={notice.error}
        class="mt-5 rounded-xl bg-[#e9edf2] px-4 py-3 text-sm text-[#43474e] [&.error]:bg-[#ffdad6] [&.error]:text-[#93000a]"
        role="status"
        aria-live="polite"
      >
        {notice.text}
      </p>
    {/if}
  </section>
</main>
