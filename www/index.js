import * as wasm from "wasm-plugin";


window.addEventListener('load', () => {
  const res = wasm.hello("Ewan Valentine");

  const counter = new wasm.Counter();

  const onClick = document.getElementById('onClick');
  onClick.addEventListener('click', () => {
    counter.increment();
    console.log(counter.get_value());
  });
});

