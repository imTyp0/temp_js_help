const form = document.forms["login"];
const login_button = form.children[4];

login_button.addEventListener("click", () => {
	const username = form.username.value;
	const password = form.password.value;

	console.log("before");
	console.log(`user: ${(username!="") ? username: "foo"}\npass: ${password ?? "bar"}`);
	console.log("after");
});