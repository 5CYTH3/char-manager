<script lang="ts">
    import type { CharData } from "../schemas/char";
    import CharCard from "$lib/components/CharCard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { type PageProps } from "./$types";

    let { data }: PageProps = $props();

    let infos: CharData[] | undefined = $state(data.infos);

    async function get_chars(event: Event) {
        event.preventDefault();
        infos = await invoke("get_chars_infos");
    }

    async function create_dummy(event: Event) {
        event.preventDefault();
        let c_data: CharData = await invoke("get_dummy_data");
        console.log(c_data);
        await invoke("create_char_file", { cData: c_data }).catch((e) =>
            console.log(e),
        );
    }

    async function print_dummy(event: Event) {
        event.preventDefault();
        let dum = await invoke("get_dummy_data");
        console.log(dum);
    }
    $inspect(infos).with(console.trace);
</script>

<button onclick={create_dummy}>create_default_char</button>
<button onclick={print_dummy}>get_dummy_data</button>
<button onclick={get_chars}>get_characters</button>
<button>delete_default_char</button>

<div class="cards-wrapper">
    {#if infos !== undefined}
        {#each infos as c}
            <CharCard
                name={c.name}
                pic_url={c.img_path}
                level={c.lvl}
                _class={c.class}
            />
        {/each}
    {/if}
</div>

<style>
    .cards-wrapper {
        height: 70vh;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
