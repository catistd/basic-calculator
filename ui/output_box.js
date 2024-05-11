const { invoke } = window.__TAURI__.tauri

let main_string = "";
document.getElementById("equation").textContent = "0";

function character_add(c) {
    if (main_string.length > 1 && main_string[0] == 'e') {
        main_string = "";
    } 
    main_string += c;

    document.getElementById("equation").textContent = main_string;
}

function clearInput() {
    main_string = "";
    document.getElementById("equation").textContent = "0";
}

function enter() {
    if (main_string[0] != 'e') {
        invoke('calculate', { equation: main_string })
            .then((response) => {
            main_string = response
            document.getElementById("equation").textContent = response;
            });
    } 
}