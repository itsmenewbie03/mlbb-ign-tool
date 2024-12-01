<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import toast from "svelte-french-toast";

  let game_id = $state("");
  let zone_id = $state("");
  let loading = $state(false);
  let ign = $state("IGN Goes Here...");

  async function get_ign(event: Event) {
    event.preventDefault();
    loading = true;
    try {
      ign = await invoke("get_ign", { game_id, zone_id });
    } catch (err: any) {
      ign = err;
    } finally {
      loading = false;
    }
  }

  async function copy_to_clipboard(event: Event) {
    event.preventDefault();
    navigator.clipboard.writeText(ign);
    toast.success("IGN copied to clipboard!", { position: "bottom-center" });
  }
</script>

<main class="container w-full h-screen p-4">
  <div class="card bg-base-100 w-full shadow-xl">
    <figure>
      <img src="https://iili.io/20T4KEN.jpg" alt="Shoes" />
    </figure>
    <div class="card-body">
      <h2 class="card-title">MLBB IGN GETTER</h2>
      <label class="input input-bordered flex items-center gap-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-4"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15 9h3.75M15 12h3.75M15 15h3.75M4.5 19.5h15a2.25 2.25 0 0 0 2.25-2.25V6.75A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25v10.5A2.25 2.25 0 0 0 4.5 19.5Zm6-10.125a1.875 1.875 0 1 1-3.75 0 1.875 1.875 0 0 1 3.75 0Zm1.294 6.336a6.721 6.721 0 0 1-3.17.789 6.721 6.721 0 0 1-3.168-.789 3.376 3.376 0 0 1 6.338 0Z"
          />
        </svg>
        <input
          type="text"
          class="grow"
          placeholder="Game ID"
          bind:value={game_id}
        />
      </label>
      <label class="input input-bordered flex items-center gap-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-4"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"
          />
        </svg>
        <input
          type="text"
          class="grow"
          placeholder="Zone ID"
          bind:value={zone_id}
        />
      </label>
      <label class="input input-bordered flex items-center gap-2">
        <input
          type="text"
          class="grow"
          placeholder="IGN Goes Here..."
          bind:value={ign}
          readonly
          onclick={copy_to_clipboard}
        />
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-4"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15.666 3.888A2.25 2.25 0 0 0 13.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 0 1-.75.75H9a.75.75 0 0 1-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 0 1-2.25 2.25H6.75A2.25 2.25 0 0 1 4.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 0 1 1.927-.184"
          />
        </svg>
      </label>
      <div class="card-actions justify-end">
        <button
          class="btn btn-primary w-full"
          onclick={get_ign}
          disabled={loading}
        >
          {#if loading}
            <span class="loading loading-dots loading-xs"></span>
            Please wait...
          {:else}
            Get IGN
          {/if}
        </button>
      </div>
    </div>
    <p class="flex items-center justify-center pb-6 text-xs">
      Developed with
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="py-1 size-8"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
        />
      </svg>
      and
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        class="py-1 size-8"
        ><path
          fill="currentColor"
          d="M11 18q-2.925 0-4.962-2.037T4 11V5q0-.825.588-1.412T6 3h12.5q1.45 0 2.475 1.025T22 6.5t-1.025 2.475T18.5 10H18v1q0 2.925-2.037 4.963T11 18M6 8h10V5H6zm12 0h.5q.625 0 1.063-.437T20 6.5t-.437-1.062T18.5 5H18zM4 21v-2h16v2z"
        /></svg
      >
      by
      <a href="https://github.com/itsmenewbie03" class="pl-1" target="_blank">
        @itsmenewbie03</a
      >
    </p>
  </div>
</main>
