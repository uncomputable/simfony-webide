export function button_success_animation(){
  bubbles()
  flash_screen()
  circle_expand()
};

async function bubbles(){
  let button = document.querySelector(".run-button button")
  button.classList.remove('bubble-animation');
  button.classList.add('bubble-animation');
  await new Promise(res => setTimeout(res, 1000))

  button.classList.remove('bubble-animation');
}

 async function flash_screen(){
    document.body.classList.remove("animate-body-flash");
    document.body.classList.add("animate-body-flash");
    await new Promise(res => setTimeout(res, 1000))

    document.body.classList.remove("animate-body-flash");
}

async function circle_expand(){
  let button = document.querySelector(".run-button button")
  button.classList.remove("expand_start");
  button.classList.remove("expand_end");
  button.classList.remove("green_button");

  button.classList.add("expand_start");
  await new Promise(res => setTimeout(res, 1000))

  button.classList.remove("expand_start");
  button.classList.add("green_button");
  await new Promise(res => setTimeout(res, 1))

  button.classList.add("expand_end");
  await new Promise(res => setTimeout(res, 200))

  button.classList.remove("expand_end");
  button.classList.remove("green_button");
}