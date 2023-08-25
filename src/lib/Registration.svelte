<!-- Компонент для окна входа и регистрации -->
<script>

import { invoke } from "@tauri-apps/api/tauri"
import { createEventDispatcher } from "svelte";

const dispatch = createEventDispatcher();

let try_name = "";
let try_pass = "";

async function get_user() {
    invoke("get_all_users");
    
}

</script>

<style>

#singing {

    margin: 0 auto;
    width: calc(100vw - 25vw);
    background-color: #3e3e3e;
    border-radius: 15px;
    top: calc(100vw - 85vw);
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    flex-wrap: wrap;
    padding: 10px 0px;
}

.users {
    width: calc(100vw - 35vw);
    margin: 5px 5px;
}

#buttons {
    width: calc(100vw - 35vw);
    padding: 5px 0px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.sg_but {
    display: inline-block;
}

</style>

<div id="singing">
    <input type="text" id="user_name" class="users" placeholder="Ник" bind:value={try_name}>
    <input type="password" id="user_password" class="users" placeholder="Пароль" bind:value={try_pass}>
    <div id="buttons">
      <button on:click={get_user} class="sg_but">Войти</button>
      <button class="sg_but">Создать аккаунт</button>
    </div>
    
</div>