

export function get_last_id(tasks) {
    let id = 0;

    for(let elem of tasks) {
        if (elem.id > id) {
            id = elem.id;
        }
    }

    return id;

}

export function change_disabled_button(names, disabled, type) {

    if(type === 'Id') {
        // @ts-ignore
        document.getElementById(names).disabled = disabled;
    } else if (type === 'Class') {
        // @ts-ignore
        let arr = document.getElementsByClassName(names);
        for(let i = 0; i < arr.length; i++) {
            // @ts-ignore
            arr[i].disabled = disabled;
        }
    }
    
}