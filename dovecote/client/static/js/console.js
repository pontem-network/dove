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

    console_block.innerHTML = ansi_up.ansi_to_html(text)
        .replace(
            /\[path([^\]]*)\]([^\[]*)\[\/path\]/g,
            '<span class="open_file" $1>$2</span>'
        )
        .replaceAll("&quot;", '"');
    inic_output();
}

function inic_output() {
    document.querySelectorAll("#console .open_file[path]:not(.click)")
        .forEach(path => {
            path.addClass("click")
                .addEventListener("click", function(e) {
                    e.stopPropagation();
                    document.getElementById("console").removeClass("active");
                    let menu = document.querySelector('#explorer .file[path="' + path.attr("path") + '"]');
                    if (!menu || !window.open_project.open_file) { return; }
                    window.open_project.open_file(
                        menu.attr("data-id"),
                        menu.attr("data-name"),
                        path.attr("line"),
                        path.attr("char")
                    );
                });
        });
}