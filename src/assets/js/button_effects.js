export function button_success_animation(){
  bubbles('success')
  flash_screen('success')
  circle_expand('success')
};

export function button_fail_animation(){
  bubbles('fail')
  flash_screen('fail')
  circle_expand('fail')
};

async function bubbles(mode){
  let button = document.querySelector(".run-button button")

  if (mode == 'success'){
    button.classList.remove('bubble-animation-green');
    button.classList.add('bubble-animation-green');
    await new Promise(res => setTimeout(res, 1000))
    button.classList.remove('bubble-animation-green');
  }
  else if (mode == 'fail'){
    button.classList.remove('bubble-animation-red');
    button.classList.add('bubble-animation-red');
    await new Promise(res => setTimeout(res, 1000))
    button.classList.remove('bubble-animation-red');
  }
}

async function flash_screen(mode){
  let flash_class = mode == 'success' ? "flash-success" : "flash-fail"
  document.body.classList.add(flash_class);
  await new Promise(res => setTimeout(res, 400))

  document.body.classList.remove(flash_class);
}

async function circle_expand(mode){
  let button = document.querySelector(".run-button button")
  if (button.classList.contains('green') || button.classList.contains('red')) return
  clear_circle_classes(button)

  if (mode == 'success')
    button.classList.add("green");
  else if (mode == 'fail')
    button.classList.add("red");

  button.classList.add("expand_start");
  await new Promise(res => setTimeout(res, 1000))

  button.classList.remove("expand_start");
  button.classList.add("button_color");
  await new Promise(res => setTimeout(res, 10))

  button.classList.add("expand_end");
  await new Promise(res => setTimeout(res, 400))

  clear_circle_classes(button)
}

function clear_circle_classes(button){
  button.classList.remove("expand_start");
  button.classList.remove("expand_end");
  button.classList.remove("button_color");
  button.classList.remove("green");
  button.classList.remove("red");
}