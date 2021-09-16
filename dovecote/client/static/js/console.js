import './lib.js';

/// Set status text in footer
export async function status(text) {
    document.querySelector("#footer .status").innerHTML = text;
}

/// Initializing the console panel
export async function inic_panel() {
    document.querySelector("#footer .console").addEventListener("click", function() {
        this.toggleClass("active");
        document.getElementById("console").toggleClass("active");
    });

}
/// open the panel and display the output
export async function output(text) {
    let console_block = document.getElementById("console");
    if (!console_block.hasClass("active")) {
        console_block.addClass("active");
        document.querySelector("#footer .console").addClass("active");
    }
    let ansi_up = new window.AnsiUp;
    console_block.innerHTML = ansi_up.ansi_to_html(text);
}