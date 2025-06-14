<script lang="ts">
    import { goto } from "$app/navigation";
    import Button from "$lib/components/Button.svelte";
    import { type PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

<section>
    <div id="lsidebar">
        <Button _class="btn" action={() => goto("/", { replaceState: true })}
            >Back to characters</Button
        >
    </div>
    <div id="content"></div>
</section>

<style>
    :global(body) {
        padding: 0;
        margin: 0;
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
        justify-content: center;
        align-items: start;
        background-color: red;
        height: 100%;
        width: 200px;
    }

    .btn {
        width: 100%;
        margin: 10px 10px 10px 10px;
    }
</style>
