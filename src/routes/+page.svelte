<script lang="ts">
    import ExternalIcon from "$lib/icons/ExternalIcon.svelte";
    import SetupScreen from "$lib/SetupScreen.svelte";
    import { didSetup } from "$lib/store";
    import { fetchMirrors, search, type Track } from "$lib/youtube";
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";

    let url = $state("");
    let lookup = $state('"{title}" - {artist} {album}');
    let mirrors: Array<{ url: string; name: string }> = $state([]);
    let currentMirror = $state("");
    let refreshRate = $state(10);
    let tickerId: number;
    let color = $state("");

    let track: Track = $state({
        title: "Unknown",
        artist: "Unknown",
        album: "Unknown",
        art: "https://placehold.co/456x456/transparent/FF?text=?&font=noto-sans",
    });

    const tick = async () => {
        if ($didSetup == false) return;
        let nTrack: Track = await invoke("get_track");

        if (track.title != nTrack.title && track.artist != nTrack.artist) {
            track = nTrack;
            const result = await search(currentMirror, track, lookup);
            track.id = result.id;
            track.art =
                result.thumbnail ||
                "https://placehold.co/456x456/transparent/FF?text=?&font=noto-sans";
        }
    };

    onMount(async () => {
        url = await invoke("get_url");
        mirrors = await fetchMirrors();
        currentMirror = mirrors[0].url;

        tickerId = setInterval(tick, refreshRate * 1000);
    });

    $effect(() => {
        clearInterval(tickerId);
        setInterval(tick, refreshRate * 1000);
    });

    $effect(() => {
        if ($didSetup) {
            (async () => {
                url = await invoke("get_url");
                await tick();
            })();
        }
    });
</script>

{#if !$didSetup}
    <SetupScreen />
{/if}

<main class="z-0 p-4 {!$didSetup ? 'blur **:pointer-events-none' : ''}">
    <div class="flex items-center px-4 mb-4">
        <h1
            class="text-xs font-bold tracking-tight uppercase pr-2 text-neutral-700"
        >
            ListenTogether
        </h1>
        <div class="h-0.5 w-full bg-neutral-700"></div>
    </div>
    <p>
        Listening with <code
            class="font-mono italic bg-neutral-800 text-neutral-300 p-1"
            >{url}</code
        >.
    </p>

    <div class="m-8 p-4 border border-neutral-800 rounded">
        <h2 class="font-semibold tracking-tight mb-2">Options</h2>
        <div class="flex *:grow gap-4">
            <div class="gap-2">
                <label
                    class="block text-sm mb-1 text-neutral-700 dark:text-neutral-300 text-center"
                    for="mirror"
                >
                    Search Mirror
                </label>
                <select
                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-700 rounded-lg
             bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100
           focus:outline-none focus:ring-2 focus:ring-neutral-500"
                    bind:value={currentMirror}
                    id="mirror"
                    style="-webkit-appearance: none;"
                >
                    {#each mirrors as m}
                        <option value={m.url}>{m.name}</option>
                    {/each}
                </select>
            </div>
            <div class="gap-2">
                <label
                    class="block text-sm mb-1 text-neutral-700 dark:text-neutral-300 text-center"
                    for="lookup"
                >
                    Lookup String
                </label>
                <input
                    type="text"
                    id="lookup"
                    bind:value={lookup}
                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-700 rounded-lg
             bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100
             placeholder-neutral-400 dark:placeholder-neutral-500
             focus:ring-2 focus:ring-neutral-500 focus:outline-none"
                />
            </div>
            <div class="gap-2">
                <label
                    class="block text-sm mb-1 text-neutral-700 dark:text-neutral-300 text-center"
                    for="refresh"
                >
                    Refresh Rate
                </label>
                <input
                    type="number"
                    id="refresh"
                    min="4"
                    max="120"
                    bind:value={refreshRate}
                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-700 rounded-lg
             bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100
             placeholder-neutral-400 dark:placeholder-neutral-500
             focus:ring-2 focus:ring-neutral-500 focus:outline-none"
                />
            </div>
        </div>
    </div>

    <div class="flex justify-center items-center flex-col p-8">
        <img src={track.art} alt="Cover Art" class="size-64" />
        <div class="p-4 text-center">
            <p class="font-bold text-lg flex items-center gap-1">
                {track.title}
                <button
                    onclick={async () =>
                        await openUrl(
                            "https://music.youtube.com/watch?v=" + track.id
                        )}><ExternalIcon className="size-4" /></button
                >
            </p>
            <p class="text-neutral-500">{track.artist}</p>
        </div>
    </div>

    <iframe
        title="Player"
        width="640"
        height="390"
        src="http://www.youtube-nocookie.com/embed/{track.id}?autoplay=1&disablekb=1&rel=0"
        frameborder="0"
        allowfullscreen
        class="hidden"
        aria-hidden="true"
    ></iframe>
</main>
