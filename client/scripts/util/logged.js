let nel = document.createElement("div");
let logout = document.createElement("div");
nel.className = "col";
logout.className = "col";
lobtn = document.createElement("button");
lobtn.onclick = async () => {
    const res = await fetch("/api/destroysession", {method: "POST"});window.open("/", "_self");
};
lobtn.innerHTML = "Logout";
lobtn.className = "btn btn-primary";
logout.appendChild(lobtn);
document.getElementById("langselector").parentElement.parentElement.appendChild(nel);
document.getElementById("langselector").parentElement.parentElement.appendChild(logout);
fetch("/api/user_info", {
    method: "POST",
}).then(r => r.json())
.then(d => nel.innerHTML = "Logged in as "+d.username);