import { fetch as fetcher } from "@tauri-apps/plugin-http";

export interface Track {
    title: string;
    artist: string;
    album: string;
    id?: string;
    art: string;
}

export async function search(mirror: string, track: Track, lookup: string) {
    const query = lookup
        .replace("{title}", encodeURIComponent(track.title))
        .replace("{artist}", encodeURIComponent(track.artist))
        .replace("{album}", encodeURIComponent(track.album));

    console.log(`Searching for <<${decodeURIComponent(query)}>>`);

    const res = await fetcher(mirror + "/search?filter=music_songs&q=" + query);
    const json = await res.json();

    console.log("Search result (first selected): " + json.items[0].id);
    return {
        id: json.items[0].url.replace("/watch?v=", ""),
        thumbnail: json.items[0].thumbnail,
    };
}

export async function fetchMirrors(): Promise<{ url: string; name: string }[]> {
    console.log("Mirrors requested.");
    const res = await fetch("https://piped-instances.kavin.rocks/");

    let instances = await res.json();

    console.log("Mirrors fetched.");

    return instances.map(
        (i: { api_url: string; locations: string; name: string }) => {
            return {
                url: i.api_url,
                name: i.locations.trim() + " " + i.name.trim(),
            };
        }
    );
}
