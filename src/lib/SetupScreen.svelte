<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CheckIcon from "./icons/CheckIcon.svelte";
    import ErrorIcon from "./icons/ErrorIcon.svelte";
    import QuestionIcon from "./icons/QuestionIcon.svelte";
    import ArrowRightIcon from "./icons/ArrowRightIcon.svelte";
    import { didSetup } from "./store";

    let url = $state("");
    let loading = $state(false);
    let error: string | null = $state(null);
    let success = $state(false);

    async function handleTest() {
        loading = true;
        error = null;
        success = false;

        try {
            await invoke("set_url", { newUrl: url });
            const result = await invoke("test_url");
            console.log(result);
            if (result) {
                success = true;
            } else {
                error = "Invalid or unreachable server.";
            }
        } catch (e) {
            // @ts-ignore oh my god just shut up
            error = e;
        } finally {
            loading = false;
        }
    }

    async function handleSave() {
        try {
            await invoke("set_url", { newUrl: url });
            didSetup.set(true);
        } catch (e) {
            error = "Could not save URL.";
        }
    }
</script>

<div class="fixed inset-0 flex items-center justify-center z-10">
    <div
        class="relative bg-neutral-50 dark:bg-neutral-900 rounded shadow-xl w-full max-w-3/5 min-h-[50vh] p-6"
    >
        <h2
            class="text-xl font-semibold mb-2 text-neutral-900 dark:text-neutral-100 tracking-tight"
        >
            Welcome!
        </h2>

        <p class="mb-4 font-light">
            Ask your friend for their listen URL. Then, click test.
        </p>

        <div class="flex gap-2">
            <input
                type="text"
                id="server"
                bind:value={url}
                placeholder="http://255.255.255.255/listen-with-me"
                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-700 rounded-lg
             bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100
             placeholder-neutral-400 dark:placeholder-neutral-500
             focus:ring-2 focus:ring-neutral-500 focus:outline-none"
            />
            <button
                onclick={handleTest}
                class="px-4 py-2 rounded bg-neutral-300 dark:bg-neutral-700 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed"
                disabled={loading}
            >
                {loading ? "Testing..." : "Test"}
            </button>
        </div>

        <div
            class="flex items-center gap-4 mt-4 p-4 rounded bg-neutral-100 dark:bg-neutral-800 {error
                ? 'border-1 border-red-500 dark:text-red-400'
                : success
                  ? 'border-1 border-green-500 dark:text-green-400'
                  : ''}"
        >
            {#if error}
                <div>
                    <ErrorIcon className="text-red-500 dark:text-red-400" />
                </div>
                <p>{error}</p>
            {:else if success}
                <div>
                    <CheckIcon className="text-green-500 dark:text-green-400" />
                </div>
                <p>It works!</p>
            {:else}
                <div>
                    <QuestionIcon />
                </div>
                {#if loading}
                    <p>Testing...</p>
                {:else}
                    <p>Test didn't run yet.</p>
                {/if}
            {/if}
        </div>

        <div class="my-4 flex justify-evenly items-center">
            <div class="h-0.5 w-full bg-neutral-500"></div>
            <p class="px-2 text-sm text-neutral-500">OR</p>
            <div class="h-0.5 w-full bg-neutral-500"></div>
        </div>

        <button class="p-8 bg-neutral-800 rounded w-full text-left">
            <h2 class="font-medium">Be the host</h2>
            <p class="text-xs">Coming soon!</p>
        </button>

        <div class="float-right flex justify-end gap-2 mt-6">
            <button
                onclick={handleSave}
                class="px-4 py-1 rounded bg-neutral-800 dark:bg-neutral-200
               text-neutral-100 dark:text-neutral-900
               hover:bg-neutral-700 dark:hover:bg-neutral-300
               disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
                disabled={!success}
            >
                Continue
                <ArrowRightIcon className="size-5" />
            </button>
        </div>
    </div>
</div>
