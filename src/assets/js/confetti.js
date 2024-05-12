
export function start_confetti(){
    console.log("confetti")

    let confetti_div = document.querySelector(".run-button button")
    confetti_div.classList.remove('animate');
    confetti_div.classList.add('animate');
    setTimeout(function(){
      confetti_div.classList.remove('animate');
    },700);
};
