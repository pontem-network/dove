import './lib.js';
import * as wasm from '../pkg/client.js';

const TEMPLATE_TAB = `
    <div class="item" data-id="{{id}}">
        <span>{{name}}</span>
        <button type="button" class="close" title="close">+</button>
    </div>`;


/// Open the file in the tab
export async function open_file(project_id, file_id, file_name) {
    if (window.open_project.files[file_id] && window.open_project.files[file_id].editor) {
        // the tab is already there
        return;
    }

    let object = create_empty();
    object.projec_id = project_id;
    object.file_id = file_id;
    object.file_name = file_name;

    // Create a tab and a block for the editor
    create_in_doom_tab_and_editor(object);
    // Create an editor
    create_editor(object);
    // set active tab
    return object.set_active();
}

function create_empty() {
    return {
        projec_id: null,
        file_id: null,
        file_name: null,
        tab: null,
        active: false,
        editor: {
            block: null,
            monaco: null
        },
        /// make the tab active
        set_active: function() {
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
        /// make the tab inactive
        inactive: function() {
            this.tab.removeClass('active');
            this.editor.block.removeClass('active');
            this.active = false;
            this.onblur();
        },
        /// loss of focus
        onblur: function() {
            /// @todo onblur. Need to add autosave
            console.log("onblur: project[" + this.projec_id + "], file[" + this.file_id + "]");
        },
        /// close the tab
        destroy: function() {
            this.onblur();
            // distroy editor
            this.editor.monaco.dispose();
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
    object.tab.addEventListener("click", function(e) {
        e.stopPropagation();
        if (this.hasClass('active')) { return; }
        object.set_active();
    })
    object.tab
        .querySelector(".close")
        .addEventListener('click', function(e) {
            e.stopPropagation();
            window.open_project.close_file(object.file_id);
        });
}

function create_editor(object) {
    object.editor.monaco = monaco.editor
        .create(object.editor.block, {
            value: null,
            language: null,
            theme: "vs-dark",
            automaticLayout: true
        });
    object.editor.monaco
        .addAction({
            id: 'dove-build',
            label: 'Build project',
            keybindings: [
                monaco.KeyMod.CtrlCmd | monaco.KeyCode.F10,
                monaco.KeyMod.chord(
                    monaco.KeyMod.CtrlCmd | monaco.KeyCode.KEY_K,
                    monaco.KeyMod.CtrlCmd | monaco.KeyCode.KEY_M
                )
            ],
            precondition: null,
            keybindingContext: null,
            contextMenuGroupId: 'navigation',
            contextMenuOrder: 1.5,
            run: function(ed) {
                // @todo dove build
                return null;
            }
        });
    // Changes in the text
    object.editor.monaco
        .getModel()
        .onDidChangeContent(async(event) => {
            await wasm.on_file_change(
                object.projec_id,
                object.file_id,
                event
            );
        });
    // loss of editor focus
    object.editor.monaco
        .onDidBlurEditorText(_ => {
            object.onblur();
        });
    // load content and type
    wasm.get_file(object.projec_id, object.file_id)
        .then(file => {
            object.editor.monaco.setValue(file.content);
            monaco.editor.setModelLanguage(object.editor.monaco.getModel(), file.tp);
        });

    return object;
}