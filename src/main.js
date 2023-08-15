// exit app when any key is pressed
const { invoke } = window.__TAURI__.tauri;

function toggleExit() {
	invoke("toggle_exit");
}

window.addEventListener('keydown', toggleExit);
window.addEventListener('click', toggleExit);