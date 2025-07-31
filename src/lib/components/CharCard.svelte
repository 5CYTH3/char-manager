<script lang="ts">
    export let pic_url, name, _class, level;
</script>

<a class="card-container" href="/{name}">
    <img class="pic" src={pic_url} aria-hidden="true" alt="character picture" />
    <div class="textholder">
        <p class="name">{name}</p>
        <p class="infos">Level {level} {_class}</p>
    </div>
</a>

<style>
    .card-container {
        width: 30vw;
        max-width: 200px;
        min-height: max-content;
        padding: 40px 0 40px 0;

        flex-flow: column;
        flex-shrink: 0;
        display: flex;
        justify-content: center;
        align-items: center;

        background-color: #f4f4f4;

        border: 1px solid darkgray;
        border-radius: 20px;

        text-align: center;
        text-decoration: none;

        transition-duration: 0.1s;
    }

    .card-container:hover {
        box-shadow: 0 5px 5px lightgray;
    }

    .pic {
        width: 100px;
        height: 100px;
        max-width: 100px;
        max-height: 100px;
        object-fit: cover;
        border-radius: 50px;
    }

    .textholder {
        flex-direction: column;
        display: flex;
        justify-content: center;
        align-items: center;
        color: black;
        text-decoration: none;
    }
</style>
