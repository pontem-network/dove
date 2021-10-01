import './lib.js';
import * as localapi from './localapi.js';
import * as project from './project.js';
import * as cons from './console.js';

const TEMPLATE_PROJECT_ELEMENT = `
    <div class="project noselect" data-id="{{id}}">
        <button type="button" class="project-title title">{{name}}</button>
        <button type="button" class="project-remove" title="Delete a project">-</button>
    </div>
    `;
const TEMPLATE_EXPLORER_DIR = `
    <span class="dir-name name">
        <i class="type-icon"><svg ><use xlink:href="#icon-arrow-bottom"></use></svg></i>
        <span>{{name}}</span>
        <div class="actions">
            <button class="add" title="Add a file"><svg ><use xlink:href="#icon-add"></use></svg></button>
            <button class="rename" title="rename a file"><svg ><use xlink:href="#icon-rename"></use></svg></button>
            <button class="remove" title="remove a file"><svg ><use xlink:href="#icon-trash"></use></svg></button>
        </div>
    </span>
    <ul class="parent">
        <li class="empty">- empty -</li>
    </ul>
`;
const TEMPLATE_EXPLORER_FILE = `
    <span class="file-name name">
        <i class="type-icon"><svg ><use xlink:href="#icon-file"></use></svg></i>
        <span>{{name}}</span>
        <div class="actions">
        <button class="rename" title="rename a file"><svg ><use xlink:href="#icon-rename"></use></svg></button>
        <button class="remove" title="remove a file"><svg ><use xlink:href="#icon-trash"></use></svg></button>
        </div>
    </span>
`;
const TEMPLATE_EXPLORER_NAME_POPUP = `
    <div class="new_name">
        <input title="Enter name" placeholder="Enter name..."/>
    </div>
`;

const TEMPLATE_EXPLORER_CHOOSE_DIALECT_POPUP = `
    <div class="new_name">
        <select title="Choose a dialect" >
            <option selected>Choose a dialect..</option>
            <option>diem</option>
            <option>dfinance</option>
            <option>pont</option>
        </select>
    </div>
`;

const TEMPLATE_RUN_COMMAND_EMTPY = `<li class="empty">- empty -</li>`;
const TEMPLATE_RUN_COMMAND_ITEM = `
<li class="item" data-id="{{id}}">
    <button class="run" title="Run the command" >{{command}}</button>
    <button class="remove" title="Delete the command">x</button>
</li>
`;

/// initializing the sidebar
export async function init() {
    /// ID of the Open project
    window.open_project = await project.create();

    init_menu();
    inic_header_buttons();
    inic_run_commands();
    await project_load();
    await cons.inic_panel();

    // Add a project
    document
        .querySelector("#projects-container .head .add_project:not(.i)")
        .addClass("i")
        .addEventListener("click", on_add_project);
    // open projects list
    on_click_icon_panel(document.querySelectorAll("#navigation .ico-panel li button")[0]);

    // displaying hints
    document.addEventListener("keydown", function(e) {
        if (e.key === "Control") {
            document
                .querySelectorAll("#navigation .ico-panel li button .keyhelp, #container .header button .keyhelp")
                .forEach(el => {
                    el.addClass("show");
                });
        }
    });
    document.addEventListener("keyup", function(e) {
        if (e.key === "Control") {
            document
                .querySelectorAll("#navigation .ico-panel li button .keyhelp, #container .header button .keyhelp")
                .forEach(el => {
                    el.removeClass("show");
                });
        }
    });
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

    document.addEventListener("keyup", function(e) {
        if (e.ctrlKey && (e.key === "1" || e.key === "2" || e.key === "3")) {
            let button = document.querySelectorAll("#navigation .ico-panel li button")[e.key - 1];
            if (button.hasClass("hide")) { return; }
            on_click_icon_panel(button);
        }
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
    cons.status("Loading projects...");
    await localapi.project_list()
        .then(
            list => {
                let projects_element = document.querySelector("#projects .cont");
                if (projects_element === undefined) {
                    return;
                }
                projects_element.innerHTML = "";

                if (!list) {
                    cons.status("Done");
                    return;
                }
                list.forEach(element => {
                    let item = TEMPLATE_PROJECT_ELEMENT
                        .replaceAll("{{id}}", element.id)
                        .replaceAll("{{name}}", element.name);
                    projects_element.insertAdjacentHTML('beforeend', item);
                });
                // open a project
                projects_element
                    .querySelectorAll(".project:not(.i)")
                    .forEach(project => {
                        project
                            .addClass('i')
                            .addEventListener('click', on_click_project);
                    });
                // delete a project
                projects_element
                    .querySelectorAll(".project .project-remove:not(.i)")
                    .forEach(project => {
                        project
                            .addClass('i')
                            .addEventListener('click', on_click_project_remove);
                    });
                cons.status("Done");
            },
            error => {
                console.error(error);
                cons.status("Error");
            }
        );
}


/// Click on the project name in the sidebar
function on_click_project(e) {
    e.stopPropagation();
    let id = this.attr('data-id');
    if (!id) {
        cons.warn('data-id is undefined');
        return false;
    }
    explorer_load(id);
    run_command_show_history(id);
}

function on_click_project_remove(e) {
    e.stopPropagation();
    cons.status("The project is being deleted");
    localapi.remove_project(this.parentNode.attr("data-id"))
        .then(_ => {
            project_load();
        }, error => {
            cons.status("Error");
            console.warn(error);
        });
}

function on_add_project(e) {
    e.stopPropagation();
    let main_block = this.parentByClass("head"),
        project_name = "";

    cons.status("Please enter the project name");
    entering_name(main_block, "")
        .then(
            name => {
                cons.status("Choose a dialect");
                project_name = name;
                return select_dialect(main_block);
            }
        )
        .then(
            dialect => {
                cons.status("Creating project");
                return localapi.create_project(project_name, dialect);
            }
        )
        .then(
            _ => {
                project_load();
            },
            error => {
                cons.status("Error");
                console.error(error);
            }
        );

    return false;
}

// ===============================================================
//  Explorer
// ===============================================================
/// load a file tree
export async function explorer_load(project_id) {
    cons.status("Loading tree")
    if (window.open_project.destroy) {
        window.open_project.destroy();
    }
    window.open_project.set_project_id(project_id);

    localapi.project_tree(project_id)
        .then(list => {
            explorer_set(list);
            cons.status("Done")
        }, error => {
            cons.status("Error: Failed to get information")
            console.warn(error);
        });
}

async function explorer_set(list) {
    let explorer = document.querySelector("#explorer .cont");
    if (explorer === undefined) {
        return;
    }
    explorer.innerHTML = "";
    explorer_add(explorer, "", [list]);

    let button = explorer
        .querySelector(".dir .actions button.rename");
    if (button) {
        button.remove()
    }
    button = explorer
        .querySelector(".dir .actions button.remove");
    if (button) {
        button.remove()
    }

    // dir click
    explorer
        .querySelectorAll("li.dir:not(.i)")
        .forEach(dir => {
            dir.addClass("i")
                .addEventListener('click', on_click_explorer_dir);
            dir.querySelector(".dir-name .actions button.add:not(.i)")
                .addEventListener("click", on_click_explorer_dir_add);
            dir.querySelectorAll(".dir-name .actions button.rename:not(.i)").forEach(el => {
                el.addEventListener("click", on_click_explorer_dir_rename);
            });
            dir.querySelectorAll(".dir-name .actions button.remove:not(.i)").forEach(el => {
                el.addEventListener("click", on_click_explorer_dir_remove);
            });
        });
    // file click
    explorer
        .querySelectorAll("li.file:not(.i)")
        .forEach(file => {
            file.addClass("i").addEventListener('click', on_click_explorer_file);
            file.querySelectorAll("button.rename:not(.i)").forEach(el => {
                el.addEventListener("click", on_click_explorer_file_rename);
            });
            file.querySelectorAll("button.remove:not(.i)").forEach(el => {
                el.addEventListener("click", on_click_explorer_file_remove);
            })
        });

    // open explorer panel
    if (!document.querySelector("#explorer-container").hasClass("open")) {
        on_click_icon_panel(document.querySelectorAll("#navigation .ico-panel li button")[1]);
    }

    for (let file_id in window.open_project.files) {
        if (!explorer.querySelector('.file[data-id="' + file_id + '"]')) {
            window.open_project.close_file(file_id);
        }
    }
    // show "run script"
    document.querySelectorAll("#navigation .ico-panel li button")[2].removeClass("hide");
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
                    console.warn("Unknown type: {" + tp_element + "}.");
                    console.log(element);
                    cons.status("Error");
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

    explorer_add(chield_block, path + "/", data);
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

function on_click_explorer_dir_add(e) {
    e.stopPropagation();
    let main_block = this.parentByClass("dir"),
        id = window.open_project.id,
        path = main_block.attr("path").split("/").slice(1).join("/");

    cons.status("Please enter the name of the new folder|file");
    explorer_entering_name(main_block)
        .then(
            name => {
                if (name.match(/(\.move|\.toml)$/)) {
                    cons.status("Creating a new file");
                    return localapi.create_file(id, path, name);
                } else {
                    cons.status("Creating a new folder");
                    return localapi.create_directory(id, path, name);
                }
            }
        )
        .then((_) => localapi.project_tree(id))
        .then(
            list => {
                explorer_set(list);
                cons.status("Done");
            },
            error => {
                cons.status("Error");
                console.error(error);
            }
        );

    return false;
}

function on_click_explorer_dir_rename(e) {
    e.stopPropagation();
    let main_block = this.parentByClass("dir"),
        id = window.open_project.id,
        path = main_block.attr("path").split("/").slice(1).slice(0, -1).join("/"),
        old_name = main_block.attr("path").split("/").slice(-1).join("/");

    cons.status("Please enter a new name for the folder");
    explorer_entering_name(main_block)
        .then(
            new_name => {
                cons.status("Changing the folder name");
                return localapi.rename_directory(id, "./" + path, old_name, new_name);
            }
        )
        .then(_ => localapi.project_tree(id))
        .then(
            list => {
                explorer_set(list);
                cons.status("Done");
            },
            error => {
                cons.status("Error: " + error);
                console.error(error);
            }
        );

    return false;
}

function on_click_explorer_dir_remove(e) {
    e.stopPropagation();
    let main_block = this.parentByClass("dir"),
        project_id = window.open_project.id,
        path = main_block.attr("path").split("/").slice(1).join("/");

    if (confirm("Are you sure you want to delete the folder?")) {
        cons.status("Deleting a directory");
        localapi.remove_directory(project_id, path)
            .then(_ => localapi.project_tree(project_id))
            .then(
                list => {
                    explorer_set(list);
                    cons.status("Done");
                },
                error => {
                    cons.status("Error: " + error);
                    console.error(error);
                }
            );
    }

    return false;
}

function on_click_explorer_file_rename(e) {
    e.stopPropagation();
    let main_block = this.parentByClass("file"),
        project_id = window.open_project.id,
        old_file_id = main_block.attr("data-id"),
        new_name_file = "";

    cons.status("Please enter a new name for the file");
    explorer_entering_name(main_block)
        .then(
            new_name => {
                new_name_file = new_name;
                cons.status("Changing the file name");
                return localapi.rename_file(project_id, old_file_id, new_name);
            },
            error => { cons.status("Error: " + error); }
        )
        .then(
            new_file_id => {
                if (window.open_project.files[old_file_id]) {
                    window.open_project.close_file(old_file_id);
                }
                main_block
                    .attr("data-id", new_file_id)
                    .attr("data-name", new_name_file)
                    .attr("path", main_block.attr("path").split("/").slice(0, -1).join("/") + "/" + new_name_file);
                main_block.querySelector(".name span").innerHTML = new_name_file;

                cons.status("Done");
            },
            error => {
                cons.status("Error: " + error);
                console.error(error);
            }
        );

    return false;
}

function on_click_explorer_file_remove(e) {
    e.stopPropagation();
    let main_block = this.parentByClass("file"),
        project_id = window.open_project.id,
        file_id = main_block.attr("data-id");

    if (confirm("Are you sure you want to delete the file?")) {
        cons.status("Deleting a file");
        localapi.remove_file(project_id, file_id)
            .then(
                _ => {
                    if (window.open_project.files[file_id]) {
                        window.open_project.close_file(file_id);
                    }
                    main_block.remove();
                    cons.status("Done");
                },
                error => {
                    cons.status("Error: " + error);
                    console.error(error);
                }
            );
    }

    return false;
}

function explorer_entering_name(element) {
    return entering_name(
        element.querySelector(".name"),
        element.querySelector(".name span").innerHTML.trim()
    );
}

function entering_name(parent, value) {
    return new Promise((resolve, reject) => {
        parent.insertAdjacentHTML('beforeend', TEMPLATE_EXPLORER_NAME_POPUP);
        let pop = parent.querySelector(".new_name"),
            pop_input = pop.getElementsByTagName("input")[0],
            old_value = value.trim();
        pop_input.value = old_value;
        pop_input.focus();
        pop_input.select();

        pop_input.addEventListener("blur", function() { this.parentNode.remove(); })
        pop_input.addEventListener("keyup",
            function(e) {
                if (e.key === 'Enter' || e.keyCode === 13) {
                    let new_value = pop_input.value.trim();

                    pop_input.remove();
                    pop.remove();

                    if (!new_value.length) {
                        reject("name cannot be empty");
                    } else if (new_value === old_value) {
                        reject("No changes");
                    } else {
                        resolve(new_value);
                    }
                }
            })
    });
}

function select_dialect(parent) {
    return new Promise((resolve, reject) => {
        parent.insertAdjacentHTML('beforeend', TEMPLATE_EXPLORER_CHOOSE_DIALECT_POPUP);
        let pop = parent.querySelector(".new_name"),
            pop_select = pop.getElementsByTagName("select")[0];
        pop_select.focus();

        pop_select.addEventListener("blur", function() { this.parentNode.remove(); })
        pop_select.addEventListener("change",
            function(e) {
                e.stopPropagation();
                let value = pop_select.value.trim();

                pop_select.remove();
                pop.remove();

                if (!value.length) {
                    reject("Name cannot be empty");
                } else {
                    resolve(value);
                }
            })
    });
}

// ===============================================================
//  Run scripts
// ===============================================================

async function inic_run_commands() {
    container
        .querySelector("#run-container input.command:not(.i)")
        .addClass("i")
        .addEventListener("keyup", function(e) {
            e.stopPropagation();
            if (e.key === 'Enter' || e.keyCode === 13) {
                let command = this.value.trim();
                if (!command.length) { reutrn; }
                run_command(command);
                this.value = "";
            }
        });
}

async function run_command(command) {
    let history = run_command_get_history(),
        find_command = history.indexOf(command);
    if (find_command !== -1) {
        history.splice(find_command, 1);
    }
    history.unshift(command);
    history = history.slice(0, 10);

    localStorage.setItem("run.commands.history." + window.open_project.id, JSON.stringify(history));
    run_command_show_history();

    if (window.open_project.run_script) {
        window.open_project.run_script(command);
    }
}

async function run_command_show_history() {
    let block_history = container.querySelector("#run_list .cont"),
        history = run_command_get_history();

    let length = history.length;
    if (!length) {
        block_history.innerHTML = TEMPLATE_RUN_COMMAND_EMTPY;
        return;
    }

    block_history.innerHTML = "";
    for (let index = 0; index < length; index++) {
        let item = TEMPLATE_RUN_COMMAND_ITEM
            .replaceAll("{{command}}", history[index])
            .replaceAll("{{id}}", index);
        block_history.insertAdjacentHTML('beforeend', item);
    }

    block_history
        .querySelectorAll('button.run')
        .forEach(button => {
            button.addEventListener("click", function(e) {
                e.stopPropagation();
                let command = run_command_get_history()[this.parentNode.attr("data-id")];
                if (command !== undefined) {
                    run_command(command);
                }
            });
        });

    block_history
        .querySelectorAll('button.remove')
        .forEach(button => {
            button.addEventListener("click", function(e) {
                e.stopPropagation();

                cons.status("Delete an entry");
                let history = run_command_get_history(),
                    index = this.parentNode.attr("data-id") * 1;
                history.splice(index, 1);
                localStorage.setItem("run.commands.history." + window.open_project.id, JSON.stringify(history));

                cons.status("Done");
                run_command_show_history();
            });
        });
}

function run_command_get_history() {
    let history = [];
    try {
        history = localStorage.getItem("run.commands.history." + window.open_project.id);
        history = JSON.parse(history ? history : "[]");
        if (!Array.isArray(history)) {
            history = [];
        }
    } catch {}
    return history;

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
    document.addEventListener("keydown", function(e) {
        if (e.ctrlKey && e.code === "F6") {
            e.stopPropagation();
            document
                .querySelector("#container .header button.build")
                .click()
                .focus();
            return false;
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
    document.addEventListener("keydown", function(e) {
        if (e.ctrlKey && e.code === "F7") {
            e.stopPropagation();
            document
                .querySelector("#container .header button.check")
                .click()
                .focus();
            return false;
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
    document.addEventListener("keydown", function(e) {
        if (e.ctrlKey && e.code === "F8") {
            e.stopPropagation();
            document
                .querySelector("#container .header button.test")
                .click()
                .focus();
            return false;
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
    document.addEventListener("keydown", function(e) {
        if (e.ctrlKey && e.code === "F9") {
            e.stopPropagation();
            document
                .querySelector("#container .header button.clean")
                .click()
                .focus();
            return false;
        }
    });
}


setTimeout(() => {
    // document.querySelector("#projects-container button.add_project").click();
    // document.querySelector("#projects-container input").value = "p_" + Math.floor(Math.random() * 1000000);
    document.querySelector('.project[data-id="46_1632992999791"] button').click();

    setTimeout(() => {
        // localapi.remove_directory(window.open_project.id, "a1/m_2renamed/1/2")
        // localapi.remove_file(window.open_project.id, "90_1633014766535");
        // document.querySelectorAll("#explorer button.remove")[0].click();
        // localapi.rename_file(window.open_project.id, "3_1633067129351", "test_2.move")
        //     .then(result => {
        //         console.log(result);
        //     }, error => {
        //         console.log(error);
        //     });
        // localapi.rename_directory(window.open_project.id, "./a1", "demo", "demo_renamed");
    }, 50);
}, 50);