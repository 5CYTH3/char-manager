<script lang="ts">
    let { unlocked, value } = $props();
</script>

{#if unlocked}
    <p>{value}</p>
{:else}
    <input type="text" {value} />
{/if}
