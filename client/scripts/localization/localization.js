let gtrl = {};
let lang = "EN";
function load(trl) {
    gtrl = trl;
    let urlParams = new URLSearchParams(window.location.search);
    if (!urlParams.has("lang")){
        insertUrlParam("lang", "EN");
        urlParams = new URLSearchParams(window.location.search);
    }
    lang = urlParams.get('lang');
}
function insertUrlParam(key, value) {
    if (history.pushState) {
        let searchParams = new URLSearchParams(window.location.search);
        searchParams.set(key, value);
        let newurl = window.location.protocol + "//" + window.location.host + window.location.pathname + '?' + searchParams.toString();
        window.history.pushState({path: newurl}, '', newurl);
    }
}

function apply() {
    if (!(lang in gtrl)) {
        console.error("Lang not found!");
        return;
    }
    insertUrlParam("lang", lang);
    var all = document.getElementsByTagName("*");

    for (var i=0, max=all.length; i < max; i++) {
        if (all[i].hasAttribute("trlKey")) {
            if (all[i].getAttribute("trlKey") in gtrl[lang]) {
                all[i].innerHTML = gtrl[lang][all[i].getAttribute("trlKey")];
            }
            else {
                all[i].innerHTML = "Not found";
            }
        }
    }
}

function getKey(key) {
    if (key in gtrl[lang]) {
        return gtrl[lang][key];
    }
    else {
        return "Not found";
    }
}