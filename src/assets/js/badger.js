let riveInstance

export function load_badger(){
  riveInstance = new rive.Rive({
      src: "/animations/badger.riv",
      canvas: document.getElementById("badger-canvas"),
      autoplay: true,
      artboard: "Artboard",
      stateMachines: "State Machine 1",
  });
  pass_mousemove_events()

}

export function lazer_eyes(){
  const inputs = riveInstance.stateMachineInputs('State Machine 1');
  const lazerTrigger = inputs.find(i => i.name === 'anim_change');
  lazerTrigger.fire()
}

export async function hide_badger(val){
  await new Promise(res => setTimeout(res, 500));

  const inputs = riveInstance.stateMachineInputs('State Machine 1');
  if (inputs){
    const hideInput = inputs.find(i => i.name === 'Hide');
    hideInput.value = val
  }
}

export async function hide_badger_timed(){
  await new Promise(res => setTimeout(res, 500));

  const inputs = riveInstance.stateMachineInputs('State Machine 1');
  if (inputs){
    const hideInput = inputs.find(i => i.name === 'Hide');
    hideInput.value = true

    await new Promise(res => setTimeout(res, 2000));
    hideInput.value = false
  }
}

// the input field captures mouse events so a new event 
// needs to be created for the badgers eyes to follow the pointer.
function pass_mousemove_events(){
  const inputElement = document.querySelector('.program-input-field');
  const badgerCanvas = document.querySelector('#badger-canvas');

  inputElement.addEventListener('mousemove', function(event) {
      const newEvent = new MouseEvent('mousemove', {
          clientX: event.clientX,
          clientY: event.clientY,
          screenX: event.screenX,
          screenY: event.screenY,
          bubbles: true,
          cancelable: true,
          view: window
      });
      badgerCanvas.dispatchEvent(newEvent);
  });
}