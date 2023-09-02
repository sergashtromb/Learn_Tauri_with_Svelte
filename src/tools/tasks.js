
export class TaskObj {
    id = -1;
    text = "";
    is_done = false;

    constructor(id, text="Новая задача", is_done=false) {
        this.id = id;
        this.text = text,
        this.is_done = is_done
    }

    set_atribute(text=this.text, is_done=this.is_done) {
        this.text = text,
        this.is_done = is_done
    }

}