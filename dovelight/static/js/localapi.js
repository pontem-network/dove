import init, { build, make_abi, module_abi } from '../../pkg/dovelight.js';

async function dbconnect() {
    return new Promise((resolve, reject) => {
        let request = indexedDB.open("dovecote", 9);
        request.onerror = function(event) {
            reject("failed to create a database. Code: " + event.target.errorCode);
        };
        request.onsuccess = function(event) {
            resolve(event.target.result)
        };
        request.onupgradeneeded = function(event) {
            let db = event.target.result;
            // projects
            if (db.objectStoreNames.contains('projects')) { db.deleteObjectStore("projects"); }
            let tbprojects = db.createObjectStore("projects", { keyPath: "id" });
            tbprojects.createIndex("name", "name", { unique: false });
            tbprojects.createIndex("dialect", "dialect", { unique: false });
            // tree
            if (db.objectStoreNames.contains('tree')) { db.deleteObjectStore("tree"); }
            let tbtree = db.createObjectStore("tree", { keyPath: "project_id" });
            tbtree.createIndex("data", "data", { unique: false });
            // files
            if (db.objectStoreNames.contains('files')) { db.deleteObjectStore("files"); }
            let tbfiles = db.createObjectStore("files", { keyPath: "file_id" });
            tbfiles.createIndex("project_id", "project_id", { unique: false });
            tbfiles.createIndex("content", "content", { unique: false });
        }
    });
}

export async function project_list() {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                db.transaction("projects").objectStore("projects").getAll().onsuccess = function(event) {
                    event.target.result.sort((a, b) => {
                        return (a.name > b.name) ? +1 : (a.name < b.name ? -1 : 0);
                    })
                    resolve(event.target.result);
                };
                db.transaction("projects").objectStore("projects").getAll().onerror = function(event) {
                    reject(event);
                };
            })
        });
}

async function project_get(project_id) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let request = db.transaction("projects")
                    .objectStore("projects")
                    .get(project_id);
                request.onsuccess = function(event) {
                    let row = event.target.result;
                    resolve(row ? row : null)
                };
                request.onerror = function(event) {
                    reject(event);
                }
            })
        })
}

export async function create_project(project_name, dialect) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let project_id = Math.floor(Math.random() * 100) + "_" + new Date().getTime(),
                    request = db.transaction("projects", 'readwrite')
                    .objectStore("projects")
                    .add({
                        id: project_id,
                        name: project_name,
                        dialect: dialect
                    });
                request.onsuccess = function(_) {
                    resolve(db_project_tree_save(project_id, {
                        Dir: [
                            project_name, [
                                { Dir: ['modules', []] },
                                { Dir: ['scripts', []] },
                                { Dir: ['tests', []] },
                            ]
                        ]
                    }));
                };
                request.onerror = function(event) {
                    reject(event);
                }
            });
        });
}

export async function remove_project(project_id) {
    dbconnect()
        .then(db => {
            let files = db
                .transaction("files", "readwrite")
                .objectStore("files");
            files.index("project_id")
                .getAll(project_id)
                .onsuccess = e => {
                    for (const result of e.target.result) {
                        files.delete(result.file_id);
                    }
                };

            return new Promise((resolve, reject) => {
                let request = db.transaction("projects", "readwrite")
                    .objectStore("projects")
                    .delete(project_id);
                request.onsuccess = function(_) {
                    resolve(true);
                };
                request.onerror = function(event) {
                    reject(event);
                }
            });
        });
}

export async function project_tree(project_id) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let request = db.transaction("tree")
                    .objectStore("tree")
                    .get(project_id);
                request.onsuccess = function(event) {
                    project_get(project_id)
                        .then(info => {
                            resolve({
                                Dir: [
                                    (info ? info.name : "none"),
                                    (event.target.result && event.target.result.data) ? event.target.result.data : []
                                ]
                            });
                        });
                };
                request.onerror = function(event) {
                    reject(event);
                }
            })
        })
}

async function db_project_tree_get_file_name(project_id, file_id) {
    function find(tree) {
        if (tree.Dir) {
            for (let index in tree.Dir[1]) {
                let result = find(tree.Dir[1][index]);
                if (result !== null) {
                    return result;
                }
            }
        } else if (tree.File) {
            if (tree.File[0] && tree.File[0] == file_id) {
                return tree.File[1];
            }
        }
        return null;
    }
    return project_tree(project_id)
        .then(tree => {
            return new Promise((resolve, reject) => {
                resolve(find(tree));
            });
        });
}

async function db_project_tree_save(project_id, data) {
    return dbconnect().then(db => {
        return new Promise((resolve, reject) => {
            let row = {
                project_id: project_id,
                data: data.Dir[1],
            };
            let request = db
                .transaction("tree", 'readwrite')
                .objectStore("tree")
                .put(row);
            request.onsuccess = function(e) {
                resolve(true)
            };
            request.onerror = function(e) {
                reject(e)
            };
        })
    });
}

async function db_project_tree_add(project_id, path, object) {
    return project_tree(project_id)
        .then(list => {
            let cursor = list,
                path_parts = path.split("/");

            for (let key in path_parts) {
                if (!cursor.Dir || !cursor.Dir[1]) { break; }
                let finded = cursor.Dir[1].find((element) => {
                    return element.Dir && element.Dir[0] === path_parts[key];
                });
                if (!finded) { break; }
                cursor = finded;
            }
            if (!cursor.Dir[1] || !Array.isArray(cursor.Dir[1])) {
                cursor.Dir[1] = [];
            }

            let duplicate = cursor.Dir[1].find(el => {
                if (object.File) {
                    return el.File && el.File[1] === object.File[1];
                } else {
                    return el.Dir && el.Dir[0] === object.Dir[0];
                }
            });
            if (duplicate) {
                return new Promise((resolve, _) => { resolve(true); });
            }

            cursor.Dir[1].push(object);
            return db_project_tree_save(project_id, list);
        });
}

export async function create_file(project_id, path, name) {
    let file_id = Math.floor(Math.random() * 100) + "_" + new Date().getTime();
    return db_project_tree_add(project_id, path, {
        File: [
            file_id,
            name
        ]
    }).then(save_file({
        file_id: file_id,
        project_id: project_id,
        content: ""
    }));
}

export async function create_directory(project_id, path, name) {
    return db_project_tree_add(project_id, path, {
        Dir: [
            name, []
        ]
    });
}

export async function rename_file(project_id, file_id, new_name) {
    return project_tree(project_id)
        .then(list => {
            function find(object, file_id) {
                if (object.Dir && object.Dir[1] && Array.isArray(object.Dir[1])) {
                    for (let index in object.Dir[1]) {
                        let result = find(object.Dir[1][index], file_id);
                        if (result) {
                            return Array.isArray(result) ? result : [object, result];
                        }
                    }
                } else if (object.File) {
                    if (object.File[0] == file_id) {
                        return object;
                    }
                }
                return null;
            };
            return new Promise((resolve, reject) => {
                let finded = find(list, file_id);
                if (!finded) {
                    reject("Not found");
                }
                let [parent_dir, file] = finded;

                if (parent_dir.Dir[1].find(el => { return el.File && el.File[1] === new_name; })) {
                    reject("A file with that name already exists");
                }
                file.File[1] = new_name;
                resolve(db_project_tree_save(project_id, list));
            });
        });
}

export async function rename_directory(project_id, path, old_name, new_name) {
    return project_tree(project_id)
        .then(list => {
            function find(object) {
                if (object.Dir) {
                    if (object.Dir[0] === old_name) {
                        return object;
                    }
                    if (object.Dir[1] && Array.isArray(object.Dir[1])) {
                        for (let index in object.Dir[1]) {
                            let result = find(object.Dir[1][index]);
                            if (result) {
                                return Array.isArray(result) ? result : [object, result];
                            }
                        }
                    }
                }
                return null;
            };
            return new Promise((resolve, reject) => {
                let finded = find(list);
                if (!finded) {
                    reject("Not found");
                }
                let parent_dir, need_dir;
                if (parent.Dir) {
                    parent_dir = null;
                    need_dir = find;
                } else {
                    [parent_dir, need_dir] = finded;
                };

                if (parent_dir && parent_dir.Dir[1].find(el => { return el.Dir && el.Dir[0] === new_name; })) {
                    reject("A file with that name already exists");
                }
                need_dir.Dir[0] = new_name;
                resolve(db_project_tree_save(project_id, list));
            });
        });
}

export async function remove_file(project_id, file_id) {
    return project_tree(project_id)
        .then(list => {
            function remove(object, file_id) {
                if (object.Dir && object.Dir[1] && Array.isArray(object.Dir[1])) {
                    for (let index in object.Dir[1]) {
                        if (object.Dir[1][index].File) {
                            if (object.Dir[1][index].File[0] === file_id) {
                                db_remove_files(object.Dir[1][index]);
                                object.Dir[1].splice(index, 1);
                                return true;
                            }
                            continue;
                        }
                        let result = remove(object.Dir[1][index], file_id);
                        if (result) {
                            return true;
                        }
                    }
                }
                return false;
            };
            remove(list, file_id);
            return db_project_tree_save(project_id, list);
        });
}

export async function remove_directory(project_id, path) {
    return project_tree(project_id)
        .then(list => {
            function remove(object, path_array) {
                if (!path_array || !Array.isArray(path_array)) { return false; }
                let name = path_array.splice(0, 1);
                if (object.Dir && object.Dir[1] && Array.isArray(object.Dir[1])) {
                    for (let index in object.Dir[1]) {
                        if (object.Dir[1][index].Dir && object.Dir[1][index].Dir[0] == name) {
                            if (!path_array.length) {
                                db_remove_files(object.Dir[1][index]);
                                object.Dir[1].splice(index, 1);
                                return true;
                            }
                            return remove(object.Dir[1][index], path_array)
                        }
                    }
                }
                return false;
            };
            remove(list, path.split('/'));
            return db_project_tree_save(project_id, list);
        });
}

export async function get_file(project_id, file_id) {
    return dbconnect()
        .then(db => {
            return new Promise((resolve, reject) => {
                    let request = db.transaction("files")
                        .objectStore("files")
                        .get(file_id);
                    request.onsuccess = function(event) {
                        return event.target.result ? resolve(event.target.result) : reject("not found");
                    };
                    request.onerror = function(event) {
                        reject(event);
                    }
                })
                .then(row => {
                    return db_project_tree_get_file_name(project_id, file_id)
                        .then(name => {
                            return new Promise((resolve, reject) => {
                                row.name = name;
                                row.tp = name.replace(/^.*\.([^\.]+)$/, "$1");
                                resolve(row);
                            });
                        })
                });
        });
}

export async function save_file(row) {
    return dbconnect().then(db => {
        return new Promise((resolve, reject) => {
            let request = db
                .transaction("files", 'readwrite')
                .objectStore("files")
                .put(row);
            request.onsuccess = function(e) {
                resolve(true)
            };
            request.onerror = function(e) {
                reject(e)
            };
        })
    });
}

async function db_remove_files(object) {
    function remove(tb, tree) {
        if (tree.Dir) {
            for (let index in tree.Dir[1]) {
                remove(tb, tree.Dir[1][index])
            }
        } else if (tree.File) {
            tb.delete(tree.File[0]);
        }
    }
    dbconnect()
        .then(db => {
            let tb = db.transaction("files", "readwrite")
                .objectStore("files");
            remove(tb, object);
        });
}

async function project_modules_and_scripts(project_id) {
    async function files(cursor, path) {
        if (cursor.Dir) {
            if (!cursor.Dir[1] || !Array.isArray(cursor.Dir[1])) {
                return [];
            }
            path = path + "/" + cursor.Dir[0];
            let answer = [];
            for (let index in cursor.Dir[1]) {
                answer = answer.concat(await files(cursor.Dir[1][index], path));
            }
            return answer;
        } else if (cursor.File) {
            let file = await get_file(project_id, cursor.File[0]);
            file.path = path + "/" + file.name;
            return [file];
        }
        return [];
    }

    return project_tree(project_id)
        .then(async tree => {
            let scripts = [],
                modules = [];
            if (tree.Dir && tree.Dir[1] && Array.isArray(tree.Dir[1])) {
                let module_cursor = null,
                    script_cursor = null;

                for (let index in tree.Dir[1]) {
                    if (tree.Dir[1][index].Dir && tree.Dir[1][index].Dir[0]) {
                        if (tree.Dir[1][index].Dir[0] == 'modules') {
                            module_cursor = tree.Dir[1][index];
                        } else if (tree.Dir[1][index].Dir[0] == 'scripts') {
                            script_cursor = tree.Dir[1][index];
                        }
                    }
                }
                if (module_cursor) {
                    modules = files(module_cursor, ".");
                }
                if (script_cursor) {
                    scripts = files(script_cursor, ".");
                }
                modules = (await modules).reduce((a, v) => ({...a, [v.path]: v }), {});
                scripts = (await scripts).reduce((a, v) => ({...a, [v.path]: v }), {});
            }
            return [modules, scripts]
        });
}

export async function project_build(project_id) {
    return project_modules_and_scripts(project_id)
        .then(files => {
            let [modules, scripts] = files,
            all = {...modules, ...scripts },
                sources = new Map(),
                source_map = { source_map: sources };
            Object.keys(all).forEach(key => {
                sources[key] = all[key].content;
            });

            return build("http://localhost:9933/", source_map, "pont", "0x1");
        });
}