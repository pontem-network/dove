import './lib.js';
import * as wasm from '../pkg/client.js';

const TEMPLATE_PROJECT_ELEMENT = `
    <div class="project noselect" data-id="{{id}}">
        <button class="project-title title">{{name}}</button>
    </div>
    `;
const TEMPLATE_EXPLORER_DIR = `
    <span class="dir-name name">
        <i><svg ><use xlink:href="#icon-arrow-bottom"></use></svg></i>
        <span>{{name}}</span>
    </span>
    <ul class="parent">
        <li class="empty">- empty -</li>
    </ul>
`;
const TEMPLATE_EXPLORER_FILE = `
    <span class="file-name name">
        <i><svg ><use xlink:href="#icon-file"></use></svg></i>
        <span>{{name}}</span>
    </span>
`;

/// initializing the sidebar
export async function init(){
    project_load();
    init_menu();

    // open projects list
    on_click_icon_panel(document.querySelectorAll("#navigation .ico-panel li button")[0]);
}
function footer_status(text){
    document.querySelector("#footer .status").innerHTML = text;
}
// ===============================================================
//  Menu
// ===============================================================
function init_menu(){
    document
        .querySelectorAll("#navigation .ico-panel li button:not(.i)")
        .forEach( button => {
            button
                .addClass('i')
                .addEventListener('click', function(e){
                    e.stopPropagation();
                    on_click_icon_panel(this);
                    return false;
                });
        });
}

function on_click_icon_panel(click_button){
    if(click_button.hasClass("open")){
        click_button.removeClass("open");
        document
            .getElementById(click_button.attr("child-panel"))
            .removeClass("open")
            .addClass('hide');
        return ;
    }

    click_button
        .parentElement
        .parentElement
        .querySelectorAll('button.open')
        .forEach( el => {
            el.removeClass("open");
        });
    click_button.addClass("open").removeClass("hide");

    document
        .querySelectorAll("#navigation .list-panel .container:not(.hide)")
        .forEach(el=>{ 
            el.removeClass("open").addClass("hide") 
        });

    document
        .getElementById(click_button.attr("child-panel"))
        .removeClass("hide")
        .addClass("open");
}
// ===============================================================
//  Projects
// ===============================================================
/// Display the found projects on the computer in the sidebar
async function project_load(){
    let projects_element = document
        .querySelector("#projects .cont")
        .addClass('load');

    footer_status("Loading projects...");

    let list = await wasm.project_list();

    if (projects_element === undefined ){ return ; }
    projects_element.innerHTML = "";

    list.projects.forEach( element => {
        let item = TEMPLATE_PROJECT_ELEMENT
            .replaceAll("{{id}}", element.id)
            .replaceAll("{{name}}", element.name);
        projects_element.insertAdjacentHTML('beforeend', item);
    });

    projects_element
        .querySelectorAll(".project:not(.i)")
        .forEach( project => {
            project
                .addClass('i')
                .addEventListener('click', on_click_project);
        });
    
    projects_element.removeClass('load');
    footer_status("Done");
}

/// Click on the project name in the sidebar
function on_click_project(){
    let id = this.attr('data-id');
    if ( !id ){
        console.warn('data-id is undefined');
        return false;
    }
    window.id_open_project = id;
    explorer_load(id);
}
// ===============================================================
//  Explorer
// ===============================================================
/// Upload a file tree
export async function explorer_load(id) {
    let explorer = document.querySelector("#explorer .cont");
    if ( explorer === undefined ){ return ; }
    explorer.addClass('load');
    footer_status("Loading tree")

    explorer.innerHTML = "";
    let info = await wasm.project_info(id);

    explorer_add(explorer, [info.tree]);

    // dir click
    explorer
        .querySelectorAll("li.dir:not(.i)")
        .forEach( dir => {
            dir.addClass("i").addEventListener('click', on_click_explorer_dir);
        });
    // file click
    explorer
        .querySelectorAll("li.file:not(.i)")
        .forEach( file => {
            file.addClass("i").addEventListener('click', on_click_explorer_file);
        });

    // open explorer panel
    on_click_icon_panel(document.querySelectorAll("#navigation .ico-panel li button")[1]);

    explorer.removeClass('load');
    footer_status("Done")
}

function on_click_explorer_dir(e){
    e.stopPropagation();

    this.toggleClass("open");
    return false;
}
function on_click_explorer_file(e){
    e.stopPropagation();

    footer_status("Loding file...");

    let config = {line_height: 18};
    wasm.open_file(window.id_open_project, this.attr("data-id"), "editor-container", config);

    footer_status("Done");
    return false;
}


function explorer_add(parent, data){
    if( !data || !data.length){ return ; }
    parent.innerHTML = "";

    data.forEach( element => {
        Object.keys(element).forEach( tp_element => {
            switch (tp_element) {
                case "Dir":
                    explorer_add_dir(parent, element[tp_element][0], element[tp_element][1]);
                    break;
                case "File":
                    explorer_add_file(parent, element[tp_element][0], element[tp_element][1]);
                    break;
                default:
                    console.warn("Unknown type: {" + tp_element + "}.");
                    console.log(element);
                    break;
            }
        });
    });
}

function explorer_add_dir(parent, name, data){
    let block = document.createElement("li").addClass("dir open"),
        chield_block;
    block.innerHTML = TEMPLATE_EXPLORER_DIR.replaceAll("{{name}}", name);
    chield_block = block.querySelector(".parent");
    parent.append(block);

    explorer_add(chield_block, data)
}

function explorer_add_file(parent, id, name){
    let block = document
        .createElement("li")
        .addClass("file")
        .attr("data-id", id);
    block.innerHTML = TEMPLATE_EXPLORER_FILE
        .replaceAll("{{name}}", name)
        .replaceAll("{{id}}", id);
    parent.append(block);
}