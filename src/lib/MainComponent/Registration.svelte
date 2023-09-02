<!-- Компонент для окна входа и регистрации -->
<script>

import { invoke } from "@tauri-apps/api/tauri"
import { createEventDispatcher } from "svelte";
import { change_disabled_button } from "../../tools/small_operation";


const dispatch = createEventDispatcher();

let try_name = "";
let try_pass = "";



function get_user() {
    
    change_disabled_button("sg_but", true, "Class");

    invoke("check_user_by_all", {
        userName: try_name.trim(),
        userPassword: try_pass
    }
    ).then((result) => {
        const user = JSON.parse(result);

        if (user.id === -1) {
            

            change_disabled_button("sg_but", false, "Class");

            return;
        }
        
        dispatch("log_in", {user: user});

    }).catch((err) => console.log(err));
    
}

function create_user() {

    change_disabled_button("sg_but", true, "Class");

    invoke("check_user_by_name", {
        userName: try_name.trim()
    }).then((result) => {
        const user = JSON.parse(result);

        if(user.id !== -1) {
            change_disabled_button("sg_but", false, "Class");
            return;
        }

        invoke("registration_user", {
            userName: try_name.trim(),
            userPassword: try_pass
        }).then((result) => {

            const new_user = JSON.parse(result);

            dispatch("log_in", {user: new_user});
            
        }).catch((err) => console.log(err));

    }).catch((err) => console.log(err));


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
      <button on:click={get_user} class="sg_but" id="sg_sub">Войти</button>
      <button on:click={create_user} class="sg_but" id="sg_reg">Создать аккаунт</button>
    </div>
    
</div>