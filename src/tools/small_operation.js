

export function get_last_id(tasks) {
    let id = 0;

    for(let elem of tasks) {
        if (elem.id > id) {
            id = elem.id;
        }
    }

    return id;

}