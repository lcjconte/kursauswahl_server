withIcon = {
    "DE": "DE🇩🇪",
    "IT": "IT🇮🇹",
    "EN": "EN🇬🇧"
}

function select_lang(nlang) {
    if (nlang == "IT") {
        alert("Language currently disabled");
        return;
    }
    lang = nlang;
    apply();
    document.getElementById("langselector").children[0].innerHTML = withIcon[nlang];
}