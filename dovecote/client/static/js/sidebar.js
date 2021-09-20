import './lib.js';
import * as wasm from '../pkg/client.js';
import * as project from './project.js';
import * as cons from './console.js';

/// ID of the Open project
window.open_project = await project.create();

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
export async function init() {
    project_load();
    init_menu();
    inic_header_buttons();
    cons.inic_panel();

    // open projects list
    on_click_icon_panel(document.querySelectorAll("#navigation .ico-panel li button")[0]);
}
// ===============================================================
//  Menu
// ===============================================================
function init_menu() {
    document
        .querySelectorAll("#navigation .ico-panel li button:not(.i)")
        .forEach(button => {
            button
                .addClass('i')
                .addEventListener('click', function(e) {
                    e.stopPropagation();
                    on_click_icon_panel(this);
                    return false;
                });
        });
}

function on_click_icon_panel(click_button) {
    if (click_button.hasClass("open")) {
        click_button.removeClass("open");
        document
            .getElementById(click_button.attr("child-panel"))
            .removeClass("open")
            .addClass('hide');
        return;
    }

    click_button
        .parentElement
        .parentElement
        .querySelectorAll('button.open')
        .forEach(el => {
            el.removeClass("open");
        });
    click_button.addClass("open").removeClass("hide");

    document
        .querySelectorAll("#navigation .list-panel .container:not(.hide)")
        .forEach(el => {
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
async function project_load() {
    let projects_element = document
        .querySelector("#projects .cont")
        .addClass('load');

    cons.status("Loading projects...");

    let list = await wasm.project_list();

    if (projects_element === undefined) {
        return;
    }
    projects_element.innerHTML = "";

    list.projects.forEach(element => {
        let item = TEMPLATE_PROJECT_ELEMENT
            .replaceAll("{{id}}", element.id)
            .replaceAll("{{name}}", element.name);
        projects_element.insertAdjacentHTML('beforeend', item);
    });

    projects_element
        .querySelectorAll(".project:not(.i)")
        .forEach(project => {
            project
                .addClass('i')
                .addEventListener('click', on_click_project);
        });

    projects_element.removeClass('load');
    cons.status("Done");
}

/// Click on the project name in the sidebar
function on_click_project() {
    let id = this.attr('data-id');
    if (!id) {
        cons.warn('data-id is undefined');
        return false;
    }
    explorer_load(id);
}

// ===============================================================
//  Explorer
// ===============================================================
/// load a file tree
export async function explorer_load(id) {
    let explorer = document.querySelector("#explorer .cont");
    if (explorer === undefined) {
        return;
    }
    explorer.addClass('load');
    cons.status("Loading tree")

    explorer.innerHTML = "";
    let info = await wasm.project_info(id);
    if (window.open_project.destroy) {
        window.open_project.destroy();
    }
    window.open_project.set_project_id(id);

    explorer_add(explorer, "", [info.tree]);

    // dir click
    explorer
        .querySelectorAll("li.dir:not(.i)")
        .forEach(dir => {
            dir.addClass("i").addEventListener('click', on_click_explorer_dir);
        });
    // file click
    explorer
        .querySelectorAll("li.file:not(.i)")
        .forEach(file => {
            file.addClass("i").addEventListener('click', on_click_explorer_file);
        });

    // open explorer panel
    on_click_icon_panel(document.querySelectorAll("#navigation .ico-panel li button")[1]);

    explorer.removeClass('load');
    cons.status("Done")
}

function on_click_explorer_dir(e) {
    e.stopPropagation();

    this.toggleClass("open");
    return false;
}

function on_click_explorer_file(e) {
    e.stopPropagation();

    window
        .open_project
        .open_file(this.attr("data-id"), this.attr("data-name"));

    return false;
}

function explorer_add(parent_element, path, data) {
    if (!data || !data.length) {
        return;
    }
    parent_element.innerHTML = "";

    data.forEach(element => {
        Object.keys(element).forEach(tp_element => {
            switch (tp_element) {
                case "Dir":
                    explorer_add_dir(parent_element, path, element[tp_element][0], element[tp_element][1]);
                    break;
                case "File":
                    explorer_add_file(parent_element, path, element[tp_element][0], element[tp_element][1]);
                    break;
                default:
                    cons.warn("Unknown type: {" + tp_element + "}.");
                    cons.log(element);
                    break;
            }
        });
    });
}

function explorer_add_dir(parent, path, name, data) {
    path += name;
    let block = document.createElement("li").addClass("dir open").attr("path", path),
        chield_block;
    block.innerHTML = TEMPLATE_EXPLORER_DIR.replaceAll("{{name}}", name);
    chield_block = block.querySelector(".parent");
    parent.append(block);

    explorer_add(chield_block, path + "/", data)
}

function explorer_add_file(parent, path, id, name) {
    path += name;
    let block = document
        .createElement("li")
        .addClass("file")
        .attr("data-id", id)
        .attr("data-name", name)
        .attr("path", path);
    block.innerHTML = TEMPLATE_EXPLORER_FILE
        .replaceAll("{{name}}", name)
        .replaceAll("{{id}}", id);
    parent.append(block);
}
// ===============================================================
//  header buttons
// ===============================================================
function inic_header_buttons() {
    document
        .querySelector("#container .header button.build")
        .addEventListener("click", function(e) {
            e.stopPropagation();
            if (window.open_project.build) {
                window.open_project.build();
            }
        });
    document
        .querySelector("#container .header button.clean")
        .addEventListener("click", function(e) {
            e.stopPropagation();
            if (window.open_project.clean) {
                window.open_project.clean();
            }
        });
    document
        .querySelector("#container .header button.test")
        .addEventListener("click", function(e) {
            e.stopPropagation();
            if (window.open_project.test) {
                window.open_project.test();
            }
        });
    document
        .querySelector("#container .header button.check")
        .addEventListener("click", function(e) {
            e.stopPropagation();
            if (window.open_project.check) {
                window.open_project.check();
            }
        });
}