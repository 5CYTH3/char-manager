<script lang="ts">
    import { Undo2 } from "@lucide/svelte";
    import { goto } from "$app/navigation";
    import Button from "$lib/components/Button.svelte";
    import { type PageProps } from "./$types";
    import Box from "$lib/components/Box.svelte";
    import LockingBox from "$lib/components/LockingBox.svelte";
    import { LockableCell, lockingMutex } from "$lib/utils.svelte";
    import Field from "$lib/components/Field.svelte";

    let { data }: PageProps = $props();

    let stats_map = new Map();
    stats_map.set("Strength", data.stats.strg);
    stats_map.set("Wisdom", data.stats.wisd);
    stats_map.set("Charisma", data.stats.chrs);
    stats_map.set("Dexterity", data.stats.dext);
    stats_map.set("Intelligence", data.stats.intl);
    stats_map.set("Constitution", data.stats.cnst);
</script>

<main>
    <section id="lsidebar">
        <Button action={() => goto("/", { replaceState: true })}>
            <Undo2></Undo2> Back to characters
        </Button>
        <Box width="100%" height="auto">
            <div class="char-id-box">
                <!-- TODO: Need to get an image in case no character image has been provided by the user. -->
                <img src={data.img_path ? data.img_path : ""} alt="character" />
                <p>{data.name}</p>
            </div>
        </Box>
        <LockingBox
            height="auto"
            width="100%"
            title="Stats"
            cell={LockableCell.StatsCell}
        >
            <!--
                PROPOSAL -- Instead of creating a map (see the <script> tag in the file), directly altering the schema to make the `stats`
                field an array of { name: "name_of_the_stat", value: "value_of_the_stat" }.
            -->
            {#each stats_map as [k, v]}
                <div>
                    <p>{k}:</p>
                    <Field value={v}></Field>
                </div>
            {/each}

            <Field value={data.stats.chrs}></Field>
        </LockingBox>
    </section>
    <section id="center">
        <div class="infos-container">
            <div class="infos">
                <p>Alignement: {data.alignment}</p>
                <p>Race: {data.race}</p>
                <p>Class: {data.class}</p>
                <p>Age: {data.appearance.age}</p>
                <p>Height: {data.appearance.height}</p>
                <p>Weight: {data.appearance.weight}</p>
            </div>
        </div>
    </section>
    <section id="rsidebar"></section>
</main>

<style>
    :global(*) {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        font-size: 20px;
    }

    main {
        display: flex;
        flex-flow: row;
        align-items: start;
        width: 100vw;
        height: 100vh;
    }

    #lsidebar {
        display: flex;
        align-items: center;
        flex-direction: column;
        background-color: #e4e4e4;
        height: 100%;
        width: 20vw;
        padding: 20px;
        gap: 25px;

        .char-id-box {
            height: 100%;
            width: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-flow: column;

            img {
                height: 150px;
                width: 150px;
                border-radius: 50%;
            }

            p {
                margin-top: 10px;
            }
        }
    }

    #center {
        padding: 20px;
        width: 100%;

        .infos-container {
            width: 100%;
            height: max-content;
            display: flex;
            justify-content: start;
            align-items: center;
            background-color: #f4f4f4;
            .infos {
                display: flex;
                flex-direction: row;
                p {
                    margin: 20px;
                }
            }
        }
    }

    #rsidebar {
        width: 30vw;
    }
</style>
