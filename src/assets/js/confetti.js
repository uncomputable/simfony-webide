
export function start_confetti(){
    console.log("confetti")

    let confetti_div = document.querySelector(".run-button button")
    confetti_div.classList.remove('animate');
    confetti_div.classList.add('animate');
    setTimeout(function(){
      confetti_div.classList.remove('animate');
    },700);

    flash_screen()
};

function flash_screen(){
    document.body.classList.remove("animate-body-flash");
    document.body.classList.add("animate-body-flash");

    setTimeout(function(){
        document.body.classList.remove("animate-body-flash");
      },1000);
}
