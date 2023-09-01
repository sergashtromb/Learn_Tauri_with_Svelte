<script>
    import { invoke } from "@tauri-apps/api/tauri";

</script>

<style>

#tools {
    height: 100vw;
    width: calc(100vw - 85vw);
    min-width: 160px;
    position: fixed;
    z-index: 1;
    top: 0; 
    left: 0;
    overflow-x: hidden;
    padding-top: 20px;
    background-color: #2f2f2f;
    border-right: 3px solid rgb(29, 29, 29);
}

#tools button {
    padding: 6px 8px 6px 16px;
    display: block;
}
</style>

<div id="tools">

    <button on:click={() => {
        invoke("get_runtime");
    }}>Получить все задачи с Базы данных</button>
    <button>Ипорт задач из .md файлов</button>
    
</div>