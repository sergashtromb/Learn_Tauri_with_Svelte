<script>

    import { createEventDispatcher } from "svelte";

    let dispatch = createEventDispatcher();


    let text = "";

</script>

<style>

</style>

<div>
    <h2>Редактирование задачи</h2>
    <hr />
    <input type="text" bind:value={text}>
</div>