let riveInstance

export function load_badger(){
  try {
    riveInstance = new rive.Rive({
        src: "/animations/badger.riv",
        canvas: document.getElementById("badger-canvas"),
        autoplay: true,
        artboard: "Artboard",
        stateMachines: "State Machine 1",
    });
    pass_mousemove_events()
  }catch(e){console.error(e)}
}

export function laser_eyes(){
  try {
    const inputs = riveInstance.stateMachineInputs('State Machine 1');
    const laserTrigger = inputs.find(i => i.name === 'anim_change');
    laserTrigger.fire()
  }catch(e){console.error(e)}
}

export async function hide_badger(val){
  try {
    await new Promise(res => setTimeout(res, 500));

    const inputs = riveInstance.stateMachineInputs('State Machine 1');
    if (inputs){
      const hideInput = inputs.find(i => i.name === 'Hide');
      hideInput.value = val
    }
  }catch(e){console.error(e)}
}

export async function hide_badger_timed(){
  try {
    await new Promise(res => setTimeout(res, 500));

    const inputs = riveInstance.stateMachineInputs('State Machine 1');
    if (inputs){
      const hideInput = inputs.find(i => i.name === 'Hide');
      hideInput.value = true

      await new Promise(res => setTimeout(res, 2000));
      hideInput.value = false
    }
  }catch(e){console.error(e)}
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