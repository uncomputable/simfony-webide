export function copy_program(text){
    navigator.clipboard.writeText(text);
    copy_success()
}

function copy_success(){
    let button = document.querySelector("#copy-button-success")
    if (!button.classList.contains("copy-show")){
        button.classList.add("copy-show");
        setTimeout(() => button.classList.remove("copy-show"), 1000)
    }
}	