import init, { representation } from './wasm_xt.js';

async function initialize() {
    await init();

    const button = document.getElementById('buttonId');
    const input = document.getElementById('numberInput');
    const resultOutput = document.getElementById('resultOutput');

    button.addEventListener('click', () => {
        const inputValue = parseInt(input.value);
        const result = representation(inputValue);
        resultOutput.textContent = result;
    });
}

initialize();