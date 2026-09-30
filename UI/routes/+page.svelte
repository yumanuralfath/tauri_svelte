<script lang="ts">
  import { authenticate, type AuthRequest } from "$lib/tauri";

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
      await authenticate(request);
      notice = {
        text: "Authentication request completed.",
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

<main class="auth-page">
  <section class="auth-form" aria-labelledby="auth-title">
    <a class="brand" href="/">Ko Insight</a>
    <h1 id="auth-title">
      {mode === "signIn" ? "Sign in" : "Create account"}
    </h1>

    <div class="mode-switch" aria-label="Authentication mode">
      <button
        class:active={mode === "signIn"}
        type="button"
        aria-pressed={mode === "signIn"}
        onclick={() => changeMode("signIn")}>Sign in</button
      >
      <button
        class:active={mode === "signUp"}
        type="button"
        aria-pressed={mode === "signUp"}
        onclick={() => changeMode("signUp")}>Create account</button
      >
    </div>

    <form onsubmit={submitAuth}>
      {#if mode === "signUp"}
        <label>
          Name
          <input bind:value={name} autocomplete="name" required />
        </label>
      {/if}

      <label>
        Email
        <input bind:value={email} type="email" autocomplete="email" required />
      </label>

      <label>
        Password
        <input
          bind:value={password}
          type="password"
          autocomplete={mode === "signIn" ? "current-password" : "new-password"}
          minlength="8"
          required
        />
      </label>

      <button class="submit-button" type="submit" disabled={submitting}>
        {submitting
          ? "Submitting..."
          : mode === "signIn"
            ? "Sign in"
            : "Create account"}
      </button>
    </form>

    {#if notice}
      <p
        class:error={notice.error}
        class="notice"
        role="status"
        aria-live="polite"
      >
        {notice.text}
      </p>
    {/if}
  </section>
</main>
