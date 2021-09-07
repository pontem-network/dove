import init, * as wasm from '../pkg/client.js';

let project_id;

export default async function run() {
    await init();
    await load_projects();
    // TODO Remove id. This is bug demo.
    console.log("3000490687877993158 is expected. Actual is " + wasm.there_be_a_bug().id);
}

export async function load_projects() {
    let list = await wasm.project_list();
    let projects_parent = document.getElementById("projects");
    projects_parent.innerHTML = "";
    list.projects.forEach(function (item) {
        projects_parent.innerHTML += `
        <div class="project noselect" onclick = "window.index.select_project('${item.id}')">
            <div style="width: 100%; height: 2px"></div>
            <h2 class="project-title title">${item.name}</h2>
        </div>
        `;
    });
}

export async function select_project(id) {
    project_id = id;
    let projects_parent = document.getElementById("explorer");
    projects_parent.innerHTML = "";
    let info = await wasm.project_info(id);
    let config = {line_height: 18};
    await wasm.open_file(id, "a6ee3f32e51a5a81", "editor-container", config);
    console.log("{}", info);
}