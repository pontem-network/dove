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

    document.addEventListener("keyup", function(e) {
        if (e.code === "Escape") {
            let console_block = document.getElementById("console");
            if (console_block.hasClass("active")) {
                console_block.removeClass("active");
                document.querySelector("#footer .console").removeClass("active");
            }
        }
    });
}

/// open the panel and display the output
export async function output(text) {
    let console_block = document.getElementById("console");
    if (!console_block.hasClass("active")) {
        console_block.addClass("active");
        document.querySelector("#footer .console").addClass("active");
    }
    text = output_preproc(text);

    let ansi_up = new window.AnsiUp;
    console_block.innerHTML = ansi_up.ansi_to_html(text)
        .replace(
            /\[path([^\]]*)\]([^\[]*)\[\/path\]/g,
            '<span class="open_file" $1>$2</span>'
        )
        .replaceAll("&quot;", '"');
    inic_output();
    console_block.focus();
}

function output_preproc(output) {
    let find, list = [],
        regex = new RegExp(/(?<source>(?<path>(\w+\/?)+\.move)(?<position>(\:\d+)+))/, 'g');

    while ((find = regex.exec(output)) !== null) {
        find = find.groups;
        let path = find.path,
            [_, line, char] = find.position.split(':'),
            file_id = document.querySelector('#explorer .file[path="' + path + '"]').attr("data-id");

        list.push({
            source: find.source,
            path,
            file_id,
            line,
            char
        });
    }
    list.filter((value, _, self) => {
            return self.find(el => el.source === value.source) === value;
        })
        .forEach(value => {
            output = output.replaceAll(
                value.source,
                '[path path="' + value.path + '" data-id="' + value.file_id + '" line="' + value.line + '" char="' + value.char + '"]' + value.source + "[/path]"
            );
        });

    return output;
}

function inic_output() {
    document.querySelectorAll("#console .open_file[path]:not(.click)")
        .forEach(path => {
            path.addClass("click")
                .addEventListener("click", function(e) {
                    e.stopPropagation();
                    document.querySelectorAll("#console, #footer .console")
                        .forEach(el => { el.removeClass("active") });
                    let menu = document.querySelector('#explorer .file[path="' + path.attr("path") + '"]');
                    if (!menu || !window.open_project.open_file) {
                        return;
                    }
                    window.open_project.open_file(
                        menu.attr("data-id"),
                        menu.attr("data-name"),
                        path.attr("line"),
                        path.attr("char")
                    );
                });
        });
}