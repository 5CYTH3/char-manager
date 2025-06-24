<script lang="ts">
    let { action, children } = $props();
</script>

<button onclick={action}>
    {@render children()}
</button>

<style>
    button {
        all: unset;
        height: 57px;
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        border-radius: 20px;
        background-color: #f4f4f4;
    }
</style>
