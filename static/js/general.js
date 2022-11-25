const sidebar = document.body.querySelector('nav');
const toggle = document.body.querySelector(".toggle");
const searchBtn = document.body.querySelector(".search-box");
const modeSwitch = document.body.querySelector(".toggle-switch");
const modeText = document.body.querySelector(".mode-text");

const toggle_theme = () =>{
	["light", "dark"].forEach(cl => document.body.classList.toggle(cl, cl === theme));
	modeText.textContent = `${theme[0].toUpperCase()}${theme.slice(1)} mode`
}

let theme = localStorage.getItem("theme");
if (!theme) localStorage.setItem("theme", "light");
else toggle_theme();

modeSwitch.addEventListener("click" , () =>{
	theme = theme === "dark" ? "light" : "dark";
	localStorage.setItem("theme", theme);
	toggle_theme();
}, {passive: true});

toggle.addEventListener("click" , () =>{
	sidebar.classList.toggle("close");
})

searchBtn.addEventListener("click" , () =>{
	sidebar.classList.remove("close");
})