import './lib.js';
import * as cons from './console.js';
import * as localapi from './localapi.js';

const TEMPLATE_TAB = `
    <div class="item" data-id="{{id}}">
        <span>{{name}}</span>
        <button type="button" class="close" title="close">+</button>
    </div>`;


/// Open the file in the tab
export async function open_file(project_id, file_id, file_name, line, char) {
    if (window.open_project.files[file_id] && window.open_project.files[file_id].editor) {
        // the tab is already there
        return;
    }
    cons.status("open file: " + file_name);
    let object = create_empty();
    object.project_id = project_id;
    object.file_id = file_id;
    object.file_name = file_name;

    // Create a tab and a block for the editor
    create_in_doom_tab_and_editor(object);
    // Create an editor
    create_editor(object, line, char);
    return object;
}

function create_empty() {
    return {
        project_id: null,
        file_id: null,
        file_name: null,
        tab: null,
        active: false,
        editor: {
            block: null,
            monaco: null
        },
        /// make the tab active
        set_active: function () {
            window
                .open_project
                .get_active_tabs()
                .forEach(tab => {
                    if (tab.file_id !== this.tab_id) {
                        tab.inactive()
                    }
                });

            if (this.active) {
                return this;
            }

            this.active = true;
            this.tab.addClass('active');
            this.editor.block.addClass('active');

            return this;
        },
        set_position: function (line, char) {
            if (!line) {
                return this;
            }
            line *= 1;
            char *= 1;
            this.editor.monaco.focus();
            this.editor.monaco.setPosition({
                lineNumber: line,
                column: char
            });
            this.editor.monaco.revealPositionInCenter({
                lineNumber: line,
                column: char
            });

            return this;
        },
        /// make the tab inactive
        inactive: function () {
            this.tab.removeClass('active');
            this.editor.block.removeClass('active');
            this.active = false;
            this.onblur();
        },
        /// loss of focus
        onblur: async function () {
            // ...
        },
        /// close the tab
        destroy: function () {
            this.onblur();
            // distroy editor
            if (this.editor.monaco) { this.editor.monaco.dispose(); }
            this.editor.block.remove();
            this.tab.remove();
        }
    };
}

/// Create a tab and a block for the editor
function create_in_doom_tab_and_editor(object) {
    let tab_id = "tab_" + object.file_id;
    // tab
    document
        .querySelector("#code-space .tabs-head")
        .insertAdjacentHTML(
            'afterbegin',
            TEMPLATE_TAB
                .replaceAll("{{name}}", object.file_name)
                .replaceAll("{{id}}", tab_id)
        );
    object.tab = document.querySelector("#code-space .tabs-head .item[data-id=\"" + tab_id + "\"]");
    // Block for the editor
    object.editor.block = document
        .createElement("div")
        .addClass("scroll item")
        .attr("id", tab_id);
    document
        .querySelector("#code-space .tabs-body")
        .append(object.editor.block);

    // event set active
    object.tab.addEventListener("click", function (e) {
        e.stopPropagation();
        if (this.hasClass('active')) {
            return;
        }
        object.set_active();
    })
    object.tab
        .querySelector(".close")
        .addEventListener('click', function (e) {
            e.stopPropagation();
            window.open_project.close_file(object.file_id);
        });
}

function create_editor(object, line, char) {
    // load content and type
    cons.status("loading a file: " + object.file_name);
    localapi
        .get_file(object.project_id, object.file_id)
        .then(file => {
            object.editor.monaco = monaco.editor
                .create(object.editor.block, {
                    value: null,
                    language: null,
                    theme: "vs-dark",
                    automaticLayout: true
                });
            // loss of editor focus
            object.editor.monaco
                .onDidBlurEditorText(_ => {
                    object.onblur();
                });
            object.editor.monaco.setValue(file.content);

            switch (file.tp) {
                case "move":
                case "toml":
                    monaco.editor.setModelLanguage(object.editor.monaco.getModel(), file.tp);
                    console.log(file.tp + "Theme");
                    monaco
                        .editor
                        .setTheme(file.tp + "Theme");
                    break;
                default:
                    console.log("default")
                    monaco.editor.setModelLanguage(object.editor.monaco.getModel(), "text/plain");
                    monaco
                        .editor
                        .setTheme("vs-dark");
                    break;
            }
            // Changes in the text
            object.editor.monaco
                .getModel()
                .onDidChangeContent(async (event) => {
                    file.content = object.editor.monaco.getValue();
                    localapi.save_file(file);
                });
            object.set_position(line, char);
            cons.status("Done");
        }, error => {
            console.warn(error)
        });

    return object.set_active();
}