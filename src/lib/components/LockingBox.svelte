<script lang="ts">
    import { Lock, LockOpen } from "@lucide/svelte";
    import { LockableCell, lockingMutex } from "$lib/utils.svelte";
    import { setContext } from "svelte";

    let { title = undefined, children, width, height, cell } = $props(); // Cell is utils/LockableCell to apply mutex-like behaviour to locking and unlocking

    let unlocked = $derived(cell === lockingMutex.value);

    setContext("locking-box", () => unlocked); // Needs to be lazily evaluated or error https://svelte.dev/docs/svelte/compiler-warnings#state_referenced_locally
</script>

<div class="container" style="width: {width}; height: {height};">
    {#if title}
        <div>
            <div
                style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;"
            >
                <div></div>
                <p>{title}</p>
                <button
                    onclick={() =>
                        unlocked
                            ? (lockingMutex.value = LockableCell.None)
                            : (lockingMutex.value = cell)}
                >
                    {#if unlocked}
                        <LockOpen></LockOpen>
                    {:else}
                        <Lock></Lock>
                    {/if}
                </button>
            </div>
            <div class="divider"></div>
        </div>
    {/if}
    {@render children({ unlocked })}
</div>

<style>
    .container {
        background-color: #f4f4f4;
        border-radius: 20px;
        padding: 20px;
    }

    .divider {
        background-color: black;
        height: 2px;
        width: 100%;
    }
</style>
