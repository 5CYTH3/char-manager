<script lang="ts">
    let { action, _class, children } = $props();
</script>

<button class={_class} onclick={action}>
    {@render children()}
</button>

<style>
    button {
        border-radius: 20px;
        background-color: #f4f4f4;
    }
</style>
