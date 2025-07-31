<script lang="ts">
    import { Undo2 } from "@lucide/svelte";
    import { goto } from "$app/navigation";
    import Button from "$lib/components/Button.svelte";
    import { type PageProps } from "./$types";
    import Box from "$lib/components/Box.svelte";
    import LockingBox from "$lib/components/LockingBox.svelte";
    import { LockableCell } from "$lib/utils.svelte";

    let { data }: PageProps = $props();
</script>

<section>
    <div id="lsidebar">
        <Button action={() => goto("/", { replaceState: true })}>
            <Undo2></Undo2> Back to characters
        </Button>
        <Box width="100%" height="auto">
            <img alt="yeah nah" />
            <p></p>
        </Box>
        <LockingBox
            height="auto"
            width="100%"
            title="Stats"
            cell={LockableCell.StatsCell}
        >
            Yeah
        </LockingBox>
    </div>
    <div id="content"></div>
</section>

<style>
    :global(*) {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        font-size: 20px;
    }

    section {
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
    }
</style>
