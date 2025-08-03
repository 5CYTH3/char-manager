<script lang="ts">
    import { getContext } from "svelte";

    let { value } = $props();
    let unlocked: () => boolean = getContext("locking-box");
</script>

{#if unlocked()}
    <input type="text" {value} />
{:else}
    <p>{value}</p>
{/if}
