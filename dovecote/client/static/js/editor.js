import './lib.js';
import * as wasm from '../pkg/client.js';

const TEMPLATE_TAB = `
    <div class="item active" data-id="{{id}}">
        <span>{{name}}</span>
        <button type="button" class="close" title="close">+</button>
    </div>`;

/// Open the file in the tab
export async function open_file(project_id, file_id, file_name) {
    if( document.querySelector("#code-space .tabs-head .item[data-id=\"tab_" + file_id + "\"]")) {
        return;
    }
    remove_active();

    // insert tab
    document
        .querySelector("#code-space .tabs-head")
        .insertAdjacentHTML(
                'afterbegin', 
                TEMPLATE_TAB
                    .replaceAll("{{name}}", file_name)
                    .replaceAll("{{id}}", "tab_" + file_id)
            );
    let tab_block = document.querySelector("#code-space .tabs-head .item.active"),
        editor_block = document
            .createElement("div")
            .addClass("scroll item active")
            .attr("id", "tab_" + file_id);
    document
        .querySelector("#code-space .tabs-body")
        .append(editor_block);
    // event set active
    tab_block.addEventListener("click", function(e) {
        e.stopPropagation();
        if( this.hasClass('active')) { return ; }
        remove_active();
        tab_block.addClass('active');
        editor_block.addClass("active")
    })
    tab_block
        .querySelector(".close")
        .addEventListener('click', function(e) {
            e.stopPropagation();
            window.open_project.close_file(file_id);
        });

    // new editor
    let monaco = inic_editor(project_id, file_id, editor_block)

    return {
        projec_id:project_id,
        file_id:file_id,
        file_name:file_name,
        tab:tab_block,
        editor:{
            block: editor_block,
            monaco: monaco
        },
        /// make the tab active
        active:function() {
            if(this.tab.hasClass('active')) {
                return this;
            }
            remove_active();
            this.tab.addClass('active');
            this.editor.block.addClass('active');

            return this;
        },
        /// close the tab
        destroy:function() {
            // distroy editor
            this.editor.monaco.dispose();
            this.tab.remove();
            this.editor.block.remove();
            
            // set the following tab to active
            if( !document.querySelector("#code-space .tabs-head .item.active")
            || !document.querySelector("#code-space .tabs-body .item.active")) {
                let tabs = document.querySelectorAll("#code-space .tabs-head .item"), 
                    body = document.querySelectorAll("#code-space .tabs-body .item");
                if(tabs.length && body.length) {
                    remove_active();
                    tabs[0].addClass('active');
                    body[0].addClass('active');
                }
            }
        }
    }
}
/// all tabs are inactive
function remove_active() {
    document
        .querySelectorAll("#code-space .tabs-head .item.active, #code-space .tabs-body .item.active")
        .forEach(element => {
            element.removeClass('active');
        });
}
function inic_editor(project_id, file_id, block) {
    let editor = monaco
        .editor
        .create(block, {
            value: null,
            language: null,
            theme: "vs-dark",
            automaticLayout: true
        });
    editor.addAction({
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
        run: function (ed) {
            // @todo dove build
            return null;
        }
    });
    editor
        .getModel()
        .onDidChangeContent(async (event) => {
            await wasm.on_file_change(
                project_id, 
                file_id, 
                event
           );
        });
    // load content and type
    wasm.get_file(project_id, file_id)
        .then(file => {
            editor.setValue(file.content);
            monaco.editor.setModelLanguage(editor.getModel(), file.tp);
        });
    
    return editor;
}
