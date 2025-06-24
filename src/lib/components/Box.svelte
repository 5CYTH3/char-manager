<script lang="ts">
    let { children, width, height } = $props();
</script>

<div
    style="width: {width}; height: {height};
    background-color: #f4f4f4;
    border-radius: 20px;
    padding: 20px;"
>
    {@render children()}
</div>

<style>
    div {
        background-color: #f4f4f4;
        border-radius: 20px;
        padding: 20px;
    }
</style>
